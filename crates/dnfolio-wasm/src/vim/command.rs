//! Vimコマンド処理
//!
//! :q, :help, :tags等のコマンド実行

use web_sys::HtmlInputElement;

use crate::dom::{query_selector_optional, window};
use crate::error::Result;

/// コマンド実行結果
#[derive(Debug, Clone)]
pub struct CommandResult {
    /// トーストタイトル
    pub title: String,
    /// トーストメッセージ
    pub message: String,
    /// アイコン
    pub icon: String,
    /// タイプ（info, warn, error, success）
    pub toast_type: String,
}

impl CommandResult {
    #[must_use] 
    pub fn info(title: &str, message: &str, icon: &str) -> Self {
        Self {
            title: title.to_string(),
            message: message.to_string(),
            icon: icon.to_string(),
            toast_type: "info".to_string(),
        }
    }

    #[must_use] 
    pub fn warn(title: &str, message: &str, icon: &str) -> Self {
        Self {
            title: title.to_string(),
            message: message.to_string(),
            icon: icon.to_string(),
            toast_type: "warn".to_string(),
        }
    }
}

/// コマンド実行器
pub struct CommandExecutor;

impl CommandExecutor {
    /// コマンドを実行
    /// 戻り値: Some(CommandResult) = トースト表示、None = 特殊処理（ページ遷移等）
    pub fn execute(cmd: &str) -> Result<Option<CommandResult>> {
        let trimmed = cmd.trim();

        // 行番号ジャンプ（:123）
        if let Some(num_str) = trimmed.strip_prefix(':') {
            if let Ok(line_num) = num_str.parse::<usize>() {
                if line_num > 0 {
                    // 行ジャンプはJS側で処理（Phase 4以降で実装）
                    web_sys::console::log_1(&format!("Jump to line: {line_num}").into());
                    return Ok(None);
                }
            }
        }

        // コマンド定義
        let result = match trimmed {
            ":q" => Some(CommandResult::warn(
                "E37: No write since last change",
                "add ! to override (冗談です、ここはWebです)",
                "!",
            )),
            ":q!" => Some(CommandResult::warn(
                "E37: No write since last change",
                "...だから、Webサイトなんですって",
                "!",
            )),
            ":wq" => Some(CommandResult::info(
                "Already saved",
                "This is a static site. すでに保存済みです。",
                "💾",
            )),
            ":w" => Some(CommandResult::info(
                ":w",
                "Static siteなので保存する必要がありません",
                "📝",
            )),
            ":x" => Some(CommandResult::info(
                ":x",
                ":wqと同じですが、ここはWebです",
                "📝",
            )),
            ":help" | ":h" => Some(CommandResult::info(
                "Help - Keybindings",
                "/: ページ内検索\n:search: 全記事検索\ngg/G: トップ/ボトム\nn/N: 次/前ハイライト\n:noh :tags :privacy :sitemap",
                "❓",
            )),
            ":version" | ":ver" => {
                // HTMLのdata-version属性からバージョンを取得
                let version = Self::get_version();
                Some(CommandResult::info(
                    &format!("dnfolio {version}"),
                    "Built with Rust + WASM\nTheme: sakurajima.nvim\nby Daiki Nakashima",
                    "🦀",
                ))
            }
            ":smile" => Some(CommandResult::info(":)", "Have a nice day!", "😊")),
            ":qa" => Some(CommandResult::warn(
                "E37: No write since last change",
                "全部閉じようとしても無駄です",
                "!",
            )),
            ":qa!" => Some(CommandResult::info(
                "Bye!",
                "window.close()は動きませんけどね",
                "👋",
            )),
            ":set number" => Some(CommandResult::info(
                ":set number",
                "行番号は既に表示されています！",
                "🔢",
            )),
            ":set nonumber" => Some(CommandResult::info(
                ":set nonumber",
                "行番号を非表示にする機能はまだありません",
                "🔢",
            )),
            ":colorscheme" => Some(CommandResult::info(
                ":colorscheme",
                "現在: sakurajima.nvim (変更不可)",
                "🎨",
            )),
            ":noh" | ":nohlsearch" => {
                // ハイライト削除
                crate::search::highlight::remove_highlights()?;
                // URLパラメータからhighlightとlineNumを削除
                Self::clear_url_highlight_params()?;
                Self::clear_commandline()?;
                None
            }
            ":privacy" => {
                // ページ遷移
                if let Ok(win) = window() {
                    let _ = win.location().set_href("/privacy/");
                }
                None
            }
            ":sitemap" => {
                // 新しいタブで開く
                if let Ok(win) = window() {
                    let _ = win.open_with_url_and_target("/sitemap.xml", "_blank");
                }
                Some(CommandResult::info(
                    ":sitemap",
                    "sitemap.xmlを新しいタブで開きました",
                    "🗺️",
                ))
            }
            ":tags" => {
                // タグモーダルを開く
                crate::events::open_tags_modal()?;
                None
            }
            ":search" | ":grep" => {
                // 検索モーダルを開く（全記事検索）
                crate::events::open_search_modal()?;
                None
            }
            ":$" => {
                // 最終行へジャンプ（Phase 4で実装）
                web_sys::console::log_1(&"Jump to last line".into());
                None
            }
            _ => {
                // 未知のコマンド
                if trimmed.starts_with(':') {
                    Some(CommandResult::warn(
                        "E492: Not an editor command",
                        trimmed,
                        "!",
                    ))
                } else {
                    None
                }
            }
        };

        Ok(result)
    }

    /// コマンドラインをクリア
    fn clear_commandline() -> Result<()> {
        if let Ok(Some(input)) = query_selector_optional::<HtmlInputElement>("#commandline-input") {
            input.set_value("");
            input.set_attribute("readonly", "").ok();
        }
        Ok(())
    }

    /// HTMLのdata-version属性からバージョンを取得
    fn get_version() -> String {
        if let Ok(win) = window() {
            if let Some(doc) = win.document() {
                if let Some(body) = doc.body() {
                    if let Some(version) = body.get_attribute("data-version") {
                        return version;
                    }
                }
            }
        }
        "dev".to_string()
    }

    /// `URLパラメータからhighlightとlineNumを削除`
    fn clear_url_highlight_params() -> Result<()> {
        if let Ok(win) = window() {
            if let Ok(href) = win.location().href() {
                if let Ok(url) = web_sys::Url::new(&href) {
                    let params = url.search_params();

                    // highlightとlineNumパラメータがあれば削除
                    if params.has("highlight") || params.has("lineNum") {
                        params.delete("highlight");
                        params.delete("lineNum");

                        // 現在のスクロール位置を保存
                        let scroll_y = win.scroll_y().unwrap_or(0.0);

                        // ハッシュ（フラグメント）を保持
                        let hash = url.hash();

                        // 新しいURLを構築
                        let params_str = params.to_string().as_string().unwrap_or_default();
                        let new_url = if params_str.is_empty() {
                            // パラメータがなくなった場合
                            format!("{}{}{}", url.origin(), url.pathname(), hash)
                        } else {
                            format!("{}{}?{}{}", url.origin(), url.pathname(), params_str, hash)
                        };

                        // history.replaceStateでURL更新（リロードなし）
                        if let Ok(history) = win.history() {
                            let _ = history.replace_state_with_url(
                                &wasm_bindgen::JsValue::NULL,
                                "",
                                Some(&new_url),
                            );
                        }

                        // スクロール位置を復元
                        win.scroll_to_with_x_and_y(0.0, scroll_y);
                    }
                }
            }
        }
        Ok(())
    }
}
