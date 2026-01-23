//! ハイライト管理
//!
//! 検索結果のハイライト表示と管理

use std::cell::RefCell;

use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement};

use crate::dom::{TextNodeWalker, document, query_selector};
use crate::error::{DnfolioError, Result};

/// ハイライト管理
pub struct HighlightManager {
    /// ハイライト要素のリスト
    highlights: RefCell<Vec<Element>>,
    /// 現在のインデックス
    current_index: RefCell<usize>,
}

impl HighlightManager {
    /// 新しいHighlightManagerを作成
    pub fn new() -> Self {
        Self {
            highlights: RefCell::new(Vec::new()),
            current_index: RefCell::new(0),
        }
    }

    /// キーワードをハイライト
    ///
    /// # Arguments
    /// * `query` - 検索キーワード
    /// * `target_line_num` - ターゲット行番号（オプション、1始まり）
    ///
    /// # Returns
    /// ハイライトの総数
    pub fn apply(&self, query: &str, target_line_num: Option<usize>) -> Result<usize> {
        // 既存のハイライトを削除
        self.remove_all()?;

        if query.is_empty() {
            return Ok(0);
        }

        let main_content = query_selector::<HtmlElement>(".main-content")?;
        let doc = document()?;
        let walker = TextNodeWalker::new(&main_content)?;
        let lower_query = query.to_lowercase();

        // ブロック要素を取得（行番号対応）
        let block_selector = ":scope > h1, :scope > h2, :scope > h3, :scope > h4, \
                              :scope > p, :scope > ul:not(.badge-list), :scope > ol, \
                              :scope > blockquote, :scope > pre, :scope > table, \
                              :scope > hr, :scope > div.code-block-wrapper";

        let block_elements = main_content
            .query_selector_all(block_selector)
            .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;

        // ターゲット要素を特定
        let target_element: Option<Element> = target_line_num.and_then(|num| {
            if num > 0 && num <= block_elements.length() as usize {
                block_elements
                    .get((num - 1) as u32)
                    .and_then(|n| n.dyn_into().ok())
            } else {
                None
            }
        });

        // テキストノードを収集
        let nodes = walker.collect_all()?;

        let mut highlights = Vec::new();
        let mut target_highlight_index: Option<usize> = None;

        for node in nodes {
            let text = match node.text_content() {
                Some(t) => t,
                None => continue,
            };

            if !text.to_lowercase().contains(&lower_query) {
                continue;
            }

            // このノードがターゲット要素内にあるかチェック
            let is_in_target = target_element
                .as_ref()
                .map(|el| el.contains(Some(&node)))
                .unwrap_or(false);

            // テキストを分割してハイライト
            let parts = self.split_by_query(&text, query);
            if parts.len() <= 1 {
                continue;
            }

            let fragment = doc.create_document_fragment();
            for part in parts {
                if part.to_lowercase() == lower_query {
                    let mark = doc
                        .create_element("mark")
                        .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;
                    mark.set_class_name("search-highlight");
                    mark.set_text_content(Some(&part));
                    fragment
                        .append_child(&mark)
                        .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;

                    // ターゲット要素内の最初のハイライトを記録
                    if is_in_target && target_highlight_index.is_none() {
                        target_highlight_index = Some(highlights.len());
                    }
                    highlights.push(mark);
                } else {
                    let text_node = doc.create_text_node(&part);
                    fragment
                        .append_child(&text_node)
                        .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;
                }
            }

            if let Some(parent) = node.parent_node() {
                parent
                    .replace_child(&fragment, &node)
                    .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;
            }
        }

        let count = highlights.len();
        *self.highlights.borrow_mut() = highlights;

        // 初期位置を設定
        let initial_index = target_highlight_index.unwrap_or(0);
        if count > 0 {
            self.set_current(initial_index)?;
        }

        Ok(count)
    }

    /// 全ハイライトを削除
    pub fn remove_all(&self) -> Result<()> {
        let doc = document()?;
        let highlights = doc
            .query_selector_all(".search-highlight")
            .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;

        for i in 0..highlights.length() {
            if let Some(mark) = highlights.get(i) {
                if let Some(parent) = mark.parent_node() {
                    let text = doc.create_text_node(&mark.text_content().unwrap_or_default());
                    parent
                        .replace_child(&text, &mark)
                        .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;
                    // 隣接テキストノードを正規化
                    parent.normalize();
                }
            }
        }

        self.highlights.borrow_mut().clear();
        *self.current_index.borrow_mut() = 0;
        self.update_count_display()?;

        Ok(())
    }

    /// 次のハイライトへ移動
    pub fn next(&self) -> Result<()> {
        let len = self.highlights.borrow().len();
        if len == 0 {
            return Ok(());
        }
        let current = *self.current_index.borrow();
        let new_index = (current + 1) % len;
        self.set_current(new_index)
    }

    /// 前のハイライトへ移動
    pub fn prev(&self) -> Result<()> {
        let len = self.highlights.borrow().len();
        if len == 0 {
            return Ok(());
        }
        let current = *self.current_index.borrow();
        let new_index = if current == 0 { len - 1 } else { current - 1 };
        self.set_current(new_index)
    }

    /// 現在のインデックスを設定
    fn set_current(&self, index: usize) -> Result<()> {
        let highlights = self.highlights.borrow();
        let mut target_element: Option<Element> = None;

        for (i, el) in highlights.iter().enumerate() {
            let class_list = el.class_list();
            if i == index {
                let _ = class_list.add_1("current");
                target_element = Some(el.clone());
            } else {
                let _ = class_list.remove_1("current");
            }
        }
        drop(highlights);

        *self.current_index.borrow_mut() = index;
        self.update_count_display()?;

        // スクロールとカーソル移動を遅延実行（DOM完全構築後）
        if let Some(el) = target_element {
            let callback = wasm_bindgen::closure::Closure::once(Box::new(move || {
                // 画面中央にスクロール（即座に実行）
                if let Some(html_el) = el.dyn_ref::<HtmlElement>() {
                    let options = web_sys::ScrollIntoViewOptions::new();
                    options.set_behavior(web_sys::ScrollBehavior::Instant);
                    options.set_block(web_sys::ScrollLogicalPosition::Center);
                    html_el.scroll_into_view_with_scroll_into_view_options(&options);
                }

                // ハイライト要素内のテキストノードにカーソルを移動
                if let Some(text_node) = el.first_child() {
                    if let Ok(sel) = crate::dom::SelectionHelper::get() {
                        let _ = sel.collapse(&text_node, 0);
                        let _ = crate::vim::cursor::update_block_cursor();
                    }
                }
            })
                as Box<dyn FnOnce()>);

            if let Some(window) = web_sys::window() {
                // 200ms待ってからスクロール・カーソル移動
                let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                    callback.as_ref().unchecked_ref(),
                    200,
                );
            }
            callback.forget();
        }

        Ok(())
    }

    /// カウント表示を更新
    fn update_count_display(&self) -> Result<()> {
        let len = self.highlights.borrow().len();
        let current = *self.current_index.borrow();

        let count_text = if len > 0 {
            format!("[{}/{}]", current + 1, len)
        } else {
            String::new()
        };

        // search-countを更新
        if let Ok(search_count) = query_selector::<HtmlElement>("#search-count") {
            search_count.set_text_content(Some(&count_text));
        }

        // highlight-nav-countを更新
        if let Ok(nav_count) = query_selector::<HtmlElement>("#highlight-nav-count") {
            nav_count.set_text_content(Some(&count_text));
        }

        // highlight-navの表示切り替え
        if let Ok(highlight_nav) = query_selector::<HtmlElement>("#highlight-nav") {
            let class_list = highlight_nav.class_list();
            if len > 0 {
                let _ = class_list.add_1("visible");
            } else {
                let _ = class_list.remove_1("visible");
            }
        }

        Ok(())
    }

    /// クエリで文字列を分割（大文字小文字を無視）
    fn split_by_query(&self, text: &str, query: &str) -> Vec<String> {
        let lower = text.to_lowercase();
        let lower_query = query.to_lowercase();
        let mut parts = Vec::new();
        let mut last_end = 0;

        for (start, matched) in lower.match_indices(&lower_query) {
            if start > last_end {
                parts.push(text[last_end..start].to_string());
            }
            // 元のテキストから該当部分を取得（大文字小文字を保持）
            parts.push(text[start..start + matched.len()].to_string());
            last_end = start + matched.len();
        }

        if last_end < text.len() {
            parts.push(text[last_end..].to_string());
        }

        parts
    }

    /// ハイライト数を取得
    pub fn count(&self) -> usize {
        self.highlights.borrow().len()
    }

    /// 現在のインデックスを取得
    pub fn current_index(&self) -> usize {
        *self.current_index.borrow()
    }

    /// DOM上のハイライトと同期
    pub fn sync_with_dom(&self) -> Result<()> {
        let doc = document()?;
        let dom_highlights = doc
            .query_selector_all(".search-highlight")
            .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;

        if dom_highlights.length() == 0 {
            // DOMにハイライトがない場合はリセット
            if !self.highlights.borrow().is_empty() {
                self.highlights.borrow_mut().clear();
                *self.current_index.borrow_mut() = 0;
                self.update_count_display()?;
            }
            return Ok(());
        }

        // DOM上のハイライトと配列が不一致の場合は再構築
        let current_len = self.highlights.borrow().len();
        if current_len != dom_highlights.length() as usize {
            let mut new_highlights = Vec::new();
            for i in 0..dom_highlights.length() {
                if let Some(el) = dom_highlights.get(i) {
                    if let Ok(element) = el.dyn_into::<Element>() {
                        new_highlights.push(element);
                    }
                }
            }
            let new_len = new_highlights.len();
            *self.highlights.borrow_mut() = new_highlights;

            let current = *self.current_index.borrow();
            if current >= new_len && new_len > 0 {
                *self.current_index.borrow_mut() = new_len - 1;
            }
            self.update_count_display()?;
        }

        Ok(())
    }
}

impl Default for HighlightManager {
    fn default() -> Self {
        Self::new()
    }
}

// グローバルなHighlightManagerインスタンス
thread_local! {
    pub static HIGHLIGHT_MANAGER: RefCell<HighlightManager> = RefCell::new(HighlightManager::new());
}

/// グローバルハイライトマネージャにアクセス
pub fn with_highlight_manager<F, R>(f: F) -> R
where
    F: FnOnce(&HighlightManager) -> R,
{
    HIGHLIGHT_MANAGER.with(|manager| f(&manager.borrow()))
}

/// ハイライトを適用
pub fn apply_highlight(query: &str, target_line_num: Option<usize>) -> Result<usize> {
    HIGHLIGHT_MANAGER.with(|manager| manager.borrow().apply(query, target_line_num))
}

/// ハイライトを削除
pub fn remove_highlights() -> Result<()> {
    HIGHLIGHT_MANAGER.with(|manager| manager.borrow().remove_all())
}
