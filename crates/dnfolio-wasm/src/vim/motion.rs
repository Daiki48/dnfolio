//! カーソル移動（モーション）
//!
//! hjkl移動、gg/G、w/b/e等のモーション処理

use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, Node};

use crate::dom::{SelectionHelper, TextNodeWalker, query_selector};
use crate::error::{DnfolioError, Result};
use crate::vim::cursor::{remove_block_cursor, update_block_cursor};
use crate::vim::mode::current_mode;

/// モーション処理
pub struct MotionHandler;

impl MotionHandler {
    /// hjkl移動
    pub fn move_cursor(direction: char) -> Result<()> {
        // 移動前にカーソルを削除して元のDOM構造を復元
        remove_block_cursor()?;

        let sel = SelectionHelper::get()?;
        let main_content = query_selector::<HtmlElement>(".main-content")?;

        // ビジュアルモードの場合は選択範囲を拡張
        if current_mode().is_visual() {
            return Self::extend_selection(direction);
        }

        // 移動前の位置を記録
        let before_node = sel.anchor_node();
        let before_offset = sel.anchor_offset();

        // j/k はブロック要素単位で移動（Neovim風）
        // 列位置を可能な限り維持する
        if direction == 'j' || direction == 'k' {
            if let Some(ref current_node) = before_node {
                Self::move_to_adjacent_block_with_column(
                    &sel,
                    current_node,
                    before_offset,
                    &main_content,
                    direction,
                )?;
            }
            update_block_cursor()?;
            return Ok(());
        }

        // h/l は文字単位の移動
        if let Some(ref current_node) = before_node {
            Self::move_character(&sel, current_node, before_offset, &main_content, direction)?;
        }

        // ブロックカーソルを更新
        update_block_cursor()?;

        Ok(())
    }

    /// 文字単位で移動（h/l用）
    /// Neovim風: 同じブロック要素内でテキストノードを超えて移動
    fn move_character(
        sel: &SelectionHelper,
        current_node: &Node,
        current_offset: u32,
        main_content: &HtmlElement,
        direction: char,
    ) -> Result<()> {
        let text = current_node.text_content().unwrap_or_default();
        let char_count = text.chars().count();
        let current_block = Self::get_containing_block(current_node);

        match direction {
            'l' => {
                // 右に移動
                if (current_offset as usize) < char_count.saturating_sub(1) {
                    // 同じノード内で右に移動
                    sel.collapse(current_node, current_offset + 1)?;
                } else {
                    // テキストノードの末尾 → 同じブロック内の次のテキストノードへ
                    let walker = TextNodeWalker::new(main_content)?;
                    if let Some(next_text) = walker.find_next_from(current_node)? {
                        // 次のテキストノードが同じブロック内かチェック
                        let next_block = Self::get_containing_block(&next_text);
                        if current_block.is_some()
                            && next_block.is_some()
                            && current_block
                                .as_ref()
                                .unwrap()
                                .is_same_node(next_block.as_ref().map(|e| e.unchecked_ref()))
                        {
                            // 同じブロック内なので移動
                            sel.collapse(&next_text, 0)?;
                        }
                        // 異なるブロックなら移動しない（Neovim風：行末で止まる）
                    }
                }
            }
            'h' => {
                // 左に移動
                if current_offset > 0 {
                    // 同じノード内で左に移動
                    sel.collapse(current_node, current_offset - 1)?;
                } else {
                    // テキストノードの先頭 → 同じブロック内の前のテキストノードへ
                    let walker = TextNodeWalker::new(main_content)?;
                    if let Some(prev_text) = walker.find_prev_from(current_node)? {
                        // 前のテキストノードが同じブロック内かチェック
                        let prev_block = Self::get_containing_block(&prev_text);
                        if current_block.is_some()
                            && prev_block.is_some()
                            && current_block
                                .as_ref()
                                .unwrap()
                                .is_same_node(prev_block.as_ref().map(|e| e.unchecked_ref()))
                        {
                            // 同じブロック内なので末尾に移動
                            let prev_text_content = prev_text.text_content().unwrap_or_default();
                            let prev_char_count = prev_text_content.chars().count();
                            let offset = if prev_char_count > 0 {
                                prev_char_count - 1
                            } else {
                                0
                            };
                            sel.collapse(&prev_text, offset as u32)?;
                        }
                        // 異なるブロックなら移動しない（Neovim風：行頭で止まる）
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }

    /// 次/前のブロック要素に移動（列位置を維持）
    /// コードブロック内では改行文字で区切られた行単位で移動
    fn move_to_adjacent_block_with_column(
        sel: &SelectionHelper,
        current_node: &Node,
        current_offset: u32,
        main_content: &HtmlElement,
        direction: char,
    ) -> Result<()> {
        let text = current_node.text_content().unwrap_or_default();
        let current_block = Self::get_containing_block(current_node);
        let current_col = Self::get_column_in_line(&text, current_offset as usize);

        // 現在のテキストノード内に改行がある場合（コードブロック等）
        // まず同じノード内で行移動を試みる
        if text.contains('\n') {
            if let Some(new_offset) =
                Self::move_within_multiline_text(&text, current_offset, direction)
            {
                sel.collapse(current_node, new_offset)?;
                return Ok(());
            }
        }

        // 同じブロック内で改行を越えて移動（シンタックスハイライトされたコードブロック対応）
        // 次の改行文字を探し、その後のテキストに移動
        if let Some(ref block) = current_block {
            match direction {
                'j' => {
                    // 次の改行を探して越える
                    if let Some((target_node, target_offset)) =
                        Self::find_position_after_next_newline(
                            current_node,
                            current_offset,
                            block,
                            main_content,
                        )?
                    {
                        let target_text = target_node.text_content().unwrap_or_default();
                        let target_char_count = target_text.chars().count();
                        // 列位置を維持（ノードの長さを超えないように）
                        let final_offset = if target_char_count == 0 {
                            0
                        } else {
                            let max_offset = (target_char_count - 1) as u32;
                            // target_offsetから始めて、列位置を加算
                            (target_offset + current_col as u32).min(max_offset)
                        };
                        sel.collapse(&target_node, final_offset)?;
                        return Ok(());
                    }
                }
                'k' => {
                    // 前の改行を探して、その行の先頭に移動
                    if let Some((target_node, target_offset)) =
                        Self::find_position_before_prev_newline(
                            current_node,
                            current_offset,
                            block,
                            main_content,
                        )?
                    {
                        let target_text = target_node.text_content().unwrap_or_default();
                        let target_char_count = target_text.chars().count();
                        // 列位置を維持（ノードの長さを超えないように）
                        let final_offset = if target_char_count == 0 {
                            0
                        } else {
                            let max_offset = (target_char_count - 1) as u32;
                            (target_offset + current_col as u32).min(max_offset)
                        };
                        sel.collapse(&target_node, final_offset)?;
                        return Ok(());
                    }
                }
                _ => {}
            }
        }

        // 次/前のブロック要素へ移動
        let target_block = match direction {
            'j' => Self::find_next_block_element(current_node, main_content)?,
            'k' => Self::find_prev_block_element(current_node, main_content)?,
            _ => None,
        };

        if let Some(block) = target_block {
            if let Some(text_node) = Self::find_first_text_node(&block) {
                if Self::is_inside_main_content(&text_node, main_content) {
                    let new_text = text_node.text_content().unwrap_or_default();
                    let char_count = new_text.chars().count();

                    // offsetが範囲内に収まるように調整
                    let target_offset = if char_count == 0 {
                        0
                    } else {
                        let max_offset = (char_count - 1) as u32;
                        (current_col as u32).min(max_offset)
                    };

                    sel.collapse(&text_node, target_offset)?;
                }
            }
        }
        Ok(())
    }

    /// 複数行テキスト内で行移動（コードブロック用）
    /// 移動可能な場合は新しいオフセットを返す
    fn move_within_multiline_text(text: &str, current_offset: u32, direction: char) -> Option<u32> {
        let chars: Vec<char> = text.chars().collect();
        let offset = current_offset as usize;

        // 現在の行の開始位置と終了位置を見つける
        let mut line_start = 0;
        let mut line_end = chars.len();

        // 現在位置より前の最後の改行を探す
        for (i, &c) in chars.iter().enumerate().take(offset) {
            if c == '\n' {
                line_start = i + 1;
            }
        }

        // 現在位置以降の最初の改行を探す
        for (i, &c) in chars.iter().enumerate().skip(offset) {
            if c == '\n' {
                line_end = i;
                break;
            }
        }

        // 現在の列位置（行内のオフセット）
        let current_col = offset - line_start;

        match direction {
            'j' => {
                // 次の行へ移動
                // 改行があり、かつその後にコンテンツがある場合のみ
                if line_end < chars.len() {
                    let next_line_start = line_end + 1;
                    // 次の行が存在するか確認（改行の後にコンテンツがあるか）
                    if next_line_start >= chars.len() {
                        return None; // 改行で終わっている場合は次の行なし
                    }
                    // 次の行の終了位置を見つける
                    let mut next_line_end = chars.len();
                    for (i, &c) in chars.iter().enumerate().skip(next_line_start) {
                        if c == '\n' {
                            next_line_end = i;
                            break;
                        }
                    }
                    let next_line_len = next_line_end - next_line_start;
                    // 列位置を維持（行が短い場合は行末）
                    let target_col = if current_col < next_line_len {
                        current_col
                    } else if next_line_len > 0 {
                        next_line_len - 1
                    } else {
                        0
                    };
                    Some((next_line_start + target_col) as u32)
                } else {
                    None // 最後の行なので次のブロックへ
                }
            }
            'k' => {
                // 前の行へ移動
                if line_start > 0 {
                    // 前の行の開始位置と終了位置を見つける
                    let prev_line_end = line_start - 1; // 改行文字の位置
                    let mut prev_line_start = 0;
                    for (i, &c) in chars.iter().enumerate().take(prev_line_end) {
                        if c == '\n' {
                            prev_line_start = i + 1;
                        }
                    }
                    let prev_line_len = prev_line_end - prev_line_start;
                    // 列位置を維持（行が短い場合は行末）
                    let target_col = if current_col < prev_line_len {
                        current_col
                    } else if prev_line_len > 0 {
                        prev_line_len - 1
                    } else {
                        0
                    };
                    Some((prev_line_start + target_col) as u32)
                } else {
                    None // 最初の行なので前のブロックへ
                }
            }
            _ => None,
        }
    }

    /// テキスト内での現在の列位置を取得（行内のオフセット）
    fn get_column_in_line(text: &str, offset: usize) -> usize {
        let chars: Vec<char> = text.chars().collect();
        let mut line_start = 0;
        for (i, &c) in chars.iter().enumerate().take(offset) {
            if c == '\n' {
                line_start = i + 1;
            }
        }
        offset - line_start
    }

    /// 次の改行の後の位置を探す（シンタックスハイライトされたコード対応）
    /// 複数のテキストノードを跨いで改行を探し、改行後のテキストノードとオフセットを返す
    fn find_position_after_next_newline(
        start_node: &Node,
        start_offset: u32,
        block: &Element,
        main_content: &HtmlElement,
    ) -> Result<Option<(Node, u32)>> {
        let walker = TextNodeWalker::new(main_content)?;
        walker.set_current(start_node);

        // まず現在のノード内で改行を探す
        let text = start_node.text_content().unwrap_or_default();
        let chars: Vec<char> = text.chars().collect();

        for (i, &c) in chars.iter().enumerate().skip(start_offset as usize) {
            if c == '\n' {
                // 改行を見つけた
                let next_pos = i + 1;
                if next_pos < chars.len() {
                    // 同じノード内に改行後のコンテンツがある
                    return Ok(Some((start_node.clone(), next_pos as u32)));
                } else {
                    // 改行がノードの最後なので、次のノードの先頭へ
                    if let Some(next_node) = walker.next()? {
                        let next_block = Self::get_containing_block(&next_node);
                        if next_block
                            .as_ref()
                            .map(|b| b.is_same_node(Some(block)))
                            .unwrap_or(false)
                        {
                            return Ok(Some((next_node, 0)));
                        }
                    }
                    return Ok(None);
                }
            }
        }

        // 現在のノードに改行がなかった場合、次のノードを探す
        while let Some(next_node) = walker.next()? {
            let next_block = Self::get_containing_block(&next_node);
            if !next_block
                .as_ref()
                .map(|b| b.is_same_node(Some(block)))
                .unwrap_or(false)
            {
                // 異なるブロックに入った
                return Ok(None);
            }

            let next_text = next_node.text_content().unwrap_or_default();
            let next_chars: Vec<char> = next_text.chars().collect();

            for (i, &c) in next_chars.iter().enumerate() {
                if c == '\n' {
                    let next_pos = i + 1;
                    if next_pos < next_chars.len() {
                        return Ok(Some((next_node, next_pos as u32)));
                    } else {
                        // さらに次のノードへ
                        if let Some(after_newline_node) = walker.next()? {
                            let after_block = Self::get_containing_block(&after_newline_node);
                            if after_block
                                .as_ref()
                                .map(|b| b.is_same_node(Some(block)))
                                .unwrap_or(false)
                            {
                                return Ok(Some((after_newline_node, 0)));
                            }
                        }
                        return Ok(None);
                    }
                }
            }
        }

        Ok(None)
    }

    /// 前の改行の位置を探す（前の行の先頭を見つける）
    fn find_position_before_prev_newline(
        start_node: &Node,
        start_offset: u32,
        block: &Element,
        main_content: &HtmlElement,
    ) -> Result<Option<(Node, u32)>> {
        let walker = TextNodeWalker::new(main_content)?;
        walker.set_current(start_node);

        // 現在位置より前で改行を探す
        let text = start_node.text_content().unwrap_or_default();
        let chars: Vec<char> = text.chars().collect();

        // 現在位置より前の最後の改行を探す
        let mut last_newline_in_current: Option<usize> = None;
        for (i, &c) in chars.iter().enumerate().take(start_offset as usize) {
            if c == '\n' {
                last_newline_in_current = Some(i);
            }
        }

        if let Some(newline_pos) = last_newline_in_current {
            // 現在のノード内で改行を見つけた
            // その改行の前の行の先頭を探す
            let mut line_start = 0;
            for (i, &c) in chars.iter().enumerate().take(newline_pos) {
                if c == '\n' {
                    line_start = i + 1;
                }
            }
            return Ok(Some((start_node.clone(), line_start as u32)));
        }

        // 前のノードを探す
        while let Some(prev_node) = walker.prev()? {
            let prev_block = Self::get_containing_block(&prev_node);
            if !prev_block
                .as_ref()
                .map(|b| b.is_same_node(Some(block)))
                .unwrap_or(false)
            {
                return Ok(None);
            }

            let prev_text = prev_node.text_content().unwrap_or_default();
            let prev_chars: Vec<char> = prev_text.chars().collect();

            // 前のノード内の最後の改行を探す
            let mut last_newline: Option<usize> = None;
            for (i, &c) in prev_chars.iter().enumerate() {
                if c == '\n' {
                    last_newline = Some(i);
                }
            }

            if let Some(newline_pos) = last_newline {
                // 改行を見つけた - その行の先頭を探す
                let mut line_start = 0;
                for (i, &c) in prev_chars.iter().enumerate().take(newline_pos) {
                    if c == '\n' {
                        line_start = i + 1;
                    }
                }
                return Ok(Some((prev_node, line_start as u32)));
            }
        }

        Ok(None)
    }

    /// 最初の行で指定列位置のオフセットを取得
    fn get_offset_at_column_in_first_line(text: &str, col: usize) -> u32 {
        let chars: Vec<char> = text.chars().collect();
        // 最初の改行までの長さを取得
        let first_line_len = chars.iter().position(|&c| c == '\n').unwrap_or(chars.len());
        let target_col = if col < first_line_len {
            col
        } else if first_line_len > 0 {
            first_line_len - 1
        } else {
            0
        };
        target_col as u32
    }

    /// 最後の行で指定列位置のオフセットを取得
    fn get_offset_at_column_in_last_line(text: &str, col: usize) -> u32 {
        let chars: Vec<char> = text.chars().collect();
        // 最後の改行位置を探す
        let last_newline = chars.iter().rposition(|&c| c == '\n');
        let last_line_start = last_newline.map(|i| i + 1).unwrap_or(0);
        let last_line_len = chars.len() - last_line_start;
        let target_col = if col < last_line_len {
            col
        } else if last_line_len > 0 {
            last_line_len - 1
        } else {
            0
        };
        (last_line_start + target_col) as u32
    }

    /// 次/前のブロック要素に移動（h/l用）
    fn move_to_adjacent_block(
        sel: &SelectionHelper,
        current_node: &Node,
        main_content: &HtmlElement,
        direction: char,
    ) -> Result<()> {
        match direction {
            'l' => {
                // 次のブロック要素の先頭に移動
                if let Some(next_block) = Self::find_next_block_element(current_node, main_content)?
                {
                    if let Some(text_node) = Self::find_first_text_node(&next_block) {
                        if Self::is_inside_main_content(&text_node, main_content) {
                            sel.collapse(&text_node, 0)?;
                        }
                    }
                }
            }
            'h' => {
                // 前のブロック要素の末尾に移動
                if let Some(prev_block) = Self::find_prev_block_element(current_node, main_content)?
                {
                    if let Some(text_node) = Self::find_last_text_node(&prev_block) {
                        if Self::is_inside_main_content(&text_node, main_content) {
                            let text = text_node.text_content().unwrap_or_default();
                            let char_count = text.chars().count();
                            let offset = if char_count > 0 { char_count - 1 } else { 0 };
                            sel.collapse(&text_node, offset as u32)?;
                        }
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }

    /// ノードがmain-content内にあるかチェック
    fn is_inside_main_content(node: &Node, main_content: &HtmlElement) -> bool {
        // main_contentがnodeを含んでいるかチェック
        main_content.contains(Some(node))
    }

    /// カーソル位置を検証し、有効でなければ調整（Neovim風）
    fn validate_cursor_position(sel: &SelectionHelper) -> Result<()> {
        if let Some(node) = sel.anchor_node() {
            // テキストノードでない場合、最も近いテキストノードに移動
            if node.node_type() != Node::TEXT_NODE {
                // 再帰的に子孫からテキストノードを探す
                if let Some(text_node) = Self::find_first_text_node(&node) {
                    let text = text_node.text_content().unwrap_or_default();
                    if !text.trim().is_empty() {
                        sel.collapse(&text_node, 0)?;
                        return Ok(());
                    }
                }

                // 見つからなければTreeWalkerで探す
                let main_content = query_selector::<HtmlElement>(".main-content")?;
                let walker = TextNodeWalker::new(&main_content)?;

                // 現在位置から次のテキストノードを探す
                if let Some(next) = walker.find_next_from(&node)? {
                    sel.collapse(&next, 0)?;
                }
            } else {
                // テキストノードの場合、オフセットが有効かチェック
                let text = node.text_content().unwrap_or_default();
                let char_count = text.chars().count();
                let offset = sel.anchor_offset() as usize;

                // オフセットがテキスト長を超えている場合、行末に調整
                if offset >= char_count && char_count > 0 {
                    sel.collapse(&node, (char_count - 1) as u32)?;
                }
            }
        }
        Ok(())
    }

    /// ノード内から最初のテキストノードを再帰的に探す
    /// button要素やcode-block-header内はスキップする
    fn find_first_text_node(node: &Node) -> Option<Node> {
        let child_nodes = node.child_nodes();
        for i in 0..child_nodes.length() {
            if let Some(child) = child_nodes.get(i) {
                if child.node_type() == Node::TEXT_NODE {
                    // テキストノードの親要素をチェック
                    if Self::is_valid_cursor_parent(&child) {
                        let text = child.text_content().unwrap_or_default();
                        if !text.trim().is_empty() {
                            return Some(child);
                        }
                    }
                } else if child.node_type() == Node::ELEMENT_NODE {
                    // スキップすべき要素かチェック
                    if !Self::should_skip_element(&child) {
                        // 再帰的に子孫を探す
                        if let Some(found) = Self::find_first_text_node(&child) {
                            return Some(found);
                        }
                    }
                }
            }
        }
        None
    }

    /// カーソルを置くべきでない要素かチェック
    fn should_skip_element(node: &Node) -> bool {
        if let Some(element) = node.dyn_ref::<Element>() {
            let tag_name = element.tag_name().to_lowercase();
            // button要素はスキップ
            if tag_name == "button" {
                return true;
            }
            // 特定のクラスを持つ要素はスキップ
            let class_list = element.class_list();
            if class_list.contains("code-block-header")
                || class_list.contains("code-copy-btn")
                || class_list.contains("code-lang")
            {
                return true;
            }
        }
        false
    }

    /// テキストノードの親がカーソルを置ける場所かチェック
    fn is_valid_cursor_parent(text_node: &Node) -> bool {
        if let Some(parent) = text_node.parent_element() {
            let tag_name = parent.tag_name().to_lowercase();
            // button内はNG
            if tag_name == "button" {
                return false;
            }
            // 特定のクラスを持つ要素内はNG
            let class_list = parent.class_list();
            if class_list.contains("code-block-header")
                || class_list.contains("code-copy-btn")
                || class_list.contains("code-lang")
            {
                return false;
            }
        }
        true
    }

    /// ブロック境界を超えて強制移動
    fn force_move_to_adjacent_node(
        sel: &SelectionHelper,
        current_node: &Node,
        direction: char,
    ) -> Result<()> {
        let main_content = query_selector::<HtmlElement>(".main-content")?;

        match direction {
            'l' => {
                // h/l は文字単位なので、次のテキストノードへ
                let walker = TextNodeWalker::new(&main_content)?;
                if let Some(next_node) = walker.find_next_from(current_node)? {
                    if Self::is_inside_main_content(&next_node, &main_content) {
                        sel.collapse(&next_node, 0)?;
                    }
                }
            }
            'h' => {
                let walker = TextNodeWalker::new(&main_content)?;
                if let Some(prev_node) = walker.find_prev_from(current_node)? {
                    if Self::is_inside_main_content(&prev_node, &main_content) {
                        let len = prev_node.text_content().unwrap_or_default().len();
                        let offset = if len > 0 { len - 1 } else { 0 };
                        sel.collapse(&prev_node, offset as u32)?;
                    }
                }
            }
            'j' => {
                // j は次のブロック要素へ移動
                if let Some(next_block) =
                    Self::find_next_block_element(current_node, &main_content)?
                {
                    if let Some(text_node) = Self::find_first_text_node(&next_block) {
                        if Self::is_inside_main_content(&text_node, &main_content) {
                            sel.collapse(&text_node, 0)?;
                        }
                    }
                }
            }
            'k' => {
                // k は前のブロック要素へ移動
                if let Some(prev_block) =
                    Self::find_prev_block_element(current_node, &main_content)?
                {
                    if let Some(text_node) = Self::find_last_text_node(&prev_block) {
                        if Self::is_inside_main_content(&text_node, &main_content) {
                            let len = text_node.text_content().unwrap_or_default().len();
                            let offset = if len > 0 { len - 1 } else { 0 };
                            sel.collapse(&text_node, offset as u32)?;
                        }
                    }
                }
            }
            _ => {}
        }

        Ok(())
    }

    /// 現在のノードが属するブロック要素を取得
    fn get_containing_block(node: &Node) -> Option<Element> {
        let block_tags = [
            "P",
            "LI",
            "H1",
            "H2",
            "H3",
            "H4",
            "H5",
            "H6",
            "BLOCKQUOTE",
            "PRE",
            "DIV",
            "TD",
            "TH",
        ];

        let mut current = node.parent_element();
        while let Some(el) = current {
            let tag = el.tag_name();
            if block_tags.contains(&tag.as_str()) {
                return Some(el);
            }
            current = el.parent_element();
        }
        None
    }

    /// 次のブロック要素を見つける
    fn find_next_block_element(
        current_node: &Node,
        main_content: &HtmlElement,
    ) -> Result<Option<Element>> {
        use wasm_bindgen::JsCast;

        let current_block = Self::get_containing_block(current_node);

        // main-content直下のブロック要素を取得
        let blocks = main_content
            .query_selector_all("p, li, h1, h2, h3, h4, h5, h6, blockquote, pre, td, th")
            .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;

        let mut found_current = false;
        for i in 0..blocks.length() {
            if let Some(block) = blocks.get(i) {
                if let Some(el) = block.dyn_ref::<Element>() {
                    // 現在のブロックを見つけたら、次のブロックを返す
                    if found_current {
                        // vim-cursor内は除外
                        if !el.class_list().contains("vim-cursor") {
                            return Ok(Some(el.clone()));
                        }
                    }

                    // 現在のブロックかチェック
                    if let Some(ref cb) = current_block {
                        if el.is_same_node(Some(cb)) {
                            found_current = true;
                        }
                    }
                }
            }
        }

        Ok(None)
    }

    /// 前のブロック要素を見つける
    fn find_prev_block_element(
        current_node: &Node,
        main_content: &HtmlElement,
    ) -> Result<Option<Element>> {
        use wasm_bindgen::JsCast;

        let current_block = Self::get_containing_block(current_node);

        // main-content直下のブロック要素を取得
        let blocks = main_content
            .query_selector_all("p, li, h1, h2, h3, h4, h5, h6, blockquote, pre, td, th")
            .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;

        let mut prev_block: Option<Element> = None;
        for i in 0..blocks.length() {
            if let Some(block) = blocks.get(i) {
                if let Some(el) = block.dyn_ref::<Element>() {
                    // 現在のブロックに到達したら、前のブロックを返す
                    if let Some(ref cb) = current_block {
                        if el.is_same_node(Some(cb)) {
                            return Ok(prev_block);
                        }
                    }

                    // vim-cursor内は除外
                    if !el.class_list().contains("vim-cursor") {
                        prev_block = Some(el.clone());
                    }
                }
            }
        }

        Ok(None)
    }

    /// ノード内の最後のテキストノードを見つける
    fn find_last_text_node(node: &Node) -> Option<Node> {
        let child_nodes = node.child_nodes();
        // 後ろから探す
        for i in (0..child_nodes.length()).rev() {
            if let Some(child) = child_nodes.get(i) {
                if child.node_type() == Node::TEXT_NODE {
                    if Self::is_valid_cursor_parent(&child) {
                        let text = child.text_content().unwrap_or_default();
                        if !text.trim().is_empty() {
                            return Some(child);
                        }
                    }
                } else if child.node_type() == Node::ELEMENT_NODE {
                    if !Self::should_skip_element(&child) {
                        if let Some(found) = Self::find_last_text_node(&child) {
                            return Some(found);
                        }
                    }
                }
            }
        }
        None
    }

    /// ビジュアルモードで選択範囲を拡張
    fn extend_selection(direction: char) -> Result<()> {
        let sel = SelectionHelper::get()?;

        match direction {
            'h' => sel.modify("extend", "backward", "character")?,
            'l' => sel.modify("extend", "forward", "character")?,
            'j' => sel.modify("extend", "forward", "line")?,
            'k' => sel.modify("extend", "backward", "line")?,
            _ => return Err(DnfolioError::DomError(format!("不明な方向: {direction}"))),
        }

        Ok(())
    }

    /// 先頭に移動（gg）
    pub fn move_to_top() -> Result<()> {
        // 移動前にカーソルを削除してDOM構造を復元
        remove_block_cursor()?;

        let main_content = query_selector::<HtmlElement>(".main-content")?;
        let walker = TextNodeWalker::new(&main_content)?;
        let nodes = walker.collect_filtered()?;

        if let Some(first_node) = nodes.first() {
            let sel = SelectionHelper::get()?;
            sel.collapse(first_node, 0)?;
            update_block_cursor()?;

            // ページトップにスクロール
            if let Some(window) = web_sys::window() {
                window.scroll_to_with_x_and_y(0.0, 0.0);
            }
        }

        Ok(())
    }

    /// 末尾に移動（G）
    pub fn move_to_bottom() -> Result<()> {
        // 移動前にカーソルを削除してDOM構造を復元
        remove_block_cursor()?;

        let main_content = query_selector::<HtmlElement>(".main-content")?;
        let walker = TextNodeWalker::new(&main_content)?;
        let nodes = walker.collect_filtered()?;

        if let Some(last_node) = nodes.last() {
            let sel = SelectionHelper::get()?;
            let text = last_node.text_content().unwrap_or_default();
            // 文字数でカウント（バイト数ではなく）
            let char_count = text.chars().count();
            let offset = if char_count > 0 { char_count - 1 } else { 0 };

            sel.collapse(last_node, offset as u32)?;
            update_block_cursor()?;

            // ページボトムにスクロール
            if let Some(window) = web_sys::window() {
                if let Some(doc) = window.document() {
                    if let Some(body) = doc.body() {
                        let height = body.scroll_height() as f64;
                        window.scroll_to_with_x_and_y(0.0, height);
                    }
                }
            }
        }

        Ok(())
    }

    /// 行頭に移動（0）
    pub fn move_to_line_start() -> Result<()> {
        let sel = SelectionHelper::get()?;
        sel.modify("move", "backward", "lineboundary")?;
        update_block_cursor()?;
        Ok(())
    }

    /// 行末に移動（$）
    pub fn move_to_line_end() -> Result<()> {
        let sel = SelectionHelper::get()?;
        sel.modify("move", "forward", "lineboundary")?;
        update_block_cursor()?;
        Ok(())
    }

    /// 半画面上にスクロール（Ctrl+U）
    pub fn scroll_half_page_up() -> Result<()> {
        if let Some(window) = web_sys::window() {
            let viewport_height = window
                .inner_height()
                .ok()
                .and_then(|v| v.as_f64())
                .unwrap_or(800.0);
            let scroll_y = window.scroll_y().unwrap_or(0.0);
            let new_scroll = (scroll_y - viewport_height / 2.0).max(0.0);
            window.scroll_to_with_x_and_y(0.0, new_scroll);

            // カーソルも移動
            Self::move_cursor_to_viewport_center()?;
        }
        Ok(())
    }

    /// 半画面下にスクロール（Ctrl+D）
    pub fn scroll_half_page_down() -> Result<()> {
        if let Some(window) = web_sys::window() {
            let viewport_height = window
                .inner_height()
                .ok()
                .and_then(|v| v.as_f64())
                .unwrap_or(800.0);
            let scroll_y = window.scroll_y().unwrap_or(0.0);
            let new_scroll = scroll_y + viewport_height / 2.0;
            window.scroll_to_with_x_and_y(0.0, new_scroll);

            // カーソルも移動
            Self::move_cursor_to_viewport_center()?;
        }
        Ok(())
    }

    /// カーソルをビューポート中央付近のテキストに移動
    fn move_cursor_to_viewport_center() -> Result<()> {
        remove_block_cursor()?;

        let main_content = query_selector::<HtmlElement>(".main-content")?;
        let walker = TextNodeWalker::new(&main_content)?;
        let nodes = walker.collect_filtered()?;

        if let Some(window) = web_sys::window() {
            let viewport_height = window
                .inner_height()
                .ok()
                .and_then(|v| v.as_f64())
                .unwrap_or(800.0);
            let target_y = viewport_height / 2.0;

            // ビューポート中央に最も近いノードを探す
            let mut best_node: Option<&Node> = None;
            let mut best_distance = f64::MAX;

            for node in &nodes {
                if let Some(parent) = node.parent_element() {
                    let rect = parent.get_bounding_client_rect();
                    let node_y = rect.top() + rect.height() / 2.0;
                    let distance = (node_y - target_y).abs();

                    if distance < best_distance {
                        best_distance = distance;
                        best_node = Some(node);
                    }
                }
            }

            if let Some(node) = best_node {
                let sel = SelectionHelper::get()?;
                sel.collapse(node, 0)?;
                update_block_cursor()?;
            }
        }

        Ok(())
    }

    /// 単語移動（w）
    pub fn move_word_forward() -> Result<()> {
        let sel = SelectionHelper::get()?;
        sel.modify("move", "forward", "word")?;
        update_block_cursor()?;
        Ok(())
    }

    /// 単語移動（b）
    pub fn move_word_backward() -> Result<()> {
        let sel = SelectionHelper::get()?;
        sel.modify("move", "backward", "word")?;
        update_block_cursor()?;
        Ok(())
    }

    /// 単語末尾移動（e）
    pub fn move_word_end() -> Result<()> {
        let sel = SelectionHelper::get()?;
        // eは単語の末尾に移動（Selection APIには直接対応するものがないので、wordで代用）
        sel.modify("move", "forward", "word")?;
        sel.modify("move", "backward", "character")?;
        update_block_cursor()?;
        Ok(())
    }
}
