//! イベントハンドラー
//!
//! キーボードイベント、マウスイベント等のハンドリング

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlElement, HtmlInputElement, KeyboardEvent};

use crate::dom::{document, query_selector_optional, validate_command, validate_search_query};
use crate::error::Result;
use crate::search::highlight::{apply_highlight, remove_highlights};
use crate::search::modal::{
    modal_clear, modal_move_down, modal_move_up, modal_open_selected, perform_search,
};
use crate::search::navigator::HighlightNavigator;
use crate::search::tags::{
    perform_tags_filter, tags_modal_clear, tags_modal_move_down, tags_modal_move_up,
    tags_modal_open_selected,
};
use crate::ui::{CommandLine, Toast};
use crate::vim::command::CommandExecutor;
use crate::vim::cursor::{update_block_cursor, update_block_cursor_no_scroll};
use crate::vim::mode::{EditorMode, current_mode, set_mode, with_editor_state};
use crate::vim::motion::MotionHandler;
use crate::vim::window::{
    FocusedPane, current_focused_pane, cycle_focus, focus_left, focus_right, is_ctrl_w_pending,
    set_ctrl_w_pending,
};

/// 全イベントハンドラーを登録
pub fn setup_all_event_handlers() -> Result<()> {
    setup_keyboard_handler()?;
    setup_commandline_handler()?;
    setup_window_button_handlers()?;
    setup_scroll_handler()?;
    setup_mouse_handlers()?;
    setup_modal_handlers()?;
    setup_code_copy_handlers()?;
    setup_folder_toggle_handlers()?;
    setup_hamburger_handler()?;
    setup_commandline_tap_handler()?;
    setup_bottomsheet_handlers()?;
    setup_highlight_nav_handlers()?;

    web_sys::console::log_1(&"  ✓ Event handlers registered".into());
    Ok(())
}

/// キーボードイベントハンドラーを登録
fn setup_keyboard_handler() -> Result<()> {
    let doc = document()?;

    let handler = Closure::wrap(Box::new(move |e: KeyboardEvent| {
        if let Err(err) = handle_keydown(&e) {
            web_sys::console::error_1(&format!("Keydown error: {err}").into());
        }
    }) as Box<dyn Fn(KeyboardEvent)>);

    doc.add_event_listener_with_callback("keydown", handler.as_ref().unchecked_ref())
        .map_err(|e| crate::error::DnfolioError::DomError(format!("{e:?}")))?;
    handler.forget();

    // main-content への直接入力を防止（IME入力、ペースト等も含む）
    // contenteditable="true" による編集を完全にブロック
    if let Some(main_content) = query_selector_optional::<HtmlElement>(".main-content")? {
        let beforeinput_handler = Closure::wrap(Box::new(move |e: web_sys::InputEvent| {
            // main-contentへの全ての入力をキャンセル
            // これによりIME入力、ペースト、ドラッグ&ドロップ等も防止
            e.prevent_default();
        }) as Box<dyn Fn(web_sys::InputEvent)>);

        main_content
            .add_event_listener_with_callback(
                "beforeinput",
                beforeinput_handler.as_ref().unchecked_ref(),
            )
            .map_err(|e| crate::error::DnfolioError::DomError(format!("{e:?}")))?;
        beforeinput_handler.forget();
    }

    Ok(())
}

/// キーダウンイベント処理
fn handle_keydown(e: &KeyboardEvent) -> Result<()> {
    let key = e.key();
    let target = e.target();

    // IME変換中は処理をスキップ（日本語入力等）
    if e.is_composing() {
        e.prevent_default();
        return Ok(());
    }

    // key="Process" はIMEがキーを処理中であることを示す
    // この場合もデフォルト動作を防止して入力をブロック
    if key == "Process" {
        e.prevent_default();
        return Ok(());
    }

    // モーダルが開いている場合の処理
    if is_modal_open()? {
        return handle_modal_keydown(e);
    }

    // 入力欄（コマンドライン）にフォーカスがある場合
    if let Some(target) = &target {
        if let Some(el) = target.dyn_ref::<HtmlElement>() {
            let tag = el.tag_name().to_uppercase();
            if tag == "INPUT" || tag == "TEXTAREA" {
                // Escapeでコマンドラインを閉じる
                if key == "Escape" {
                    CommandLine::deactivate()?;
                    e.prevent_default();
                }
                // Enterでコマンド実行
                if key == "Enter" {
                    if let Some(input) = target.dyn_ref::<HtmlInputElement>() {
                        let value = input.value();
                        if value.starts_with(':') {
                            // コマンドの長さを検証
                            if validate_command(&value).is_ok() {
                                execute_command(&value)?;
                            } else {
                                Toast::error(
                                    "E484: Command too long",
                                    "コマンドが長すぎます",
                                    "!",
                                )?;
                            }
                            CommandLine::deactivate()?;
                            e.prevent_default();
                        } else if value.starts_with('/') {
                            // 検索実行（長さ検証付き）
                            let query = value.trim_start_matches('/');
                            if !query.is_empty() {
                                if validate_search_query(query).is_ok() {
                                    apply_highlight(query, None)?;
                                } else {
                                    Toast::error(
                                        "E486: Search pattern too long",
                                        "検索パターンが長すぎます",
                                        "!",
                                    )?;
                                }
                            }
                            CommandLine::deactivate()?;
                            e.prevent_default();
                        }
                    }
                }
                return Ok(());
            }
        }
    }

    // メインコンテンツでの文字入力を防止（静的サイトなので編集不可）
    // 1文字のキー入力（修飾キーなし）は Vim コマンドとして処理するか防止
    // 注意: key.chars().count() を使用してUnicode文字数を判定（UTF-8バイト長ではない）
    let is_printable = key.chars().count() == 1 && !e.ctrl_key() && !e.alt_key() && !e.meta_key();

    // gw プレフィックスが押された後の処理（ウィンドウ操作）
    // 注意: Ctrl+W はブラウザの「タブを閉じる」ショートカットと競合するため、
    //       gw プレフィックスを代替として使用
    if is_ctrl_w_pending() {
        e.prevent_default();
        set_ctrl_w_pending(false);
        crate::ui::StatusLine::clear_pending_key()?;

        match key.as_str() {
            // gw h: 左のペイン（EXPLORER）へ
            "h" => {
                switch_to_explorer()?;
                return Ok(());
            }
            // gw l: 右のペイン（main-content）へ
            "l" => {
                switch_to_main_content()?;
                return Ok(());
            }
            // gw w: ペイン間をサイクル
            "w" => {
                let new_pane = cycle_focus();
                apply_pane_focus(new_pane)?;
                return Ok(());
            }
            _ => {
                // 不明なキーの場合は何もしない
                return Ok(());
            }
        }
    }

    // Ctrl キーコンビネーション
    if e.ctrl_key() {
        match key.as_str() {
            // Ctrl+U: 半画面上スクロール
            "u" => {
                e.prevent_default();
                MotionHandler::scroll_half_page_up()?;
                return Ok(());
            }
            // Ctrl+D: 半画面下スクロール
            "d" => {
                e.prevent_default();
                MotionHandler::scroll_half_page_down()?;
                return Ok(());
            }
            // Ctrl+K: 検索モーダルを開く（VSCode風）
            "k" => {
                e.prevent_default();
                open_search_modal()?;
                return Ok(());
            }
            _ => {}
        }
    }

    // EXPLORERにフォーカスがある場合の処理
    if current_focused_pane() == FocusedPane::Explorer {
        return handle_explorer_keydown(e);
    }

    let mode = current_mode();

    match key.as_str() {
        // hjkl移動
        "h" | "j" | "k" | "l" => {
            e.prevent_default();
            MotionHandler::move_cursor(key.chars().next().unwrap())?;
        }

        // g プレフィックスコマンド
        // - gg: 先頭へ
        // - gw + h/l/w: ウィンドウ（ペイン）操作
        "g" => {
            e.prevent_default();
            let last_key = with_editor_state(|s| s.last_key());
            if last_key == Some('g') {
                // gg: 先頭へ移動
                MotionHandler::move_to_top()?;
                with_editor_state(|s| s.set_last_key(None));
            } else {
                with_editor_state(|s| s.set_last_key(Some('g')));
                // ステータスラインにペンディングキーを表示
                crate::ui::StatusLine::show_pending_key("g")?;
            }
        }

        // gw: ウィンドウ操作プレフィックス（g が前に押されていた場合）
        "w" if !e.ctrl_key() => {
            let last_key = with_editor_state(|s| s.last_key());
            if last_key == Some('g') {
                // gw プレフィックス: 次のキーを待つ
                e.prevent_default();
                with_editor_state(|s| s.set_last_key(None));
                set_ctrl_w_pending(true);
                crate::ui::StatusLine::show_pending_key("gw")?;
            } else {
                // 通常の w: 単語移動
                e.prevent_default();
                MotionHandler::move_word_forward()?;
            }
        }

        // G (末尾へ)
        "G" => {
            e.prevent_default();
            MotionHandler::move_to_bottom()?;
        }

        // 0 (行頭へ)
        "0" => {
            e.prevent_default();
            MotionHandler::move_to_line_start()?;
        }

        // $ (行末へ)
        "$" => {
            e.prevent_default();
            MotionHandler::move_to_line_end()?;
        }

        // b (単語戻り)
        "b" => {
            e.prevent_default();
            MotionHandler::move_word_backward()?;
        }

        // e (単語末尾)
        "e" => {
            e.prevent_default();
            MotionHandler::move_word_end()?;
        }

        // v (ビジュアルモード)
        "v" => {
            e.prevent_default();
            if mode == EditorMode::Normal {
                set_mode(EditorMode::Visual)?;
                Toast::info("-- VISUAL --", "hjklで選択範囲を拡張、yでヤンク", "👁")?;
            } else if mode == EditorMode::Visual {
                set_mode(EditorMode::Normal)?;
                update_block_cursor()?;
            }
        }

        // V (ビジュアルラインモード)
        "V" => {
            e.prevent_default();
            if mode == EditorMode::Normal {
                set_mode(EditorMode::VisualLine)?;
                Toast::info("-- VISUAL LINE --", "j/kで行選択を拡張、yでヤンク", "👁")?;
            } else if mode == EditorMode::VisualLine {
                set_mode(EditorMode::Normal)?;
                update_block_cursor()?;
            }
        }

        // y (ヤンク/コピー)
        "y" => {
            e.prevent_default();
            if mode.is_visual() {
                copy_selection()?;
                set_mode(EditorMode::Normal)?;
                update_block_cursor()?;
            }
        }

        // Escape (ノーマルモードに戻る)
        "Escape" => {
            e.prevent_default();
            if mode != EditorMode::Normal {
                set_mode(EditorMode::Normal)?;
                update_block_cursor()?;
            }
            // ハイライトも削除
            remove_highlights()?;
        }

        // : (コマンドモード)
        ":" => {
            e.prevent_default();
            CommandLine::activate()?;
        }

        // / (ページ内検索 - Neovim風)
        "/" => {
            e.prevent_default();
            CommandLine::activate_search()?;
        }

        // n (次のハイライト)
        "n" => {
            e.prevent_default();
            HighlightNavigator::next()?;
        }

        // N (前のハイライト)
        "N" => {
            e.prevent_default();
            HighlightNavigator::prev()?;
        }

        // i, a (インサートモード風の演出)
        "i" | "a" => {
            e.prevent_default();
            set_mode(EditorMode::Insert)?;
            Toast::warn(
                "E21: Cannot modify",
                "This is a read-only buffer (静的サイトです)",
                "📝",
            )?;
            // 少し待ってノーマルモードに戻す
            let callback = Closure::wrap(Box::new(move || {
                let _ = set_mode(EditorMode::Normal);
                let _ = update_block_cursor();
            }) as Box<dyn Fn()>);

            if let Some(window) = web_sys::window() {
                let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                    callback.as_ref().unchecked_ref(),
                    1000,
                );
            }
            callback.forget();
        }

        // Enter (リンクを開く)
        "Enter" => {
            e.prevent_default();
            open_link_at_cursor()?;
        }

        _ => {
            // 未知のキーはlast_keyをリセット
            with_editor_state(|s| s.set_last_key(None));

            // 印刷可能文字の場合はデフォルト動作を防止（テキスト入力防止）
            if is_printable {
                e.prevent_default();
            }
        }
    }

    Ok(())
}

/// モーダルが開いている時のキーハンドリング
fn handle_modal_keydown(e: &KeyboardEvent) -> Result<()> {
    let key = e.key();

    // 常にデフォルト動作を防止（背景の本文編集を防ぐ）
    // ただし、INPUT内の通常入力は許可する必要があるので、
    // 特定のキー以外は防止しない
    let target = e.target();
    let is_in_input = target
        .as_ref()
        .and_then(|t| t.dyn_ref::<HtmlElement>())
        .map(|el| el.tag_name().to_uppercase() == "INPUT")
        .unwrap_or(false);

    // どのモーダルが開いているか判定
    let is_tags_modal = is_tags_modal_open()?;

    match key.as_str() {
        "Escape" => {
            e.prevent_default();
            handle_modal_escape(is_in_input)?;
        }
        // j/k for navigation in NORMAL mode (not in input)
        "j" if !is_in_input => {
            e.prevent_default();
            if is_tags_modal {
                tags_modal_move_down()?;
            } else {
                modal_move_down()?;
            }
        }
        "k" if !is_in_input => {
            e.prevent_default();
            if is_tags_modal {
                tags_modal_move_up()?;
            } else {
                modal_move_up()?;
            }
        }
        "Enter" if !is_in_input => {
            e.prevent_default();
            if is_tags_modal {
                tags_modal_open_selected()?;
            } else {
                modal_open_selected()?;
            }
        }
        // INSERTモードに戻る
        "i" | "a" if !is_in_input => {
            e.prevent_default();
            focus_modal_input()?;
        }
        _ => {
            // INPUT内でない場合は全てのキーを防止
            if !is_in_input {
                e.prevent_default();
            }
        }
    }

    Ok(())
}

/// タグモーダルが開いているか
fn is_tags_modal_open() -> Result<bool> {
    if let Some(modal) = query_selector_optional::<HtmlElement>("#tags-modal")? {
        if modal.class_list().contains("open") {
            return Ok(true);
        }
    }
    Ok(false)
}

/// モーダル内でEscapeキーを処理
fn handle_modal_escape(is_in_input: bool) -> Result<()> {
    if is_in_input {
        // INSERT → NORMAL: 結果リストにフォーカス
        focus_modal_results()?;
        update_modal_mode_indicator("NORMAL")?;
    } else {
        // NORMAL → 閉じる
        close_all_modals();
    }
    Ok(())
}

/// モーダルの入力欄にフォーカス
fn focus_modal_input() -> Result<()> {
    // 検索モーダルの入力欄
    if let Some(input) = query_selector_optional::<HtmlInputElement>("#grep-search-input")? {
        if let Some(modal) = query_selector_optional::<HtmlElement>("#search-modal")? {
            if modal.class_list().contains("open") {
                input.focus().ok();
                update_modal_mode_indicator("INSERT")?;
                return Ok(());
            }
        }
    }
    // タグモーダルの入力欄
    if let Some(input) = query_selector_optional::<HtmlInputElement>("#tags-filter-input")? {
        if let Some(modal) = query_selector_optional::<HtmlElement>("#tags-modal")? {
            if modal.class_list().contains("open") {
                input.focus().ok();
                update_modal_mode_indicator("INSERT")?;
                return Ok(());
            }
        }
    }
    Ok(())
}

/// モーダルの結果リストにフォーカス
fn focus_modal_results() -> Result<()> {
    // 検索モーダルの結果リスト
    if let Some(list) = query_selector_optional::<HtmlElement>("#grep-results-list")? {
        if let Some(modal) = query_selector_optional::<HtmlElement>("#search-modal")? {
            if modal.class_list().contains("open") {
                list.set_attribute("tabindex", "-1").ok();
                list.focus().ok();
                return Ok(());
            }
        }
    }
    // タグモーダルの結果リスト
    if let Some(list) = query_selector_optional::<HtmlElement>("#tags-list")? {
        if let Some(modal) = query_selector_optional::<HtmlElement>("#tags-modal")? {
            if modal.class_list().contains("open") {
                list.set_attribute("tabindex", "-1").ok();
                list.focus().ok();
                return Ok(());
            }
        }
    }
    Ok(())
}

/// モーダルのモードインジケーターを更新
fn update_modal_mode_indicator(mode: &str) -> Result<()> {
    // 検索モーダル
    if let Some(indicator) = query_selector_optional::<HtmlElement>("#search-mode-indicator")? {
        indicator.set_text_content(Some(mode));
        if mode == "INSERT" {
            indicator.class_list().remove_1("mode-normal").ok();
            indicator.class_list().add_1("mode-insert").ok();
        } else {
            indicator.class_list().remove_1("mode-insert").ok();
            indicator.class_list().add_1("mode-normal").ok();
        }
    }
    // タグモーダル
    if let Some(indicator) = query_selector_optional::<HtmlElement>("#tags-mode-indicator")? {
        indicator.set_text_content(Some(mode));
        if mode == "INSERT" {
            indicator.class_list().remove_1("mode-normal").ok();
            indicator.class_list().add_1("mode-insert").ok();
        } else {
            indicator.class_list().remove_1("mode-insert").ok();
            indicator.class_list().add_1("mode-normal").ok();
        }
    }
    Ok(())
}

/// 全モーダルを閉じる
fn close_all_modals() {
    if let Ok(Some(modal)) = query_selector_optional::<HtmlElement>("#search-modal") {
        if modal.class_list().contains("open") {
            close_search_modal();
            return;
        }
    }
    if let Ok(Some(modal)) = query_selector_optional::<HtmlElement>("#tags-modal") {
        if modal.class_list().contains("open") {
            close_tags_modal();
        }
    }
}

/// コマンドを実行
fn execute_command(cmd: &str) -> Result<()> {
    if let Some(result) = CommandExecutor::execute(cmd)? {
        match result.toast_type.as_str() {
            "warn" => Toast::warn(&result.title, &result.message, &result.icon)?,
            "error" => Toast::error(&result.title, &result.message, &result.icon)?,
            "success" => Toast::success(&result.title, &result.message, &result.icon)?,
            _ => Toast::info(&result.title, &result.message, &result.icon)?,
        }
    }
    Ok(())
}

/// モーダルが開いているか確認
fn is_modal_open() -> Result<bool> {
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

/// 選択範囲をコピー
fn copy_selection() -> Result<()> {
    if let Some(window) = web_sys::window() {
        if let Some(sel) = window.get_selection().ok().flatten() {
            let text = sel.to_string();
            let text_str = text.as_string().unwrap_or_default();
            if !text_str.is_empty() {
                // クリップボードにコピー
                let clipboard = window.navigator().clipboard();
                let _ = clipboard.write_text(&text_str);
                Toast::info("Yanked!", &format!("{} characters", text_str.len()), "📋")?;
            }
        }
    }
    Ok(())
}

/// カーソル位置のリンクを開く
fn open_link_at_cursor() -> Result<()> {
    if let Some(window) = web_sys::window() {
        if let Some(sel) = window.get_selection().ok().flatten() {
            if let Some(node) = sel.anchor_node() {
                let mut current = Some(node);
                while let Some(n) = current {
                    if let Some(el) = n.dyn_ref::<HtmlElement>() {
                        if el.tag_name().to_uppercase() == "A" {
                            if let Some(href) = el.get_attribute("href") {
                                if href.starts_with('#') {
                                    // アンカーリンク - hashのみ変更してスクロール
                                    let _ = window.location().set_hash(&href);
                                    // 該当要素にスクロール
                                    if let Some(doc) = window.document() {
                                        let id = href.trim_start_matches('#');
                                        if let Some(target) = doc.get_element_by_id(id) {
                                            target.scroll_into_view();
                                        }
                                    }
                                } else {
                                    // 外部リンク
                                    let _ = window.open_with_url_and_target(&href, "_blank");
                                }
                                return Ok(());
                            }
                        }
                    }
                    current = n.parent_node();
                }
            }
        }
    }
    Ok(())
}

/// コマンドラインのイベントハンドラー
fn setup_commandline_handler() -> Result<()> {
    // コマンドラインの入力イベントをリッスン（/検索のリアクティブハイライト用）
    if let Some(input) = query_selector_optional::<HtmlInputElement>("#commandline-input")? {
        let handler = Closure::wrap(Box::new(move |_: web_sys::InputEvent| {
            if let Ok(Some(input)) =
                query_selector_optional::<HtmlInputElement>("#commandline-input")
            {
                let value = input.value();
                // /で始まる場合はリアクティブにハイライト
                if let Some(query) = value.strip_prefix('/') {
                    // 長さ制限を超えている場合は処理をスキップ
                    if query.len() > crate::dom::MAX_SEARCH_QUERY_LEN {
                        return;
                    }
                    if !query.is_empty() {
                        let _ = apply_highlight(query, None);
                    } else {
                        // クエリが空の場合はハイライトを削除
                        let _ = remove_highlights();
                    }
                }
            }
        }) as Box<dyn Fn(web_sys::InputEvent)>);

        input
            .add_event_listener_with_callback("input", handler.as_ref().unchecked_ref())
            .map_err(|e| crate::error::DnfolioError::DomError(format!("{e:?}")))?;
        handler.forget();
    }
    Ok(())
}

/// ウィンドウボタン（−□×）のイベントハンドラー
fn setup_window_button_handlers() -> Result<()> {
    // 閉じるボタン
    if let Some(btn) = query_selector_optional::<HtmlElement>(".btn-close")? {
        let handler = Closure::wrap(Box::new(move || {
            let messages = [
                (
                    "E32: No file name",
                    "バッファを保存してから終了してください...というのは冗談です",
                    "!",
                ),
                (
                    "Nice try!",
                    "このウィンドウは閉じられません。Neovimではないので。",
                    "😏",
                ),
                (
                    ":q!",
                    "本当に閉じたいなら :q! を入力してください（嘘です）",
                    "📝",
                ),
                (
                    "Segmentation fault",
                    "Core dumped...していません。Rustなので。",
                    "🦀",
                ),
            ];
            let idx = (js_sys::Math::random() * messages.len() as f64) as usize;
            let (title, msg, icon) = messages[idx.min(messages.len() - 1)];
            let _ = Toast::warn(title, msg, icon);
        }) as Box<dyn Fn()>);

        btn.add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())
            .map_err(|e| crate::error::DnfolioError::DomError(format!("{e:?}")))?;
        handler.forget();
    }

    // 最小化ボタン
    if let Some(btn) = query_selector_optional::<HtmlElement>(".btn-minimize")? {
        let handler = Closure::wrap(Box::new(move || {
            let messages = [
                (
                    "window.minimize is not a function",
                    "TypeError: ブラウザAPIにそんなメソッドはありません",
                    "🔧",
                ),
                (
                    ":hide",
                    "Neovimならバッファを隠せますが、ここはWebです",
                    "📦",
                ),
                (
                    "cargo build --release",
                    "この機能はリリースビルドに含まれていません",
                    "🦀",
                ),
            ];
            let idx = (js_sys::Math::random() * messages.len() as f64) as usize;
            let (title, msg, icon) = messages[idx.min(messages.len() - 1)];
            let _ = Toast::info(title, msg, icon);
        }) as Box<dyn Fn()>);

        btn.add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())
            .map_err(|e| crate::error::DnfolioError::DomError(format!("{e:?}")))?;
        handler.forget();
    }

    // 最大化ボタン
    if let Some(btn) = query_selector_optional::<HtmlElement>(".btn-maximize")? {
        let handler = Closure::wrap(Box::new(move || {
            let messages = [
                (
                    "F11",
                    "フルスクリーンはブラウザにお任せしています（F11キー推奨）",
                    "⌨️",
                ),
                (
                    "width: 100vw",
                    "すでに最大幅です。これ以上は物理的に無理です。",
                    "📏",
                ),
                (
                    "panic!()",
                    "thread 'main' panicked at 'not implemented'",
                    "🦀",
                ),
            ];
            let idx = (js_sys::Math::random() * messages.len() as f64) as usize;
            let (title, msg, icon) = messages[idx.min(messages.len() - 1)];
            let _ = Toast::info(title, msg, icon);
        }) as Box<dyn Fn()>);

        btn.add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())
            .map_err(|e| crate::error::DnfolioError::DomError(format!("{e:?}")))?;
        handler.forget();
    }

    Ok(())
}

/// スクロールイベントハンドラー
fn setup_scroll_handler() -> Result<()> {
    let handler = Closure::wrap(Box::new(move || {
        let _ = crate::ui::StatusLine::update_scroll_position();
        // OUTLINE（目次）の現在位置を更新
        let _ = crate::ui::update_active_heading();
    }) as Box<dyn Fn()>);

    if let Some(window) = web_sys::window() {
        window
            .add_event_listener_with_callback("scroll", handler.as_ref().unchecked_ref())
            .map_err(|e| crate::error::DnfolioError::DomError(format!("{e:?}")))?;
    }
    handler.forget();

    // 初期表示
    crate::ui::StatusLine::update_scroll_position()?;
    crate::ui::update_active_heading()?;

    Ok(())
}

/// モーダルのイベントハンドラー
fn setup_modal_handlers() -> Result<()> {
    // 検索モーダルの閉じるボタン
    if let Some(btn) = query_selector_optional::<HtmlElement>("#search-modal-close")? {
        let handler = Closure::wrap(Box::new(move || {
            close_search_modal();
        }) as Box<dyn Fn()>);

        btn.add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())
            .map_err(|e| crate::error::DnfolioError::DomError(format!("{e:?}")))
            .ok();
        handler.forget();
    }

    // 検索モーダル外クリックで閉じる
    if let Some(modal) = query_selector_optional::<HtmlElement>("#search-modal")? {
        let handler = Closure::wrap(Box::new(move |e: web_sys::MouseEvent| {
            if let Some(target) = e.target() {
                if let Some(el) = target.dyn_ref::<HtmlElement>() {
                    if el.id() == "search-modal" {
                        close_search_modal();
                    }
                }
            }
        }) as Box<dyn Fn(web_sys::MouseEvent)>);

        modal
            .add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())
            .map_err(|e| crate::error::DnfolioError::DomError(format!("{e:?}")))
            .ok();
        handler.forget();
    }

    // タグモーダルの閉じるボタン
    if let Some(btn) = query_selector_optional::<HtmlElement>("#tags-modal-close")? {
        let handler = Closure::wrap(Box::new(move || {
            close_tags_modal();
        }) as Box<dyn Fn()>);

        btn.add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())
            .map_err(|e| crate::error::DnfolioError::DomError(format!("{e:?}")))
            .ok();
        handler.forget();
    }

    // タグモーダル外クリックで閉じる
    if let Some(modal) = query_selector_optional::<HtmlElement>("#tags-modal")? {
        let handler = Closure::wrap(Box::new(move |e: web_sys::MouseEvent| {
            if let Some(target) = e.target() {
                if let Some(el) = target.dyn_ref::<HtmlElement>() {
                    if el.id() == "tags-modal" {
                        close_tags_modal();
                    }
                }
            }
        }) as Box<dyn Fn(web_sys::MouseEvent)>);

        modal
            .add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())
            .map_err(|e| crate::error::DnfolioError::DomError(format!("{e:?}")))
            .ok();
        handler.forget();
    }

    // 検索モーダルの結果アイテムをタップで開く（イベント委譲）
    if let Some(list) = query_selector_optional::<HtmlElement>("#grep-results-list")? {
        let handler = Closure::wrap(Box::new(move |e: web_sys::MouseEvent| {
            // クリックされた要素から .search-result-item を探す
            if let Some(target) = e.target() {
                if let Some(el) = target.dyn_ref::<web_sys::Element>() {
                    if let Some(item) = el.closest(".search-result-item").ok().flatten() {
                        if let Some(index_str) = item.get_attribute("data-index") {
                            if let Ok(index) = index_str.parse::<usize>() {
                                let _ = crate::search::modal::modal_select_and_open(index);
                            }
                        }
                    }
                }
            }
        }) as Box<dyn Fn(web_sys::MouseEvent)>);

        list.add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())
            .map_err(|e| crate::error::DnfolioError::DomError(format!("{e:?}")))
            .ok();
        handler.forget();
    }

    // タグモーダルの結果アイテムをタップで開く（イベント委譲）
    if let Some(list) = query_selector_optional::<HtmlElement>("#tags-list")? {
        let handler = Closure::wrap(Box::new(move |e: web_sys::MouseEvent| {
            if let Some(target) = e.target() {
                if let Some(el) = target.dyn_ref::<web_sys::Element>() {
                    if let Some(item) = el.closest(".tag-result-item").ok().flatten() {
                        if let Some(index_str) = item.get_attribute("data-index") {
                            if let Ok(index) = index_str.parse::<usize>() {
                                let _ = crate::search::tags::tags_modal_select_and_open(index);
                            }
                        }
                    }
                }
            }
        }) as Box<dyn Fn(web_sys::MouseEvent)>);

        list.add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())
            .map_err(|e| crate::error::DnfolioError::DomError(format!("{e:?}")))
            .ok();
        handler.forget();
    }

    Ok(())
}

/// タグモーダルを閉じる
fn close_tags_modal() {
    if let Ok(Some(modal)) = query_selector_optional::<HtmlElement>("#tags-modal") {
        modal.class_list().remove_1("open").ok();
    }
    // メインコンテンツにフォーカスを戻す
    restore_main_content_focus();
}

/// 検索モーダルを開く
pub fn open_search_modal() -> Result<()> {
    // まずカーソルを削除（背景の編集を防ぐ）
    crate::vim::cursor::remove_block_cursor()?;

    if let Some(modal) = query_selector_optional::<HtmlElement>("#search-modal")? {
        // openクラスを追加してモーダルを表示
        modal.class_list().add_1("open").ok();

        // 入力欄にフォーカス（INSERTモード）
        if let Some(input) = query_selector_optional::<HtmlInputElement>("#grep-search-input")? {
            input.set_value("");
            input.focus().ok();

            // 検索入力ハンドラを設定
            setup_search_input_handler(&input)?;
        }

        // モーダル状態をクリア
        modal_clear()?;

        // モードインジケーターをINSERTに
        update_modal_mode_indicator("INSERT")?;
    }
    Ok(())
}

/// 検索入力ハンドラを設定
fn setup_search_input_handler(input: &HtmlInputElement) -> Result<()> {
    let handler = Closure::wrap(Box::new(move |_: web_sys::InputEvent| {
        if let Ok(Some(input)) = query_selector_optional::<HtmlInputElement>("#grep-search-input") {
            let query = input.value();
            // 長さ制限を超えている場合は処理をスキップ
            if query.len() > crate::dom::MAX_SEARCH_QUERY_LEN {
                web_sys::console::warn_1(&"Search query too long, skipping".into());
                return;
            }
            // 非同期検索を実行
            wasm_bindgen_futures::spawn_local(async move {
                if let Err(e) = perform_search(query).await {
                    web_sys::console::error_1(&format!("Search error: {e}").into());
                }
            });
        }
    }) as Box<dyn Fn(web_sys::InputEvent)>);

    input
        .add_event_listener_with_callback("input", handler.as_ref().unchecked_ref())
        .map_err(|e| crate::error::DnfolioError::DomError(format!("{e:?}")))?;
    handler.forget();

    Ok(())
}

/// 検索モーダルを閉じる
fn close_search_modal() {
    if let Ok(Some(modal)) = query_selector_optional::<HtmlElement>("#search-modal") {
        modal.class_list().remove_1("open").ok();
    }
    // メインコンテンツにフォーカスを戻す
    restore_main_content_focus();
}

/// タグモーダルを開く
pub fn open_tags_modal() -> Result<()> {
    // まずカーソルを削除（背景の編集を防ぐ）
    crate::vim::cursor::remove_block_cursor()?;

    if let Some(modal) = query_selector_optional::<HtmlElement>("#tags-modal")? {
        // openクラスを追加してモーダルを表示
        modal.class_list().add_1("open").ok();

        // 入力欄にフォーカス（INSERTモード）
        if let Some(input) = query_selector_optional::<HtmlInputElement>("#tags-filter-input")? {
            input.set_value("");
            input.focus().ok();

            // タグ入力ハンドラを設定
            setup_tags_input_handler(&input)?;
        }

        // モーダル状態をクリア
        tags_modal_clear()?;

        // 初期タグ一覧を表示
        wasm_bindgen_futures::spawn_local(async {
            if let Err(e) = perform_tags_filter(String::new()).await {
                web_sys::console::error_1(&format!("Tags filter error: {e}").into());
            }
        });

        // モードインジケーターをINSERTに
        update_modal_mode_indicator("INSERT")?;
    }
    Ok(())
}

/// タグ入力ハンドラを設定
fn setup_tags_input_handler(input: &HtmlInputElement) -> Result<()> {
    let handler = Closure::wrap(Box::new(move |_: web_sys::InputEvent| {
        if let Ok(Some(input)) = query_selector_optional::<HtmlInputElement>("#tags-filter-input") {
            let query = input.value();
            // 長さ制限を超えている場合は処理をスキップ
            if query.len() > crate::dom::MAX_SEARCH_QUERY_LEN {
                web_sys::console::warn_1(&"Tag filter query too long, skipping".into());
                return;
            }
            // 非同期でフィルタリング実行
            wasm_bindgen_futures::spawn_local(async move {
                if let Err(e) = perform_tags_filter(query).await {
                    web_sys::console::error_1(&format!("Tags filter error: {e}").into());
                }
            });
        }
    }) as Box<dyn Fn(web_sys::InputEvent)>);

    input
        .add_event_listener_with_callback("input", handler.as_ref().unchecked_ref())
        .map_err(|e| crate::error::DnfolioError::DomError(format!("{e:?}")))?;
    handler.forget();

    Ok(())
}

/// メインコンテンツにフォーカスを戻す
fn restore_main_content_focus() {
    if let Ok(Some(main)) = query_selector_optional::<HtmlElement>(".main-content") {
        main.set_attribute("tabindex", "-1").ok();
        main.focus().ok();
    }
    let _ = update_block_cursor();
}

/// フォルダのアコーディオン開閉ハンドラーを登録
fn setup_folder_toggle_handlers() -> Result<()> {
    let doc = crate::dom::document()?;
    let toggles = doc
        .query_selector_all(".folder-toggle")
        .map_err(|e| crate::error::DnfolioError::DomError(format!("{e:?}")))?;

    for i in 0..toggles.length() {
        if let Some(node) = toggles.get(i) {
            if let Some(el) = node.dyn_ref::<HtmlElement>() {
                let el_clone = el.clone();
                let handler = Closure::wrap(Box::new(move |e: web_sys::MouseEvent| {
                    e.stop_propagation();

                    // 親の .folder-item に collapsed クラスをトグル
                    if let Some(parent) = el_clone.parent_element() {
                        if parent.class_list().contains("folder-item") {
                            let _ = parent.class_list().toggle("collapsed");
                        }
                    }
                })
                    as Box<dyn FnMut(web_sys::MouseEvent)>);

                el.add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())
                    .map_err(|e| crate::error::DnfolioError::DomError(format!("{e:?}")))?;
                handler.forget();
            }
        }
    }

    Ok(())
}

/// コードブロックのコピーボタンハンドラーを登録
fn setup_code_copy_handlers() -> Result<()> {
    let doc = crate::dom::document()?;
    let buttons = doc
        .query_selector_all(".code-copy-btn")
        .map_err(|e| crate::error::DnfolioError::DomError(format!("{e:?}")))?;

    for i in 0..buttons.length() {
        if let Some(node) = buttons.get(i) {
            if let Some(btn) = node.dyn_ref::<HtmlElement>() {
                let btn_clone = btn.clone();
                let handler = Closure::wrap(Box::new(move |e: web_sys::MouseEvent| {
                    e.prevent_default();
                    e.stop_propagation();

                    if let Some(el) = btn_clone.dyn_ref::<web_sys::Element>() {
                        if let Some(code) = el.get_attribute("data-code") {
                            // HTMLエスケープを解除
                            let decoded = code
                                .replace("&amp;", "&")
                                .replace("&lt;", "<")
                                .replace("&gt;", ">")
                                .replace("&quot;", "\"")
                                .replace("&#39;", "'");

                            // クリップボードにコピー
                            if let Some(window) = web_sys::window() {
                                let clipboard = window.navigator().clipboard();
                                let _ = clipboard.write_text(&decoded);
                                let _ = Toast::success("コピーしました!", "", "📋");
                            }
                        }
                    }
                })
                    as Box<dyn FnMut(web_sys::MouseEvent)>);

                btn.add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())
                    .map_err(|e| crate::error::DnfolioError::DomError(format!("{e:?}")))?;
                handler.forget();
            }
        }
    }

    Ok(())
}

/// ヘッダーアンカーリンクのクリックハンドラを設定
fn setup_anchor_link_handlers() -> Result<()> {
    let doc = crate::dom::document()?;
    let anchors = doc
        .query_selector_all(".header-anchor-link")
        .map_err(|e| crate::error::DnfolioError::DomError(format!("{e:?}")))?;

    web_sys::console::log_1(&format!("Setting up {} anchor handlers", anchors.length()).into());

    for i in 0..anchors.length() {
        if let Some(node) = anchors.get(i) {
            if let Some(anchor) = node.dyn_ref::<HtmlElement>() {
                let anchor_clone = anchor.clone();
                let handler = Closure::wrap(Box::new(move |e: web_sys::MouseEvent| {
                    web_sys::console::log_1(&"Anchor click handler called".into());
                    e.prevent_default();
                    e.stop_propagation();

                    if let Some(el) = anchor_clone.dyn_ref::<web_sys::Element>() {
                        if let Some(href) = el.get_attribute("href") {
                            web_sys::console::log_1(&format!("Anchor href: {}", href).into());

                            if let Some(window) = web_sys::window() {
                                // 該当要素にスクロール（先に実行）
                                if let Some(doc) = window.document() {
                                    let id = href.trim_start_matches('#');
                                    web_sys::console::log_1(
                                        &format!("Looking for element with id: {}", id).into(),
                                    );

                                    if let Some(target) = doc.get_element_by_id(id) {
                                        // getBoundingClientRectで位置を取得してスクロール
                                        // OUTLINEと同じ位置になるようマージンを設定
                                        let rect = target.get_bounding_client_rect();
                                        let scroll_y = window.scroll_y().unwrap_or(0.0);
                                        let target_y = (scroll_y + rect.top() - 60.0).max(0.0);

                                        window.scroll_to_with_x_and_y(0.0, target_y);
                                    }
                                }

                                // pushStateでURLを更新（スクロールを発生させない）
                                if let Some(history) = window.history().ok() {
                                    let current_url = window.location().href().unwrap_or_default();
                                    let base_url =
                                        current_url.split('#').next().unwrap_or(&current_url);
                                    let new_url = format!("{}{}", base_url, href);
                                    let _ = history.push_state_with_url(
                                        &wasm_bindgen::JsValue::NULL,
                                        "",
                                        Some(&new_url),
                                    );
                                }
                            }
                        }
                    }
                }) as Box<dyn Fn(web_sys::MouseEvent)>);

                anchor
                    .add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())
                    .map_err(|e| crate::error::DnfolioError::DomError(format!("{e:?}")))?;
                handler.forget();
            }
        }
    }

    Ok(())
}

/// マウスイベントハンドラー（クリック、選択）
fn setup_mouse_handlers() -> Result<()> {
    let main_content = match query_selector_optional::<HtmlElement>(".main-content")? {
        Some(el) => el,
        None => return Ok(()),
    };

    // ヘッダーアンカーリンクのクリックハンドラ
    setup_anchor_link_handlers()?;

    // クリック時にカーソル更新（アンカーリンククリック時はスキップ）
    let click_handler = Closure::wrap(Box::new(move |e: web_sys::MouseEvent| {
        // クリック対象がアンカーリンクかチェック
        if let Some(target) = e.target() {
            if let Some(el) = target.dyn_ref::<web_sys::Element>() {
                // アンカーリンクまたはその子要素の場合はスキップ
                if el.closest(".header-anchor-link").ok().flatten().is_some() {
                    return;
                }
            }

            // 行番号クリック検出（左端約50px内のクリック）
            if let Some(html_el) = target.dyn_ref::<HtmlElement>() {
                let click_x = e.client_x() as f64;
                if crate::ui::is_line_number_click(html_el, click_x) {
                    // main-content直下のブロック要素を取得
                    if let Some(block_el) = crate::ui::get_block_element(html_el) {
                        let _ = crate::ui::set_current_line(&block_el);
                        return;
                    }
                }
            }
        }

        // 行番号以外をクリックした場合はcurrent-lineをクリア
        let _ = crate::ui::clear_current_line();

        // 少し遅延してカーソル更新（マウス操作なのでスクロール抑制）
        let callback = Closure::wrap(Box::new(move || {
            let _ = update_block_cursor_no_scroll();
        }) as Box<dyn Fn()>);

        if let Some(window) = web_sys::window() {
            let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                callback.as_ref().unchecked_ref(),
                10,
            );
        }
        callback.forget();
    }) as Box<dyn Fn(web_sys::MouseEvent)>);

    main_content
        .add_event_listener_with_callback("click", click_handler.as_ref().unchecked_ref())
        .map_err(|e| crate::error::DnfolioError::DomError(format!("{e:?}")))?;
    click_handler.forget();

    // マウスアップ時にビジュアルモード切り替え（アンカーリンククリック時はスキップ）
    let mouseup_handler = Closure::wrap(Box::new(move |e: web_sys::MouseEvent| {
        // クリック対象がアンカーリンクかチェック
        if let Some(target) = e.target() {
            if let Some(el) = target.dyn_ref::<web_sys::Element>() {
                if el.closest(".header-anchor-link").ok().flatten().is_some() {
                    return;
                }
            }
        }

        let callback = Closure::wrap(Box::new(move || {
            if let Some(window) = web_sys::window() {
                if let Some(sel) = window.get_selection().ok().flatten() {
                    let text = sel.to_string().as_string().unwrap_or_default();
                    if !text.is_empty() && !sel.is_collapsed() {
                        // テキストが選択されている → ビジュアルモード
                        let mode = current_mode();
                        if !mode.is_visual() {
                            let _ = set_mode(EditorMode::Visual);
                        }
                    } else {
                        // 選択なし → ノーマルモード
                        let mode = current_mode();
                        if mode.is_visual() {
                            let _ = set_mode(EditorMode::Normal);
                        }
                        let _ = update_block_cursor_no_scroll();
                    }
                }
            }
        }) as Box<dyn Fn()>);

        if let Some(window) = web_sys::window() {
            let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                callback.as_ref().unchecked_ref(),
                10,
            );
        }
        callback.forget();
    }) as Box<dyn Fn(web_sys::MouseEvent)>);

    main_content
        .add_event_listener_with_callback("mouseup", mouseup_handler.as_ref().unchecked_ref())
        .map_err(|e| crate::error::DnfolioError::DomError(format!("{e:?}")))?;
    mouseup_handler.forget();

    Ok(())
}

// =============================================================================
// ウィンドウ（ペイン）フォーカス関連
// =============================================================================

/// EXPLORERにフォーカスを切り替え
fn switch_to_explorer() -> Result<()> {
    focus_left();
    apply_pane_focus(FocusedPane::Explorer)?;
    Ok(())
}

/// main-contentにフォーカスを切り替え
fn switch_to_main_content() -> Result<()> {
    focus_right();
    apply_pane_focus(FocusedPane::MainContent)?;
    Ok(())
}

/// ペインフォーカスを適用（視覚的フィードバック）
fn apply_pane_focus(pane: FocusedPane) -> Result<()> {
    // 全ペインからfocusedクラスを削除
    if let Some(main) = query_selector_optional::<HtmlElement>(".main-content")? {
        main.class_list().remove_1("pane-focused").ok();
    }
    if let Some(sidebar) = query_selector_optional::<HtmlElement>(".sidebar-left")? {
        sidebar.class_list().remove_1("pane-focused").ok();
    }

    match pane {
        FocusedPane::Explorer => {
            // EXPLORERにフォーカス
            if let Some(sidebar) = query_selector_optional::<HtmlElement>(".sidebar-left")? {
                sidebar.class_list().add_1("pane-focused").ok();
                // フォーカス可能にしてフォーカスを設定
                sidebar.set_attribute("tabindex", "-1").ok();
                sidebar.focus().ok();
            }
            // main-contentのカーソルを非表示
            crate::vim::cursor::remove_block_cursor()?;
            // 最初のファイルを選択状態に
            highlight_explorer_item(0)?;
            // ステータスラインを更新
            crate::ui::StatusLine::set_pane_indicator("EXPLORER")?;
        }
        FocusedPane::MainContent => {
            // main-contentにフォーカス
            if let Some(main) = query_selector_optional::<HtmlElement>(".main-content")? {
                main.class_list().add_1("pane-focused").ok();
                main.focus().ok();
            }
            // EXPLORERの選択をクリア
            clear_explorer_selection()?;
            // カーソルを復元
            update_block_cursor()?;
            // ステータスラインを更新
            crate::ui::StatusLine::set_pane_indicator("")?;
        }
    }

    // トースト通知（フォーカス移動を明示）
    Toast::info(&format!("→ {}", pane.name()), "gw h/l でペイン移動", "🪟")?;

    Ok(())
}

/// EXPLORERでのキーボード操作を処理
fn handle_explorer_keydown(e: &KeyboardEvent) -> Result<()> {
    let key = e.key();

    match key.as_str() {
        // j: 下に移動
        "j" => {
            e.prevent_default();
            explorer_move_selection(1)?;
        }
        // k: 上に移動
        "k" => {
            e.prevent_default();
            explorer_move_selection(-1)?;
        }
        // Enter: 選択したファイルを開く
        "Enter" => {
            e.prevent_default();
            explorer_open_selected()?;
        }
        // Escape: main-contentに戻る
        "Escape" => {
            e.prevent_default();
            switch_to_main_content()?;
        }
        // gg: 先頭へ
        "g" => {
            e.prevent_default();
            let last_key = with_editor_state(|s| s.last_key());
            if last_key == Some('g') {
                crate::vim::window::set_explorer_selected_index(0);
                highlight_explorer_item(0)?;
                with_editor_state(|s| s.set_last_key(None));
            } else {
                with_editor_state(|s| s.set_last_key(Some('g')));
            }
        }
        // G: 末尾へ
        "G" => {
            e.prevent_default();
            let count = get_explorer_item_count()?;
            if count > 0 {
                let last_index = count - 1;
                crate::vim::window::set_explorer_selected_index(last_index);
                highlight_explorer_item(last_index)?;
            }
        }
        // gw プレフィックス（EXPLORER内でのウィンドウ操作）
        "w" => {
            let last_key = with_editor_state(|s| s.last_key());
            if last_key == Some('g') {
                e.prevent_default();
                with_editor_state(|s| s.set_last_key(None));
                set_ctrl_w_pending(true);
                crate::ui::StatusLine::show_pending_key("gw")?;
            }
        }
        _ => {}
    }

    Ok(())
}

/// EXPLORER内のアイテム数を取得
fn get_explorer_item_count() -> Result<usize> {
    let doc = document()?;
    // sidebar-left内のリンク要素をカウント
    let items = doc
        .query_selector_all(".sidebar-left .file-tree a, .sidebar-left .sidebar-content a")
        .map_err(|e| crate::error::DnfolioError::DomError(format!("{e:?}")))?;
    Ok(items.length() as usize)
}

/// EXPLORER内の選択を移動
fn explorer_move_selection(delta: i32) -> Result<()> {
    let count = get_explorer_item_count()?;
    if count == 0 {
        return Ok(());
    }

    let current = crate::vim::window::explorer_selected_index();
    let new_index = if delta > 0 {
        (current + delta as usize).min(count - 1)
    } else {
        current.saturating_sub((-delta) as usize)
    };

    crate::vim::window::set_explorer_selected_index(new_index);
    highlight_explorer_item(new_index)?;

    Ok(())
}

/// EXPLORER内のアイテムをハイライト
fn highlight_explorer_item(index: usize) -> Result<()> {
    let doc = document()?;

    // 全てのハイライトを削除
    let items = doc
        .query_selector_all(".sidebar-left .file-tree a, .sidebar-left .sidebar-content a")
        .map_err(|e| crate::error::DnfolioError::DomError(format!("{e:?}")))?;

    for i in 0..items.length() {
        if let Some(node) = items.get(i) {
            if let Some(el) = node.dyn_ref::<HtmlElement>() {
                el.class_list().remove_1("explorer-selected").ok();
            }
        }
    }

    // 指定インデックスをハイライト
    if let Some(node) = items.get(index as u32) {
        if let Some(el) = node.dyn_ref::<HtmlElement>() {
            el.class_list().add_1("explorer-selected").ok();
            // スクロールして見える位置に
            el.scroll_into_view();
        }
    }

    Ok(())
}

/// EXPLORERの選択をクリア
fn clear_explorer_selection() -> Result<()> {
    let doc = document()?;
    let items = doc
        .query_selector_all(".sidebar-left .explorer-selected")
        .map_err(|e| crate::error::DnfolioError::DomError(format!("{e:?}")))?;

    for i in 0..items.length() {
        if let Some(node) = items.get(i) {
            if let Some(el) = node.dyn_ref::<HtmlElement>() {
                el.class_list().remove_1("explorer-selected").ok();
            }
        }
    }

    Ok(())
}

/// 選択されたEXPLORERアイテムを開く
fn explorer_open_selected() -> Result<()> {
    let doc = document()?;
    let index = crate::vim::window::explorer_selected_index();

    let items = doc
        .query_selector_all(".sidebar-left .file-tree a, .sidebar-left .sidebar-content a")
        .map_err(|e| crate::error::DnfolioError::DomError(format!("{e:?}")))?;

    if let Some(node) = items.get(index as u32) {
        if let Some(el) = node.dyn_ref::<HtmlElement>() {
            if let Some(href) = el.get_attribute("href") {
                // ページ遷移
                if let Some(window) = web_sys::window() {
                    let _ = window.location().set_href(&href);
                }
            }
        }
    }

    Ok(())
}

/// ハンバーガーボタンとオーバーレイのクリックハンドラー
fn setup_hamburger_handler() -> Result<()> {
    // ハンバーガーボタン: サイドバーをトグル
    if let Some(btn) = query_selector_optional::<HtmlElement>("#hamburger-btn")? {
        let handler = Closure::wrap(Box::new(move || {
            if let Ok(Some(sidebar)) = query_selector_optional::<HtmlElement>("#sidebar-left") {
                let _ = sidebar.class_list().toggle("is-open");
            }
            if let Ok(Some(overlay)) = query_selector_optional::<HtmlElement>("#overlay") {
                let _ = overlay.class_list().toggle("is-open");
            }
        }) as Box<dyn Fn()>);

        btn.add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())
            .map_err(|e| crate::error::DnfolioError::DomError(format!("{e:?}")))?;
        handler.forget();
    }

    // オーバーレイ: クリックでサイドバーを閉じる
    if let Some(overlay) = query_selector_optional::<HtmlElement>("#overlay")? {
        let handler = Closure::wrap(Box::new(move || {
            if let Ok(Some(sidebar)) = query_selector_optional::<HtmlElement>("#sidebar-left") {
                sidebar.class_list().remove_1("is-open").ok();
            }
            if let Ok(Some(overlay)) = query_selector_optional::<HtmlElement>("#overlay") {
                overlay.class_list().remove_1("is-open").ok();
            }
        }) as Box<dyn Fn()>);

        overlay
            .add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())
            .map_err(|e| crate::error::DnfolioError::DomError(format!("{e:?}")))?;
        handler.forget();
    }

    Ok(())
}

/// コマンドラインのタップハンドラー（モバイル用ボトムシート表示）
fn setup_commandline_tap_handler() -> Result<()> {
    if let Some(commandline) = query_selector_optional::<HtmlElement>(".commandline")? {
        let handler = Closure::wrap(Box::new(move |e: web_sys::Event| {
            // デスクトップ幅（992px超）ではボトムシートを表示しない
            if let Some(window) = web_sys::window() {
                let width = window
                    .inner_width()
                    .ok()
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.0);
                if width > 992.0 {
                    return;
                }
            }
            // モバイルではinputのフォーカスを防止してボトムシートを表示
            e.prevent_default();
            open_command_bottomsheet();
        }) as Box<dyn Fn(web_sys::Event)>);

        commandline
            .add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())
            .map_err(|e| crate::error::DnfolioError::DomError(format!("{e:?}")))?;
        handler.forget();
    }
    Ok(())
}

/// ボトムシートを開く
fn open_command_bottomsheet() {
    if let Ok(Some(sheet)) = query_selector_optional::<HtmlElement>("#command-bottomsheet") {
        sheet.class_list().add_1("is-open").ok();
    }
    if let Ok(Some(overlay)) = query_selector_optional::<HtmlElement>("#bottomsheet-overlay") {
        overlay.class_list().add_1("is-open").ok();
    }
}

/// ボトムシートを閉じる
fn close_command_bottomsheet() {
    if let Ok(Some(sheet)) = query_selector_optional::<HtmlElement>("#command-bottomsheet") {
        sheet.class_list().remove_1("is-open").ok();
    }
    if let Ok(Some(overlay)) = query_selector_optional::<HtmlElement>("#bottomsheet-overlay") {
        overlay.class_list().remove_1("is-open").ok();
    }
}

/// ボトムシートのイベントハンドラー（オーバーレイクリック + プリセット選択）
fn setup_bottomsheet_handlers() -> Result<()> {
    // オーバーレイクリックでボトムシートを閉じる
    if let Some(overlay) = query_selector_optional::<HtmlElement>("#bottomsheet-overlay")? {
        let handler = Closure::wrap(Box::new(move || {
            close_command_bottomsheet();
        }) as Box<dyn Fn()>);

        overlay
            .add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())
            .map_err(|e| crate::error::DnfolioError::DomError(format!("{e:?}")))?;
        handler.forget();
    }

    // プリセットボタンのクリックハンドラー
    let doc = document()?;
    let buttons = doc
        .query_selector_all(".bottomsheet-preset-btn")
        .map_err(|e| crate::error::DnfolioError::DomError(format!("{e:?}")))?;

    for i in 0..buttons.length() {
        if let Some(node) = buttons.get(i) {
            if let Some(btn) = node.dyn_ref::<HtmlElement>() {
                let handler = Closure::wrap(Box::new(move |e: web_sys::Event| {
                    if let Some(target) = e.current_target() {
                        if let Some(el) = target.dyn_ref::<HtmlElement>() {
                            if let Some(cmd) = el.get_attribute("data-command") {
                                // ボトムシートを閉じる
                                close_command_bottomsheet();

                                if cmd.starts_with('/') {
                                    // 検索モード: コマンドラインを検索入力状態にする
                                    let _ = CommandLine::activate_search();
                                } else {
                                    // タイプライター演出でコマンドを実行
                                    CommandLine::typewriter_execute(&cmd);
                                }
                            }
                        }
                    }
                }) as Box<dyn Fn(web_sys::Event)>);

                btn.add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())
                    .map_err(|e| crate::error::DnfolioError::DomError(format!("{e:?}")))?;
                handler.forget();
            }
        }
    }

    Ok(())
}

/// ハイライトナビゲーションボタンのクリックハンドラー
fn setup_highlight_nav_handlers() -> Result<()> {
    // 次へボタン（↓）
    if let Some(btn) = query_selector_optional::<HtmlElement>("#highlight-nav-next")? {
        let handler = Closure::wrap(Box::new(move || {
            let _ = HighlightNavigator::next();
        }) as Box<dyn Fn()>);

        btn.add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())
            .map_err(|e| crate::error::DnfolioError::DomError(format!("{e:?}")))?;
        handler.forget();
    }

    // 前へボタン（↑）
    if let Some(btn) = query_selector_optional::<HtmlElement>("#highlight-nav-prev")? {
        let handler = Closure::wrap(Box::new(move || {
            let _ = HighlightNavigator::prev();
        }) as Box<dyn Fn()>);

        btn.add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())
            .map_err(|e| crate::error::DnfolioError::DomError(format!("{e:?}")))?;
        handler.forget();
    }

    Ok(())
}
