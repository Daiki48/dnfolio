//! Selection APIラッパー
//!
//! テキスト選択とカーソル位置の操作

use web_sys::{Node, Range, Selection};

use crate::dom::elements::window;
use crate::error::{DnfolioError, Result};

/// Selection APIのヘルパー構造体
pub struct SelectionHelper {
    selection: Selection,
}

impl SelectionHelper {
    /// 現在のSelectionを取得
    pub fn get() -> Result<Self> {
        let win = window()?;
        let selection = win
            .get_selection()
            .map_err(|e| DnfolioError::SelectionError(format!("{e:?}")))?
            .ok_or_else(|| DnfolioError::SelectionError("Selectionが取得できません".to_string()))?;

        Ok(Self { selection })
    }

    /// 内部のSelectionへの参照を取得
    pub fn inner(&self) -> &Selection {
        &self.selection
    }

    /// アンカーノード（選択開始点）を取得
    pub fn anchor_node(&self) -> Option<Node> {
        self.selection.anchor_node()
    }

    /// アンカーオフセット（選択開始位置）を取得
    pub fn anchor_offset(&self) -> u32 {
        self.selection.anchor_offset()
    }

    /// フォーカスノード（選択終了点）を取得
    pub fn focus_node(&self) -> Option<Node> {
        self.selection.focus_node()
    }

    /// フォーカスオフセット（選択終了位置）を取得
    pub fn focus_offset(&self) -> u32 {
        self.selection.focus_offset()
    }

    /// 選択が折りたたまれているか（カーソルのみの状態か）
    pub fn is_collapsed(&self) -> bool {
        self.selection.is_collapsed()
    }

    /// Range数を取得
    pub fn range_count(&self) -> u32 {
        self.selection.range_count()
    }

    /// 指定インデックスのRangeを取得
    pub fn get_range_at(&self, index: u32) -> Result<Range> {
        self.selection
            .get_range_at(index)
            .map_err(|e| DnfolioError::SelectionError(format!("{e:?}")))
    }

    /// カーソルを移動（Selection.modify使用）
    ///
    /// # Arguments
    /// * `alter` - "move" or "extend"
    /// * `direction` - "forward", "backward", "left", "right"
    /// * `granularity` - "character", "word", "line", "lineboundary", etc.
    ///
    /// # Example
    /// ```ignore
    /// selection.modify("move", "forward", "character")?; // 1文字右へ
    /// selection.modify("move", "backward", "line")?;     // 1行上へ
    /// ```
    pub fn modify(&self, alter: &str, direction: &str, granularity: &str) -> Result<()> {
        self.selection
            .modify(alter, direction, granularity)
            .map_err(|e| DnfolioError::SelectionError(format!("{e:?}")))
    }

    /// 指定ノードの指定位置にカーソルを折りたたむ
    pub fn collapse(&self, node: &Node, offset: u32) -> Result<()> {
        self.selection
            .collapse_with_offset(Some(node), offset)
            .map_err(|e| DnfolioError::SelectionError(format!("{e:?}")))
    }

    /// 選択を拡張
    pub fn extend(&self, node: &Node, offset: u32) -> Result<()> {
        self.selection
            .extend_with_offset(node, offset)
            .map_err(|e| DnfolioError::SelectionError(format!("{e:?}")))
    }

    /// 全ての選択範囲をクリア
    pub fn remove_all_ranges(&self) -> Result<()> {
        self.selection
            .remove_all_ranges()
            .map_err(|e| DnfolioError::SelectionError(format!("{e:?}")))
    }

    /// Rangeを追加
    pub fn add_range(&self, range: &Range) -> Result<()> {
        self.selection
            .add_range(range)
            .map_err(|e| DnfolioError::SelectionError(format!("{e:?}")))
    }

    /// 選択範囲のテキストを取得
    pub fn to_string(&self) -> String {
        self.selection.to_string().as_string().unwrap_or_default()
    }
}
