//! ウィンドウ（ペイン）フォーカス管理
//!
//! Neovimのウィンドウ分割とフォーカス移動を再現
//! - Ctrl+w h/l: 左右のペインへ移動
//! - Ctrl+w w: ペイン間をサイクル

use std::cell::RefCell;

/// フォーカス可能なペイン
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum FocusedPane {
    /// メインコンテンツ（記事本文）
    #[default]
    MainContent,
    /// 左サイドバー（EXPLORER）
    Explorer,
}

impl FocusedPane {
    /// ペイン名を取得
    pub fn name(&self) -> &'static str {
        match self {
            Self::MainContent => "main-content",
            Self::Explorer => "EXPLORER",
        }
    }

    /// CSSセレクタを取得
    pub fn selector(&self) -> &'static str {
        match self {
            Self::MainContent => ".main-content",
            Self::Explorer => ".sidebar-left",
        }
    }
}

/// ウィンドウフォーカス状態
struct WindowState {
    /// 現在フォーカスされているペイン
    focused_pane: FocusedPane,
    /// Ctrl+w プレフィックスが押されたか
    ctrl_w_pending: bool,
    /// EXPLORER内で選択されているアイテムのインデックス
    explorer_selected_index: usize,
}

impl Default for WindowState {
    fn default() -> Self {
        Self {
            focused_pane: FocusedPane::MainContent,
            ctrl_w_pending: false,
            explorer_selected_index: 0,
        }
    }
}

thread_local! {
    static WINDOW_STATE: RefCell<WindowState> = RefCell::new(WindowState::default());
}

/// 現在のフォーカスペインを取得
pub fn current_focused_pane() -> FocusedPane {
    WINDOW_STATE.with(|state| state.borrow().focused_pane)
}

/// フォーカスペインを設定
pub fn set_focused_pane(pane: FocusedPane) {
    WINDOW_STATE.with(|state| {
        state.borrow_mut().focused_pane = pane;
    });
}

/// Ctrl+w プレフィックスが押されたかを取得
pub fn is_ctrl_w_pending() -> bool {
    WINDOW_STATE.with(|state| state.borrow().ctrl_w_pending)
}

/// Ctrl+w プレフィックスの状態を設定
pub fn set_ctrl_w_pending(pending: bool) {
    WINDOW_STATE.with(|state| {
        state.borrow_mut().ctrl_w_pending = pending;
    });
}

/// EXPLORER内の選択インデックスを取得
pub fn explorer_selected_index() -> usize {
    WINDOW_STATE.with(|state| state.borrow().explorer_selected_index)
}

/// EXPLORER内の選択インデックスを設定
pub fn set_explorer_selected_index(index: usize) {
    WINDOW_STATE.with(|state| {
        state.borrow_mut().explorer_selected_index = index;
    });
}

/// フォーカスを次のペインへ移動（サイクル）
pub fn cycle_focus() -> FocusedPane {
    WINDOW_STATE.with(|state| {
        let mut s = state.borrow_mut();
        s.focused_pane = match s.focused_pane {
            FocusedPane::MainContent => FocusedPane::Explorer,
            FocusedPane::Explorer => FocusedPane::MainContent,
        };
        s.focused_pane
    })
}

/// フォーカスを左のペインへ移動
pub fn focus_left() -> FocusedPane {
    set_focused_pane(FocusedPane::Explorer);
    FocusedPane::Explorer
}

/// フォーカスを右のペインへ移動
pub fn focus_right() -> FocusedPane {
    set_focused_pane(FocusedPane::MainContent);
    FocusedPane::MainContent
}
