//! コマンドライン管理
//!
//! Vimコマンドの入力と実行

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::HtmlInputElement;

use crate::dom::query_selector_optional;
use crate::error::Result;
use crate::ui::Toast;
use crate::vim::command::CommandExecutor;

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

    /// タイプライター演出でコマンドを入力し、完了後に実行する
    pub fn typewriter_execute(cmd: &str) {
        let cmd = cmd.to_string();
        let chars: Vec<char> = cmd.chars().collect();
        let total = chars.len();

        if total == 0 {
            return;
        }

        // コマンドラインをアクティブ化（空の状態で開始）
        if let Ok(Some(input)) = Self::get_input() {
            input.remove_attribute("readonly").ok();
            input.set_value("");
            input.focus().ok();
        }

        // 1文字ずつ入力するアニメーション
        let index = Rc::new(RefCell::new(0usize));
        let interval_id = Rc::new(RefCell::new(0i32));

        let index_clone = index.clone();
        let interval_id_clone = interval_id.clone();
        let chars_clone = chars.clone();
        let cmd_clone = cmd.clone();

        let callback = Closure::wrap(Box::new(move || {
            let current = *index_clone.borrow();
            if current < total {
                // 1文字追加
                let display: String = chars_clone[..=current].iter().collect();
                if let Ok(Some(input)) =
                    query_selector_optional::<HtmlInputElement>("#commandline-input")
                {
                    input.set_value(&display);
                }
                *index_clone.borrow_mut() = current + 1;
            } else {
                // アニメーション完了: インターバルをクリアして実行
                if let Some(window) = web_sys::window() {
                    window.clear_interval_with_handle(*interval_id_clone.borrow());
                }

                // 少し間を置いてからコマンドを実行
                let cmd_exec = cmd_clone.clone();
                let exec_callback = Closure::wrap(Box::new(move || {
                    // コマンドを実行
                    if let Ok(Some(result)) = CommandExecutor::execute(&cmd_exec) {
                        let _ = match result.toast_type.as_str() {
                            "warn" => Toast::warn(&result.title, &result.message, &result.icon),
                            "error" => Toast::error(&result.title, &result.message, &result.icon),
                            "success" => {
                                Toast::success(&result.title, &result.message, &result.icon)
                            }
                            _ => Toast::info(&result.title, &result.message, &result.icon),
                        };
                    }
                    // コマンドラインを非アクティブ化
                    let _ = CommandLine::deactivate();
                }) as Box<dyn Fn()>);

                if let Some(window) = web_sys::window() {
                    let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                        exec_callback.as_ref().unchecked_ref(),
                        250,
                    );
                }
                exec_callback.forget();
            }
        }) as Box<dyn Fn()>);

        // 60msごとに1文字ずつ入力
        if let Some(window) = web_sys::window() {
            let id = window
                .set_interval_with_callback_and_timeout_and_arguments_0(
                    callback.as_ref().unchecked_ref(),
                    60,
                )
                .unwrap_or(0);
            *interval_id.borrow_mut() = id;
        }
        callback.forget();
    }
}
