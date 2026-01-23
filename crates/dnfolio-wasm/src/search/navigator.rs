//! ハイライトナビゲーション
//!
//! n/Nキーでハイライト間を移動

use crate::error::Result;
use crate::search::highlight::HIGHLIGHT_MANAGER;

/// ハイライトナビゲーター
pub struct HighlightNavigator;

impl HighlightNavigator {
    /// 次のハイライトへ移動（n）
    pub fn next() -> Result<()> {
        HIGHLIGHT_MANAGER.with(|manager| {
            let m = manager.borrow();
            // まずDOMと同期
            m.sync_with_dom()?;
            m.next()
        })
    }

    /// 前のハイライトへ移動（N）
    pub fn prev() -> Result<()> {
        HIGHLIGHT_MANAGER.with(|manager| {
            let m = manager.borrow();
            // まずDOMと同期
            m.sync_with_dom()?;
            m.prev()
        })
    }

    /// ハイライトが存在するか
    pub fn has_highlights() -> bool {
        HIGHLIGHT_MANAGER.with(|manager| manager.borrow().count() > 0)
    }

    /// 現在位置の情報を取得 (current_index, total_count)
    pub fn get_position() -> (usize, usize) {
        HIGHLIGHT_MANAGER.with(|manager| {
            let m = manager.borrow();
            (m.current_index(), m.count())
        })
    }
}
