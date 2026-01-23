//! 共通エラー型定義

use thiserror::Error;
use wasm_bindgen::JsValue;

/// dnfolio WASMモジュールのエラー型
#[derive(Error, Debug)]
pub enum DnfolioError {
    /// DOM要素が見つからない
    #[error("DOM要素が見つかりません: {0}")]
    ElementNotFound(String),

    /// Selection APIのエラー
    #[error("Selection取得に失敗: {0}")]
    SelectionError(String),

    /// TreeWalker操作のエラー
    #[error("TreeWalker操作に失敗: {0}")]
    TreeWalkerError(String),

    /// DOM操作のエラー
    #[error("DOM操作に失敗: {0}")]
    DomError(String),

    /// JavaScriptからのエラー
    #[error("JavaScriptエラー: {0}")]
    JsError(String),

    /// バリデーションエラー（セキュリティ関連）
    #[error("バリデーションエラー: {0}")]
    ValidationError(String),
}

impl From<JsValue> for DnfolioError {
    fn from(value: JsValue) -> Self {
        let msg = value.as_string().unwrap_or_else(|| format!("{value:?}"));
        Self::JsError(msg)
    }
}

/// Result型のエイリアス
pub type Result<T> = std::result::Result<T, DnfolioError>;
