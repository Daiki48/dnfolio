//! トースト通知
//!
//! noice.nvim風のトースト通知を表示

use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

use crate::dom::{document, query_selector};
use crate::error::{DnfolioError, Result};

/// トーストタイプ
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ToastType {
    Info,
    Warn,
    Error,
    Success,
}

impl ToastType {
    /// CSSクラス名を取得
    fn css_class(&self) -> &'static str {
        match self {
            Self::Info => "toast-info",
            Self::Warn => "toast-warn",
            Self::Error => "toast-error",
            Self::Success => "toast-success",
        }
    }
}

/// トースト通知
pub struct Toast;

impl Toast {
    /// トーストを表示（5秒後に自動削除）
    pub fn show(title: &str, message: &str, icon: &str, toast_type: ToastType) -> Result<()> {
        let container = query_selector::<HtmlElement>("#toast-container")?;
        let doc = document()?;

        // トースト要素を作成
        let toast = doc
            .create_element("div")
            .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;
        toast.set_class_name(&format!("toast {}", toast_type.css_class()));

        // XSS対策: textContentを使用してDOM構築
        let icon_span = doc
            .create_element("span")
            .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;
        icon_span.set_class_name("toast-icon");
        icon_span.set_text_content(Some(icon));

        let content_div = doc
            .create_element("div")
            .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;
        content_div.set_class_name("toast-content");

        let title_div = doc
            .create_element("div")
            .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;
        title_div.set_class_name("toast-title");
        title_div.set_text_content(Some(title));

        let message_div = doc
            .create_element("div")
            .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;
        message_div.set_class_name("toast-message");
        message_div.set_text_content(Some(message));

        content_div
            .append_child(&title_div)
            .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;
        content_div
            .append_child(&message_div)
            .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;

        // 閉じるボタン
        let close_btn = doc
            .create_element("button")
            .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;
        close_btn.set_class_name("toast-close");
        close_btn.set_text_content(Some("×"));

        toast
            .append_child(&icon_span)
            .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;
        toast
            .append_child(&content_div)
            .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;
        toast
            .append_child(&close_btn)
            .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;

        container
            .append_child(&toast)
            .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;

        // 閉じるボタンのイベントリスナー
        let toast_clone = toast.clone();
        let close_handler = Closure::wrap(Box::new(move || {
            let _ = toast_clone.class_list().add_1("hiding");
            let toast_inner = toast_clone.clone();
            let remove_handler = Closure::wrap(Box::new(move || {
                toast_inner.remove();
            }) as Box<dyn Fn()>);

            let window = web_sys::window().unwrap();
            let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                remove_handler.as_ref().unchecked_ref(),
                300,
            );
            remove_handler.forget();
        }) as Box<dyn Fn()>);

        close_btn
            .add_event_listener_with_callback("click", close_handler.as_ref().unchecked_ref())
            .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;
        close_handler.forget();

        // 5秒後に自動削除
        let toast_clone2 = toast.clone();
        let auto_hide = Closure::wrap(Box::new(move || {
            let _ = toast_clone2.class_list().add_1("hiding");
            let toast_inner = toast_clone2.clone();
            let remove_handler = Closure::wrap(Box::new(move || {
                toast_inner.remove();
            }) as Box<dyn Fn()>);

            let window = web_sys::window().unwrap();
            let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                remove_handler.as_ref().unchecked_ref(),
                300,
            );
            remove_handler.forget();
        }) as Box<dyn Fn()>);

        let window = web_sys::window()
            .ok_or_else(|| DnfolioError::DomError("windowが見つかりません".to_string()))?;
        window
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                auto_hide.as_ref().unchecked_ref(),
                5000,
            )
            .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;
        auto_hide.forget();

        Ok(())
    }

    /// 情報トーストを表示
    pub fn info(title: &str, message: &str, icon: &str) -> Result<()> {
        Self::show(title, message, icon, ToastType::Info)
    }

    /// 警告トーストを表示
    pub fn warn(title: &str, message: &str, icon: &str) -> Result<()> {
        Self::show(title, message, icon, ToastType::Warn)
    }

    /// エラートーストを表示
    pub fn error(title: &str, message: &str, icon: &str) -> Result<()> {
        Self::show(title, message, icon, ToastType::Error)
    }

    /// 成功トーストを表示
    pub fn success(title: &str, message: &str, icon: &str) -> Result<()> {
        Self::show(title, message, icon, ToastType::Success)
    }
}
