//! 検索・ハイライト機能モジュール
//!
//! テキスト検索とハイライト表示、n/Nナビゲーションを提供

pub mod highlight;
pub mod index;
pub mod modal;
pub mod navigator;
pub mod tags;

pub use highlight::HighlightManager;
pub use index::{
    SEARCH_INDEX, SearchIndex, SearchMatch, load_search_index, search_articles, search_lines,
};
pub use modal::{
    SEARCH_MODAL_STATE, SearchModalState, modal_clear, modal_move_down, modal_move_up,
    modal_open_selected, perform_search,
};
pub use navigator::HighlightNavigator;
pub use tags::{
    TAGS_MODAL_STATE, TagInfo, TagsModalState, perform_tags_filter, tags_modal_clear,
    tags_modal_move_down, tags_modal_move_up, tags_modal_open_selected,
};
