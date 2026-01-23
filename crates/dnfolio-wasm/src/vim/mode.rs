//! Vimモード管理
//!
//! EditorMode enumとグローバル状態管理

use std::cell::RefCell;

use web_sys::HtmlElement;

use crate::dom::{query_selector, query_selector_optional};
use crate::error::Result;

/// エディタモード
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum EditorMode {
    #[default]
    Normal,
    Insert,
    Visual,
    VisualLine,
}

impl EditorMode {
    /// ステータスラインに表示するテキスト
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Normal => "NORMAL",
            Self::Insert => "INSERT",
            Self::Visual => "VISUAL",
            Self::VisualLine => "V-LINE",
        }
    }

    /// CSSクラス名
    pub fn css_class(&self) -> &'static str {
        match self {
            Self::Normal => "mode-normal",
            Self::Insert => "mode-insert",
            Self::Visual => "mode-visual",
            Self::VisualLine => "mode-visual-line",
        }
    }

    /// ビジュアルモードかどうか
    pub fn is_visual(&self) -> bool {
        matches!(self, Self::Visual | Self::VisualLine)
    }
}

/// エディタ状態
#[derive(Default)]
pub struct EditorState {
    /// 現在のモード
    mode: EditorMode,
    /// 最後に押されたキー（2キーコマンド用）
    last_key: Option<char>,
    /// キーシーケンス（コマンドラインモード用）
    key_sequence: String,
}

impl EditorState {
    /// 新しいEditorStateを作成
    pub fn new() -> Self {
        Self::default()
    }

    /// 現在のモードを取得
    pub fn mode(&self) -> EditorMode {
        self.mode
    }

    /// モードを設定し、UIを更新
    pub fn set_mode(&mut self, mode: EditorMode) -> Result<()> {
        self.mode = mode;
        self.update_ui()?;
        Ok(())
    }

    /// 最後のキーを取得
    pub fn last_key(&self) -> Option<char> {
        self.last_key
    }

    /// 最後のキーを設定
    pub fn set_last_key(&mut self, key: Option<char>) {
        self.last_key = key;
    }

    /// キーシーケンスを取得
    pub fn key_sequence(&self) -> &str {
        &self.key_sequence
    }

    /// キーシーケンスをクリア
    pub fn clear_key_sequence(&mut self) {
        self.key_sequence.clear();
    }

    /// キーシーケンスに追加
    pub fn push_key(&mut self, c: char) {
        self.key_sequence.push(c);
    }

    /// UIを更新（ステータスライン、main-contentのクラス）
    fn update_ui(&self) -> Result<()> {
        // ステータスライン更新
        if let Some(el) = query_selector_optional::<HtmlElement>(".statusline-mode")? {
            el.set_text_content(Some(self.mode.display_name()));
            el.set_class_name(&format!("statusline-mode {}", self.mode.css_class()));
        }

        // main-contentのvisual-modeクラス切り替え
        if let Ok(main_content) = query_selector::<HtmlElement>(".main-content") {
            let class_list = main_content.class_list();
            let _ = class_list.remove_1("visual-mode");
            if self.mode.is_visual() {
                let _ = class_list.add_1("visual-mode");
            }
        }

        Ok(())
    }
}

// スレッドローカルなグローバル状態
thread_local! {
    static EDITOR_STATE: RefCell<EditorState> = RefCell::new(EditorState::new());
}

/// グローバル状態にアクセスして操作を実行
pub fn with_editor_state<F, R>(f: F) -> R
where
    F: FnOnce(&mut EditorState) -> R,
{
    EDITOR_STATE.with(|state| f(&mut state.borrow_mut()))
}

/// 現在のモードを取得
pub fn current_mode() -> EditorMode {
    EDITOR_STATE.with(|state| state.borrow().mode())
}

/// モードを設定
pub fn set_mode(mode: EditorMode) -> Result<()> {
    EDITOR_STATE.with(|state| state.borrow_mut().set_mode(mode))
}
