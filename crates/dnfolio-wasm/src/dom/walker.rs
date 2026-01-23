//! TreeWalkerラッパー
//!
//! テキストノードを効率的に走査するためのヘルパー

use web_sys::{HtmlElement, Node, TreeWalker};

use crate::dom::elements::document;
use crate::error::{DnfolioError, Result};

/// NodeFilter.SHOW_TEXT の値（テキストノードのみ表示）
const SHOW_TEXT: u32 = 0x4;

/// テキストノードを走査するTreeWalkerラッパー
pub struct TextNodeWalker {
    walker: TreeWalker,
}

impl TextNodeWalker {
    /// 指定したルート要素内のテキストノードを走査するWalkerを作成
    ///
    /// # Arguments
    /// * `root` - 走査のルートとなる要素
    ///
    /// # Example
    /// ```ignore
    /// let main_content = query_selector::<HtmlElement>(".main-content")?;
    /// let walker = TextNodeWalker::new(&main_content)?;
    /// ```
    pub fn new(root: &HtmlElement) -> Result<Self> {
        let doc = document()?;

        let walker = doc
            .create_tree_walker_with_what_to_show(root, SHOW_TEXT)
            .map_err(|e| DnfolioError::TreeWalkerError(format!("{e:?}")))?;

        Ok(Self { walker })
    }

    /// 次のテキストノードを取得
    pub fn next(&self) -> Result<Option<Node>> {
        self.walker
            .next_node()
            .map_err(|e| DnfolioError::TreeWalkerError(format!("{e:?}")))
    }

    /// 前のテキストノードを取得
    pub fn prev(&self) -> Result<Option<Node>> {
        self.walker
            .previous_node()
            .map_err(|e| DnfolioError::TreeWalkerError(format!("{e:?}")))
    }

    /// 現在位置を設定
    pub fn set_current(&self, node: &Node) {
        self.walker.set_current_node(node);
    }

    /// ルートノードを取得
    pub fn root(&self) -> Node {
        self.walker.root()
    }

    /// 現在のノードを取得
    pub fn current_node(&self) -> Node {
        self.walker.current_node()
    }

    /// 指定ノードから次のテキストノードを探す
    pub fn find_next_from(&self, start: &Node) -> Result<Option<Node>> {
        self.set_current(start);
        self.next()
    }

    /// 指定ノードから前のテキストノードを探す
    pub fn find_prev_from(&self, start: &Node) -> Result<Option<Node>> {
        self.set_current(start);
        self.prev()
    }

    /// ルートにリセット
    pub fn reset(&self) {
        self.walker.set_current_node(&self.root());
    }

    /// 全テキストノードを収集（フィルタリング付き）
    ///
    /// 以下のノードは除外:
    /// - vim-cursorクラスを持つ親要素内のノード
    /// - 空白のみのノード
    pub fn collect_filtered(&self) -> Result<Vec<Node>> {
        self.reset();

        let mut nodes = Vec::new();
        while let Some(node) = self.next()? {
            // vim-cursorクラスを持つ親は除外
            if let Some(parent) = node.parent_element() {
                if parent.class_list().contains("vim-cursor") {
                    continue;
                }
            }

            // 空白のみのノードは除外
            if let Some(text) = node.text_content() {
                if !text.trim().is_empty() {
                    nodes.push(node);
                }
            }
        }

        Ok(nodes)
    }

    /// 全テキストノードを収集（フィルタリングなし）
    pub fn collect_all(&self) -> Result<Vec<Node>> {
        self.reset();

        let mut nodes = Vec::new();
        while let Some(node) = self.next()? {
            nodes.push(node);
        }

        Ok(nodes)
    }
}
