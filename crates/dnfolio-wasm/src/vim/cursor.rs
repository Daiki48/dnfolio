//! ブロックカーソル管理
//!
//! Neovim風のブロックカーソル（現在位置の文字をハイライト）

use std::cell::RefCell;

use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, Node, Range};

use crate::dom::{SelectionHelper, document};
use crate::error::{DnfolioError, Result};
use crate::vim::mode::current_mode;

/// ブロックカーソル管理
pub struct BlockCursor {
    /// 現在のカーソル要素
    current_element: RefCell<Option<Element>>,
    /// 更新ペンディングフラグ（rAF用）
    update_pending: RefCell<bool>,
}

impl BlockCursor {
    /// 新しいBlockCursorを作成
    pub fn new() -> Self {
        Self {
            current_element: RefCell::new(None),
            update_pending: RefCell::new(false),
        }
    }

    /// カーソルを更新（requestAnimationFrame経由）
    pub fn update(&self) -> Result<()> {
        // 既にペンディング中なら何もしない
        if *self.update_pending.borrow() {
            return Ok(());
        }

        *self.update_pending.borrow_mut() = true;

        // 直接更新（rAFはJS側で管理するか、gloo-renderを使用）
        // 簡略化のため直接更新
        *self.update_pending.borrow_mut() = false;
        self.update_impl()
    }

    /// カーソル更新の実装
    fn update_impl(&self) -> Result<()> {
        // 既存のカーソルを削除
        self.remove()?;

        // ビジュアルモード時はブロックカーソルを表示しない
        if current_mode().is_visual() {
            return Ok(());
        }

        let doc = document()?;
        let sel = SelectionHelper::get()?;

        if sel.range_count() == 0 {
            return Ok(());
        }

        let range = sel.get_range_at(0)?;

        // 選択範囲がある場合はスキップ
        if !sel.is_collapsed() {
            return Ok(());
        }

        // カーソル位置のノードを取得
        // 失敗した場合は最も近いテキストノードを探す
        let (node, offset) = match self.get_cursor_node(&range) {
            Ok(result) => result,
            Err(_) => {
                // フォールバック: Selectionの位置から最も近いテキストノードを探す
                if let Some(fallback) = self.find_nearest_text_node(&range)? {
                    fallback
                } else {
                    return Ok(());
                }
            }
        };

        let text = match node.text_content() {
            Some(t) if !t.is_empty() => t,
            _ => return Ok(()),
        };

        // 文字単位でオフセットを計算（UTF-8バイト数ではなく）
        let char_count = text.chars().count();

        // オフセットを調整（行末の場合は最後の文字をハイライト）
        let mut offset = if offset as usize >= char_count {
            if char_count > 0 {
                char_count - 1
            } else {
                return Ok(());
            }
        } else {
            offset as usize
        };

        // カーソル位置の文字を取得
        let chars: Vec<char> = text.chars().collect();
        let mut char_at_cursor = chars.get(offset).copied();

        // 改行文字の場合、次の有効な文字を探す
        if char_at_cursor == Some('\n') || char_at_cursor == Some('\r') {
            let mut found = false;
            // 現在位置から後ろに有効な文字を探す
            for i in (offset + 1)..chars.len() {
                let c = chars[i];
                if c != '\n' && c != '\r' {
                    offset = i;
                    char_at_cursor = Some(c);
                    found = true;
                    break;
                }
            }
            // 後ろに見つからなければ、前から探す
            if !found {
                for i in (0..offset).rev() {
                    let c = chars[i];
                    if c != '\n' && c != '\r' {
                        offset = i;
                        char_at_cursor = Some(c);
                        break;
                    }
                }
            }
        }

        // それでも有効な文字がなければリターン
        if char_at_cursor.is_none() || char_at_cursor == Some('\n') || char_at_cursor == Some('\r')
        {
            return Ok(());
        }
        let char_at_cursor = char_at_cursor.unwrap();

        // テキストノードを3つに分割してカーソル文字をspanでラップ
        let before: String = text.chars().take(offset).collect();
        let after: String = text.chars().skip(offset + 1).collect();

        let cursor_span = doc
            .create_element("span")
            .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;
        cursor_span.set_class_name("vim-cursor");
        cursor_span.set_text_content(Some(&char_at_cursor.to_string()));

        let parent = node
            .parent_node()
            .ok_or_else(|| DnfolioError::DomError("親ノードがありません".to_string()))?;

        let fragment = doc.create_document_fragment();

        if !before.is_empty() {
            let before_node = doc.create_text_node(&before);
            fragment
                .append_child(&before_node)
                .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;
        }

        fragment
            .append_child(&cursor_span)
            .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;

        if !after.is_empty() {
            let after_node = doc.create_text_node(&after);
            fragment
                .append_child(&after_node)
                .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;
        }

        parent
            .replace_child(&fragment, &node)
            .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;

        // カーソル要素を保存
        *self.current_element.borrow_mut() = Some(cursor_span.clone());

        // カーソル位置にSelectionを再設定
        self.reset_selection_after_cursor(&cursor_span)?;

        // カーソルが見えるようにスクロール
        self.scroll_to_cursor(&cursor_span)?;

        Ok(())
    }

    /// カーソル位置のテキストノードとオフセットを取得
    fn get_cursor_node(&self, range: &Range) -> Result<(Node, u32)> {
        let mut node = range
            .start_container()
            .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;
        let mut offset = range
            .start_offset()
            .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;

        // テキストノードでない場合
        if node.node_type() != Node::TEXT_NODE {
            // 子ノードがある場合、該当位置のテキストノードを探す
            let child_nodes = node.child_nodes();
            if child_nodes.length() > 0 && offset < child_nodes.length() {
                if let Some(child) = child_nodes.get(offset) {
                    if child.node_type() == Node::TEXT_NODE {
                        let text = child.text_content().unwrap_or_default();
                        if !text.is_empty() {
                            node = child;
                            offset = 0;
                        } else {
                            return Err(DnfolioError::DomError("空のテキストノード".to_string()));
                        }
                    } else {
                        return Err(DnfolioError::DomError(
                            "テキストノードではありません".to_string(),
                        ));
                    }
                } else {
                    return Err(DnfolioError::DomError(
                        "子ノードが見つかりません".to_string(),
                    ));
                }
            } else {
                return Err(DnfolioError::DomError("子ノードがありません".to_string()));
            }
        }

        Ok((node, offset))
    }

    /// 最も近いテキストノードを探す（フォールバック用）
    fn find_nearest_text_node(&self, range: &Range) -> Result<Option<(Node, u32)>> {
        use crate::dom::{TextNodeWalker, query_selector};
        use web_sys::HtmlElement;

        let container = range
            .start_container()
            .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;

        // main-content内のテキストノードを探す
        let main_content = query_selector::<HtmlElement>(".main-content")?;
        let walker = TextNodeWalker::new(&main_content)?;

        // 現在のcontainerの位置を取得
        if let Some(container_el) = container.dyn_ref::<HtmlElement>() {
            let rect = container_el.get_bounding_client_rect();
            let target_y = rect.top();

            // 最も近いテキストノードを探す
            let nodes = walker.collect_filtered()?;
            let mut best_node: Option<&Node> = None;
            let mut best_distance = f64::MAX;

            for node in &nodes {
                if let Some(parent) = node.parent_element() {
                    let node_rect = parent.get_bounding_client_rect();
                    let distance = (node_rect.top() - target_y).abs();

                    if distance < best_distance {
                        best_distance = distance;
                        best_node = Some(node);
                    }
                }
            }

            if let Some(node) = best_node {
                let text = node.text_content().unwrap_or_default();
                let char_count = text.chars().count();
                // 行末にカーソルを置く（Neovim風）
                let offset = if char_count > 0 { char_count - 1 } else { 0 };
                return Ok(Some((node.clone(), offset as u32)));
            }
        }

        // 要素ノードの子から探す
        if container.node_type() != Node::TEXT_NODE {
            let child_nodes = container.child_nodes();
            for i in 0..child_nodes.length() {
                if let Some(child) = child_nodes.get(i) {
                    if child.node_type() == Node::TEXT_NODE {
                        let text = child.text_content().unwrap_or_default();
                        if !text.trim().is_empty() {
                            let char_count = text.chars().count();
                            let offset = if char_count > 0 { char_count - 1 } else { 0 };
                            return Ok(Some((child, offset as u32)));
                        }
                    }
                }
            }
        }

        Ok(None)
    }

    /// カーソル更新後にSelectionを再設定
    /// カーソルspan内のテキストノードにSelectionを設定する
    fn reset_selection_after_cursor(&self, cursor_span: &Element) -> Result<()> {
        let sel = SelectionHelper::get()?;

        // カーソルspan内のテキストノードにSelectionを設定
        // これにより次の移動が正しい位置から開始される
        if let Some(text_node) = cursor_span.first_child() {
            sel.collapse(&text_node, 0)?;
        }

        Ok(())
    }

    /// カーソルが見えるようにスクロール（Neovim scrolloff風）
    fn scroll_to_cursor(&self, cursor_span: &Element) -> Result<()> {
        if let Some(html_el) = cursor_span.dyn_ref::<HtmlElement>() {
            // カーソル位置を取得
            let rect = html_el.get_bounding_client_rect();
            let cursor_y = rect.top();

            if let Some(window) = web_sys::window() {
                let viewport_height = window
                    .inner_height()
                    .ok()
                    .and_then(|v| v.as_f64())
                    .unwrap_or(800.0);

                // scrolloff: 画面の25%をマージンとして確保
                let margin = viewport_height * 0.25;
                let scroll_y = window.scroll_y().unwrap_or(0.0);

                // カーソルが上端マージン内にある場合
                if cursor_y < margin {
                    let new_scroll = scroll_y + cursor_y - margin;
                    window.scroll_to_with_x_and_y(0.0, new_scroll.max(0.0));
                }
                // カーソルが下端マージン内にある場合
                else if cursor_y > viewport_height - margin {
                    let new_scroll = scroll_y + cursor_y - (viewport_height - margin);
                    window.scroll_to_with_x_and_y(0.0, new_scroll);
                }
            }
        }
        Ok(())
    }

    /// 現在のカーソルを削除
    pub fn remove(&self) -> Result<()> {
        let current = self.current_element.borrow_mut().take();
        if let Some(cursor_el) = current {
            if let Some(parent) = cursor_el.parent_node() {
                let doc = document()?;
                let text = cursor_el.text_content().unwrap_or_default();
                let text_node = doc.create_text_node(&text);

                // カーソル前の直接の兄弟テキストノードの長さを計算（Selection復元用）
                // 注意：要素ノード（<code>等）は飛ばさない、直前のテキストノードのみカウント
                let mut offset_in_merged_node = 0u32;
                if let Some(prev) = cursor_el.previous_sibling() {
                    if prev.node_type() == Node::TEXT_NODE {
                        let prev_text = prev.text_content().unwrap_or_default();
                        offset_in_merged_node = prev_text.chars().count() as u32;
                    }
                }

                parent
                    .replace_child(&text_node, &cursor_el)
                    .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;

                // normalize前にSelectionを設定（text_nodeに対して）
                // これによりnormalize後も正しい位置を維持できる
                let sel = SelectionHelper::get()?;
                sel.collapse(&text_node, 0)?;

                // 隣接テキストノードを正規化
                parent.normalize();

                // normalize後、Selectionの位置を調整
                // 前に兄弟テキストノードがあった場合、それと結合されているので
                // オフセットを調整する必要がある
                if let Some(anchor) = sel.anchor_node() {
                    if anchor.node_type() == Node::TEXT_NODE && offset_in_merged_node > 0 {
                        sel.collapse(&anchor, offset_in_merged_node)?;
                    }
                }
            }
        }
        Ok(())
    }

    /// カーソル要素が存在するか
    pub fn has_cursor(&self) -> bool {
        self.current_element.borrow().is_some()
    }
}

impl Default for BlockCursor {
    fn default() -> Self {
        Self::new()
    }
}

// グローバルなBlockCursorインスタンス
thread_local! {
    static BLOCK_CURSOR: RefCell<BlockCursor> = RefCell::new(BlockCursor::new());
    static UPDATE_SCHEDULED: RefCell<bool> = RefCell::new(false);
}

/// グローバルカーソルを更新（requestAnimationFrameでバッチ処理）
pub fn update_block_cursor() -> Result<()> {
    // 既にスケジュール済みなら何もしない
    let already_scheduled = UPDATE_SCHEDULED.with(|s| {
        let was = *s.borrow();
        if !was {
            *s.borrow_mut() = true;
        }
        was
    });

    if already_scheduled {
        return Ok(());
    }

    // requestAnimationFrameで次フレームに実行
    if let Some(window) = web_sys::window() {
        let callback = wasm_bindgen::closure::Closure::once(Box::new(move || {
            UPDATE_SCHEDULED.with(|s| *s.borrow_mut() = false);
            let _ = BLOCK_CURSOR.with(|cursor| cursor.borrow().update());
        }) as Box<dyn FnOnce()>);

        let _ = window.request_animation_frame(callback.as_ref().unchecked_ref());
        callback.forget();
    }

    Ok(())
}

/// グローバルカーソルを削除（即座に実行）
pub fn remove_block_cursor() -> Result<()> {
    // 削除は即座に実行（移動前に必要なため）
    UPDATE_SCHEDULED.with(|s| *s.borrow_mut() = false);
    BLOCK_CURSOR.with(|cursor| cursor.borrow().remove())
}
