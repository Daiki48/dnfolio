//! 検索モーダル状態管理
//!
//! 検索結果の表示、j/kナビゲーション、記事遷移を管理
//! grep形式で行単位の結果を表示

use std::cell::RefCell;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

use crate::dom::{
    MAX_SEARCH_QUERY_LEN, clear_children, create_text_div, document, query_selector_optional,
    validate_url,
};
use crate::error::{DnfolioError, Result};
use crate::search::index::{SearchMatch, load_search_index, search_lines};

/// 検索モーダルの状態
pub struct SearchModalState {
    /// 検索結果（行単位）
    results: RefCell<Vec<SearchMatch>>,
    /// 選択中のインデックス
    selected_index: RefCell<usize>,
    /// 現在の検索クエリ
    current_query: RefCell<String>,
}

impl SearchModalState {
    pub fn new() -> Self {
        Self {
            results: RefCell::new(Vec::new()),
            selected_index: RefCell::new(0),
            current_query: RefCell::new(String::new()),
        }
    }

    /// 検索を実行して結果を更新
    pub async fn search(&self, query: &str) -> Result<()> {
        // クエリを保存
        *self.current_query.borrow_mut() = query.to_string();

        // 空クエリの場合は結果をクリア
        if query.trim().is_empty() {
            *self.results.borrow_mut() = Vec::new();
            *self.selected_index.borrow_mut() = 0;
            self.render_results()?;
            self.render_preview()?;
            self.update_count()?;
            return Ok(());
        }

        // インデックスをロード（初回のみ）
        load_search_index().await?;

        // 検索実行（行単位）
        let results = search_lines(query);

        *self.results.borrow_mut() = results;
        *self.selected_index.borrow_mut() = 0;

        // 結果を描画
        self.render_results()?;
        self.render_preview()?;
        self.update_count()?;

        Ok(())
    }

    /// 結果リストを描画（grep形式）
    pub fn render_results(&self) -> Result<()> {
        let list = query_selector_optional::<HtmlElement>("#grep-results-list")?
            .ok_or_else(|| DnfolioError::ElementNotFound("#grep-results-list".into()))?;

        let doc = document()?;
        // XSS対策: set_inner_html("")の代わりに安全な方法で子要素を削除
        clear_children(&list)?;

        let results = self.results.borrow();
        let selected = *self.selected_index.borrow();
        let query = self.current_query.borrow().clone();

        if results.is_empty() {
            let empty_div = doc
                .create_element("div")
                .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;
            empty_div.set_class_name("search-results-empty");
            empty_div.set_text_content(Some("No results found"));
            list.append_child(&empty_div)
                .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;
            return Ok(());
        }

        for (i, result) in results.iter().enumerate() {
            let item = doc
                .create_element("div")
                .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;
            item.set_class_name(if i == selected {
                "search-result-item selected"
            } else {
                "search-result-item"
            });

            // data属性にインデックスを設定
            item.set_attribute("data-index", &i.to_string())
                .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;

            // ファイル名:行番号（grep形式のヘッダー）
            let header_div = doc
                .create_element("div")
                .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;
            header_div.set_class_name("search-result-header");

            // タイトル部分
            let title_span = doc
                .create_element("span")
                .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;
            title_span.set_class_name("search-result-title");
            title_span.set_text_content(Some(&result.title));

            // 行番号部分
            let line_num_span = doc
                .create_element("span")
                .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;
            line_num_span.set_class_name("search-result-line-num");
            line_num_span.set_text_content(Some(&format!(":{}", result.line_num)));

            header_div
                .append_child(&title_span)
                .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;
            header_div
                .append_child(&line_num_span)
                .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;

            // 行テキスト（キーワードハイライト付き）
            let text_div = doc
                .create_element("div")
                .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;
            text_div.set_class_name("search-result-text");

            // キーワードをハイライト
            self.render_highlighted_text(&doc, &text_div, &result.line_text, &query)?;

            item.append_child(&header_div)
                .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;
            item.append_child(&text_div)
                .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;

            list.append_child(&item)
                .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;
        }

        // 選択中の要素をスクロールして表示
        self.scroll_selected_into_view()?;

        Ok(())
    }

    /// キーワードをハイライトしてテキストを描画
    fn render_highlighted_text(
        &self,
        doc: &web_sys::Document,
        container: &web_sys::Element,
        text: &str,
        query: &str,
    ) -> Result<()> {
        if query.is_empty() {
            container.set_text_content(Some(text));
            return Ok(());
        }

        let lower_text = text.to_lowercase();
        let lower_query = query.to_lowercase();
        let mut last_end = 0;

        // 大文字小文字を無視してマッチ箇所を探す
        for (start, _) in lower_text.match_indices(&lower_query) {
            // マッチ前のテキスト
            if start > last_end {
                let before_text = doc.create_text_node(&text[last_end..start]);
                container
                    .append_child(&before_text)
                    .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;
            }

            // マッチ部分（ハイライト）
            let mark = doc
                .create_element("mark")
                .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;
            mark.set_class_name("search-keyword-highlight");
            mark.set_text_content(Some(&text[start..start + query.len()]));
            container
                .append_child(&mark)
                .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;

            last_end = start + query.len();
        }

        // 残りのテキスト
        if last_end < text.len() {
            let after_text = doc.create_text_node(&text[last_end..]);
            container
                .append_child(&after_text)
                .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;
        }

        Ok(())
    }

    /// プレビューを描画（選択中の行のコンテキスト表示）
    pub fn render_preview(&self) -> Result<()> {
        let preview = query_selector_optional::<HtmlElement>("#grep-preview")?
            .ok_or_else(|| DnfolioError::ElementNotFound("#grep-preview".into()))?;

        let results = self.results.borrow();
        let selected = *self.selected_index.borrow();
        let query = self.current_query.borrow().clone();

        if let Some(result) = results.get(selected) {
            let doc = document()?;
            // XSS対策: set_inner_html("")の代わりに安全な方法で子要素を削除
            clear_children(&preview)?;

            // タイトル
            let title = doc
                .create_element("div")
                .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;
            title.set_class_name("preview-title");
            title.set_text_content(Some(&result.title));
            preview
                .append_child(&title)
                .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;

            // URL
            let url_div = doc
                .create_element("div")
                .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;
            url_div.set_class_name("preview-url");
            url_div.set_text_content(Some(&result.url));
            preview
                .append_child(&url_div)
                .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;

            // 区切り線
            let separator = doc
                .create_element("div")
                .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;
            separator.set_class_name("preview-separator");
            preview
                .append_child(&separator)
                .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;

            // 選択中の行を表示（キーワードハイライト付き）
            let line_div = doc
                .create_element("div")
                .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;
            line_div.set_class_name("preview-line");

            let line_num = doc
                .create_element("span")
                .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;
            line_num.set_class_name("preview-line-num");
            line_num.set_text_content(Some(&format!("{}:", result.line_num)));

            let line_text = doc
                .create_element("span")
                .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;
            line_text.set_class_name("preview-line-text");

            // キーワードハイライト
            self.render_highlighted_text(&doc, &line_text, &result.line_text, &query)?;

            line_div
                .append_child(&line_num)
                .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;
            line_div
                .append_child(&line_text)
                .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;
            preview
                .append_child(&line_div)
                .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;
        } else {
            // XSS対策: set_inner_htmlの代わりに安全なDOM操作を使用
            clear_children(&preview)?;
            create_text_div(&preview, "preview-empty", "No preview available")?;
        }

        Ok(())
    }

    /// 件数表示を更新
    pub fn update_count(&self) -> Result<()> {
        let count_el = query_selector_optional::<HtmlElement>("#grep-results-count")?;
        if let Some(el) = count_el {
            let count = self.results.borrow().len();
            el.set_text_content(Some(&if count == 1 {
                "1 match".to_string()
            } else {
                format!("{count} matches")
            }));
        }
        Ok(())
    }

    /// 選択中の要素を表示領域にスクロール
    fn scroll_selected_into_view(&self) -> Result<()> {
        let doc = document()?;
        let selected = *self.selected_index.borrow();
        let selector = format!(".search-result-item[data-index=\"{selected}\"]");

        if let Ok(Some(item)) = doc.query_selector(&selector) {
            if let Some(el) = item.dyn_ref::<HtmlElement>() {
                let opts = web_sys::ScrollIntoViewOptions::new();
                opts.set_behavior(web_sys::ScrollBehavior::Instant);
                opts.set_block(web_sys::ScrollLogicalPosition::Nearest);
                el.scroll_into_view_with_scroll_into_view_options(&opts);
            }
        }

        Ok(())
    }

    /// 選択を下に移動
    pub fn move_down(&self) -> Result<()> {
        let len = self.results.borrow().len();
        if len == 0 {
            return Ok(());
        }
        let current = *self.selected_index.borrow();
        *self.selected_index.borrow_mut() = (current + 1) % len;
        self.render_results()?;
        self.render_preview()?;
        Ok(())
    }

    /// 選択を上に移動
    pub fn move_up(&self) -> Result<()> {
        let len = self.results.borrow().len();
        if len == 0 {
            return Ok(());
        }
        let current = *self.selected_index.borrow();
        *self.selected_index.borrow_mut() = if current == 0 { len - 1 } else { current - 1 };
        self.render_results()?;
        self.render_preview()?;
        Ok(())
    }

    /// 選択中の記事を開く
    pub fn open_selected(&self) -> Result<()> {
        let results = self.results.borrow();
        let selected = *self.selected_index.borrow();
        let query = self.current_query.borrow().clone();

        if let Some(result) = results.get(selected) {
            // セキュリティ: URLをバリデーション
            validate_url(&result.url)?;

            // セキュリティ: クエリの長さを制限
            let safe_query = if query.len() > MAX_SEARCH_QUERY_LEN {
                &query[..MAX_SEARCH_QUERY_LEN]
            } else {
                &query
            };

            let encoded_query = js_sys::encode_uri_component(safe_query);
            let url = format!(
                "{}?highlight={}&lineNum={}",
                result.url, encoded_query, result.line_num
            );

            // ローディングオーバーレイを表示
            crate::dom::show_loading()?;

            if let Some(window) = web_sys::window() {
                let _ = window.location().set_href(&url);
            }
        }
        Ok(())
    }

    /// 結果をクリア
    pub fn clear(&self) -> Result<()> {
        *self.results.borrow_mut() = Vec::new();
        *self.selected_index.borrow_mut() = 0;
        *self.current_query.borrow_mut() = String::new();
        Ok(())
    }

    /// 現在の検索クエリを取得
    pub fn current_query(&self) -> String {
        self.current_query.borrow().clone()
    }
}

impl Default for SearchModalState {
    fn default() -> Self {
        Self::new()
    }
}

// グローバル状態
thread_local! {
    pub static SEARCH_MODAL_STATE: RefCell<SearchModalState> =
        RefCell::new(SearchModalState::new());
}

// =============================================================================
// 公開ヘルパー関数（events.rsから呼び出し用）
// thread_local!のライフタイム問題を回避するため、非同期処理は関数内で完結させる
// =============================================================================

/// 検索を実行（グローバル状態を使用）
pub async fn perform_search(query: String) -> Result<()> {
    // クエリを保存
    SEARCH_MODAL_STATE.with(|state| {
        *state.borrow().current_query.borrow_mut() = query.clone();
    });

    // 空クエリの場合は結果をクリア
    if query.trim().is_empty() {
        SEARCH_MODAL_STATE.with(|state| {
            let s = state.borrow();
            *s.results.borrow_mut() = Vec::new();
            *s.selected_index.borrow_mut() = 0;
        });
        render_search_results()?;
        render_search_preview()?;
        update_search_count()?;
        return Ok(());
    }

    // インデックスをロード（初回のみ）
    load_search_index().await?;

    // 検索実行（行単位）
    let results = search_lines(&query);

    // 結果を保存
    SEARCH_MODAL_STATE.with(|state| {
        let s = state.borrow();
        *s.results.borrow_mut() = results;
        *s.selected_index.borrow_mut() = 0;
    });

    // 結果を描画
    render_search_results()?;
    render_search_preview()?;
    update_search_count()?;

    Ok(())
}

/// 選択を下に移動
pub fn modal_move_down() -> Result<()> {
    SEARCH_MODAL_STATE.with(|state| state.borrow().move_down())
}

/// 選択を上に移動
pub fn modal_move_up() -> Result<()> {
    SEARCH_MODAL_STATE.with(|state| state.borrow().move_up())
}

/// 選択中の記事を開く
pub fn modal_open_selected() -> Result<()> {
    SEARCH_MODAL_STATE.with(|state| state.borrow().open_selected())
}

/// モーダル状態をクリア
pub fn modal_clear() -> Result<()> {
    SEARCH_MODAL_STATE.with(|state| state.borrow().clear())
}

/// 結果リストを描画（内部ヘルパー）
fn render_search_results() -> Result<()> {
    SEARCH_MODAL_STATE.with(|state| state.borrow().render_results())
}

/// プレビューを描画（内部ヘルパー）
fn render_search_preview() -> Result<()> {
    SEARCH_MODAL_STATE.with(|state| state.borrow().render_preview())
}

/// 件数表示を更新（内部ヘルパー）
fn update_search_count() -> Result<()> {
    SEARCH_MODAL_STATE.with(|state| state.borrow().update_count())
}
