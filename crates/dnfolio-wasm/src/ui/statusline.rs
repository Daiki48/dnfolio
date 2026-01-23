//! ステータスライン管理
//!
//! Neovim風ステータスラインの更新

use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys::HtmlElement;

use crate::dom::{query_selector_optional, window};
use crate::error::Result;
use crate::vim::mode::EditorMode;

/// ステータスライン管理
pub struct StatusLine;

impl StatusLine {
    /// モード表示を更新
    pub fn update_mode(mode: EditorMode) -> Result<()> {
        if let Some(el) = query_selector_optional::<HtmlElement>(".statusline-mode")? {
            el.set_text_content(Some(mode.display_name()));
            el.set_class_name(&format!("statusline-mode {}", mode.css_class()));
        }
        Ok(())
    }

    /// スクロール位置を更新
    pub fn update_scroll_position() -> Result<()> {
        let scroll_position = match query_selector_optional::<HtmlElement>("#scroll-position")? {
            Some(el) => el,
            None => return Ok(()),
        };

        let win = window()?;
        let scroll_y = win.scroll_y().unwrap_or(0.0);

        let doc = win.document().ok_or_else(|| {
            crate::error::DnfolioError::DomError("documentが見つかりません".to_string())
        })?;

        let doc_element = doc.document_element().ok_or_else(|| {
            crate::error::DnfolioError::DomError("documentElementが見つかりません".to_string())
        })?;

        let scroll_height = doc_element.scroll_height() as f64;
        let inner_height = win
            .inner_height()
            .ok()
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);

        let doc_height = scroll_height - inner_height;

        let text = if doc_height <= 0.0 {
            "All".to_string()
        } else {
            let percent = ((scroll_y / doc_height) * 100.0).round() as i32;
            if percent <= 0 {
                "Top".to_string()
            } else if percent >= 100 {
                "Bot".to_string()
            } else {
                format!("{}%", percent)
            }
        };

        scroll_position.set_text_content(Some(&text));
        Ok(())
    }

    /// ファイル情報を更新（パス表示）
    pub fn update_file_info(path: &str) -> Result<()> {
        if let Some(el) = query_selector_optional::<HtmlElement>(".statusline-filename")? {
            el.set_text_content(Some(path));
        }
        Ok(())
    }

    /// 検索カウントを更新
    pub fn update_search_count(current: usize, total: usize) -> Result<()> {
        if let Some(el) = query_selector_optional::<HtmlElement>("#search-count")? {
            let text = if total > 0 {
                format!("[{}/{}]", current + 1, total)
            } else {
                String::new()
            };
            el.set_text_content(Some(&text));
        }
        Ok(())
    }

    /// ペンディングキーを表示（Ctrl+w 等の2キーコマンド用）
    ///
    /// 一定時間後に自動的にクリアされる
    pub fn show_pending_key(key: &str) -> Result<()> {
        // コマンドライン領域に表示
        if let Some(el) = query_selector_optional::<HtmlElement>("#pending-key")? {
            el.set_text_content(Some(key));
            el.class_list().add_1("visible").ok();

            // 2秒後に自動クリア
            let el_clone = el.clone();
            let callback = Closure::wrap(Box::new(move || {
                el_clone.set_text_content(Some(""));
                el_clone.class_list().remove_1("visible").ok();
            }) as Box<dyn Fn()>);

            if let Some(window) = web_sys::window() {
                let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                    callback.as_ref().unchecked_ref(),
                    2000,
                );
            }
            callback.forget();
        }
        Ok(())
    }

    /// ペンディングキーをクリア
    pub fn clear_pending_key() -> Result<()> {
        if let Some(el) = query_selector_optional::<HtmlElement>("#pending-key")? {
            el.set_text_content(Some(""));
            el.class_list().remove_1("visible").ok();
        }
        Ok(())
    }

    /// ペインインジケーターを設定（EXPLORERにフォーカス時等）
    pub fn set_pane_indicator(pane_name: &str) -> Result<()> {
        if let Some(el) = query_selector_optional::<HtmlElement>("#pane-indicator")? {
            el.set_text_content(Some(pane_name));
            if pane_name.is_empty() {
                el.class_list().remove_1("visible").ok();
            } else {
                el.class_list().add_1("visible").ok();
            }
        }
        Ok(())
    }
}
