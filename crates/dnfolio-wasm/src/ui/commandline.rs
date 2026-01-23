//! コマンドライン管理
//!
//! Vimコマンドの入力と実行

use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

use crate::dom::query_selector_optional;
use crate::error::Result;

/// コマンドライン管理
pub struct CommandLine;

impl CommandLine {
    /// コマンドラインを取得
    fn get_input() -> Result<Option<HtmlInputElement>> {
        query_selector_optional::<HtmlInputElement>("#commandline-input")
    }

    /// コマンドラインをアクティブ化（入力可能に）
    pub fn activate() -> Result<()> {
        if let Some(input) = Self::get_input()? {
            input.remove_attribute("readonly").ok();
            input.set_value(":");
            input.focus().ok();

            // カーソルを末尾に移動
            let len = input.value().len() as u32;
            input.set_selection_start(Some(len)).ok();
            input.set_selection_end(Some(len)).ok();
        }
        Ok(())
    }

    /// コマンドラインを非アクティブ化
    pub fn deactivate() -> Result<()> {
        if let Some(input) = Self::get_input()? {
            input.set_value("");
            input.set_attribute("readonly", "").ok();
            input.blur().ok();
        }

        // モーダルが開いている場合はフォーカス復元をスキップ
        // （モーダル側でフォーカス管理するため）
        if !Self::is_modal_open()? {
            Self::restore_focus_to_content()?;
        }

        Ok(())
    }

    /// モーダルが開いているか確認
    fn is_modal_open() -> Result<bool> {
        use web_sys::HtmlElement;

        if let Some(modal) = query_selector_optional::<HtmlElement>("#search-modal")? {
            if modal.class_list().contains("open") {
                return Ok(true);
            }
        }
        if let Some(modal) = query_selector_optional::<HtmlElement>("#tags-modal")? {
            if modal.class_list().contains("open") {
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// メインコンテンツにフォーカスを戻す
    fn restore_focus_to_content() -> Result<()> {
        use web_sys::HtmlElement;

        // main-contentにフォーカスを戻す
        if let Some(main) = query_selector_optional::<HtmlElement>(".main-content")? {
            // tabindexを設定してフォーカス可能に
            main.set_attribute("tabindex", "-1").ok();
            main.focus().ok();
        }

        // カーソル更新はスキップ（スクロール位置を維持するため）
        // カーソルは既に表示されているか、必要に応じて呼び出し元で更新

        Ok(())
    }

    /// コマンドラインの値を取得
    pub fn get_value() -> Result<String> {
        Ok(Self::get_input()?.map(|i| i.value()).unwrap_or_default())
    }

    /// コマンドラインの値を設定
    pub fn set_value(value: &str) -> Result<()> {
        if let Some(input) = Self::get_input()? {
            input.set_value(value);
        }
        Ok(())
    }

    /// コマンドラインがアクティブか
    pub fn is_active() -> Result<bool> {
        Ok(Self::get_input()?
            .map(|i| !i.has_attribute("readonly"))
            .unwrap_or(false))
    }

    /// コマンドラインにフォーカスがあるか
    pub fn has_focus() -> Result<bool> {
        if let Some(input) = Self::get_input()? {
            if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                if let Some(active) = doc.active_element() {
                    return Ok(active
                        .dyn_ref::<HtmlInputElement>()
                        .map(|el| el == &input)
                        .unwrap_or(false));
                }
            }
        }
        Ok(false)
    }

    /// 検索モードでコマンドラインを開く
    pub fn activate_search() -> Result<()> {
        if let Some(input) = Self::get_input()? {
            input.remove_attribute("readonly").ok();
            input.set_value("/");
            input.focus().ok();

            let len = input.value().len() as u32;
            input.set_selection_start(Some(len)).ok();
            input.set_selection_end(Some(len)).ok();
        }
        Ok(())
    }
}
