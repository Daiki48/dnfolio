//! dnfolio WASM モジュール
//!
//! Neovim風UIのインタラクティブ機能をRust/WASMで実装
//!
//! # モジュール構成
//!
//! - `error` - 共通エラー型
//! - `dom` - DOM操作ヘルパー
//!   - `elements` - 要素取得
//!   - `walker` - TreeWalkerラッパー
//!   - `selection` - Selection APIラッパー
//! - `vim` - Vim機能
//!   - `mode` - EditorMode, EditorState
//!   - `cursor` - BlockCursor
//!   - `motion` - hjkl移動
//!   - `command` - コマンド処理
//! - `search` - 検索・ハイライト機能
//!   - `highlight` - HighlightManager
//!   - `navigator` - n/Nナビゲーション
//! - `ui` - UIコンポーネント
//!   - `toast` - トースト通知
//!   - `statusline` - ステータスライン
//!   - `commandline` - コマンドライン
//! - `events` - イベントハンドラー

use wasm_bindgen::prelude::*;

pub mod dom;
pub mod error;
pub mod events;
pub mod search;
pub mod ui;
pub mod vim;

use error::Result;

/// WASMモジュールのエントリポイント
/// DOMContentLoaded後に自動実行される
#[wasm_bindgen(start)]
pub fn main() -> std::result::Result<(), JsValue> {
    // パニック時のスタックトレースをコンソールに出力
    console_error_panic_hook_setup();

    // 初期化処理
    if let Err(e) = init_app() {
        web_sys::console::error_1(&format!("dnfolio WASM 初期化エラー: {e}").into());
        return Err(JsValue::from_str(&e.to_string()));
    }

    Ok(())
}

/// アプリケーション初期化
fn init_app() -> Result<()> {
    // バージョン情報をコンソールに出力
    web_sys::console::log_1(&"dnfolio WASM v0.1.0 initialized".into());

    // DOM要素の存在確認（デバッグ用）
    if let Ok(_main_content) = dom::query_selector::<web_sys::HtmlElement>(".main-content") {
        web_sys::console::log_1(&"  ✓ .main-content found".into());
    }

    if let Ok(_statusline) = dom::query_selector::<web_sys::HtmlElement>(".statusline") {
        web_sys::console::log_1(&"  ✓ .statusline found".into());
    }

    if let Ok(_commandline) = dom::query_selector::<web_sys::HtmlElement>(".commandline") {
        web_sys::console::log_1(&"  ✓ .commandline found".into());
    }

    // イベントハンドラーを登録
    events::setup_all_event_handlers()?;

    // 初期カーソルを設定
    setup_initial_cursor()?;

    // URLパラメータからハイライトを適用（ローディング制御付き）
    let has_highlight = apply_url_highlight()?;

    // URLフラグメントがあれば該当要素にスクロール
    scroll_to_url_fragment()?;

    // ハイライトがある場合は遅延してローディングを非表示
    // （スクロール・カーソル移動完了を待つ）
    if has_highlight {
        let callback = wasm_bindgen::closure::Closure::once(Box::new(|| {
            let _ = dom::hide_loading();
        }) as Box<dyn FnOnce()>);

        if let Some(window) = web_sys::window() {
            // 300ms待ってからローディングを非表示
            let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                callback.as_ref().unchecked_ref(),
                300,
            );
        }
        callback.forget();
    } else {
        // ハイライトがない場合は即座にローディングを非表示
        dom::hide_loading()?;
    }

    Ok(())
}

/// 初期カーソルを設定
fn setup_initial_cursor() -> Result<()> {
    // main-content内の最初のテキストノードにカーソルを設定
    if let Ok(main_content) = dom::query_selector::<web_sys::HtmlElement>(".main-content") {
        let walker = dom::TextNodeWalker::new(&main_content)?;
        let nodes = walker.collect_filtered()?;

        if let Some(first_node) = nodes.first() {
            let sel = dom::SelectionHelper::get()?;
            sel.collapse(first_node, 0)?;
            vim::cursor::update_block_cursor()?;
        }
    }
    Ok(())
}

/// URLパラメータからハイライトを適用
/// 戻り値: ハイライトが適用されたかどうか
///
/// セキュリティ対策:
/// - highlight パラメータの長さ制限（ReDoS対策、メモリ保護）
/// - lineNum パラメータの範囲制限（整数オーバーフロー対策）
fn apply_url_highlight() -> Result<bool> {
    // URLパラメータの最大長
    const MAX_HIGHLIGHT_PARAM_LEN: usize = 200;
    // 行番号の最大値（現実的な範囲）
    const MAX_LINE_NUM: usize = 10000;

    if let Some(window) = web_sys::window() {
        if let Ok(href) = window.location().href() {
            if let Ok(url) = web_sys::Url::new(&href) {
                let params = url.search_params();

                // highlight パラメータを取得
                if let Some(query) = params.get("highlight") {
                    // 長さ制限（攻撃的に長いクエリを拒否）
                    if query.len() > MAX_HIGHLIGHT_PARAM_LEN {
                        web_sys::console::warn_1(&"Highlight parameter too long, ignoring".into());
                        return Ok(false);
                    }

                    // lineNum パラメータを取得（範囲チェック付き）
                    let line_num = params
                        .get("lineNum")
                        .and_then(|s| s.parse::<usize>().ok())
                        .filter(|&n| n > 0 && n <= MAX_LINE_NUM);

                    if !query.is_empty() {
                        search::highlight::apply_highlight(&query, line_num)?;
                        web_sys::console::log_1(
                            &format!("  ✓ Highlight applied: {}", query).into(),
                        );
                        return Ok(true);
                    }
                }
            }
        }
    }
    Ok(false)
}

/// URLフラグメント（#heading-id）があれば該当要素にスクロール
///
/// セキュリティ対策:
/// - フラグメントIDの長さ制限
/// - 許可された文字のみ（英数字、ハイフン、アンダースコア、日本語）
fn scroll_to_url_fragment() -> Result<()> {
    // フラグメントIDの最大長
    const MAX_FRAGMENT_LEN: usize = 200;

    if let Some(window) = web_sys::window() {
        if let Ok(hash) = window.location().hash() {
            if !hash.is_empty() {
                let id = hash.trim_start_matches('#');

                // 長さ制限
                if id.len() > MAX_FRAGMENT_LEN {
                    web_sys::console::warn_1(&"Fragment ID too long, ignoring".into());
                    return Ok(());
                }

                // 文字種チェック（英数字、ハイフン、アンダースコア、日本語のみ許可）
                let is_valid = id
                    .chars()
                    .all(|c| c.is_alphanumeric() || c == '-' || c == '_');

                if !is_valid {
                    web_sys::console::warn_1(&"Invalid fragment ID characters, ignoring".into());
                    return Ok(());
                }

                if let Some(doc) = window.document() {
                    if let Some(target) = doc.get_element_by_id(id) {
                        // 少し遅延してスクロール（DOMが完全に構築されてから）
                        let callback = wasm_bindgen::closure::Closure::once(Box::new(move || {
                            target.scroll_into_view();
                        })
                            as Box<dyn FnOnce()>);
                        let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                            callback.as_ref().unchecked_ref(),
                            100,
                        );
                        callback.forget();
                    }
                }
            }
        }
    }
    Ok(())
}

/// パニックフックの設定
///
/// セキュリティ対策:
/// - リリースビルドでは詳細なエラー情報を隠蔽
/// - デバッグビルドでのみスタックトレースを出力
fn console_error_panic_hook_setup() {
    std::panic::set_hook(Box::new(|panic_info| {
        // デバッグビルドでは詳細情報を出力
        #[cfg(debug_assertions)]
        {
            let msg = panic_info.to_string();
            web_sys::console::error_1(&format!("WASM Panic: {msg}").into());

            // ロケーション情報も出力（デバッグ用）
            if let Some(location) = panic_info.location() {
                web_sys::console::error_1(
                    &format!(
                        "  at {}:{}:{}",
                        location.file(),
                        location.line(),
                        location.column()
                    )
                    .into(),
                );
            }
        }

        // リリースビルドでは最小限の情報のみ
        #[cfg(not(debug_assertions))]
        {
            web_sys::console::error_1(&"An internal error occurred".into());
        }
    }));
}
