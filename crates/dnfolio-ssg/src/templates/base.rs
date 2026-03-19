use crate::models::MetaData;
use crate::templates::base_stylesheet::BASE_STYLESHEET;
use crate::templates::icons;
use css_minify::optimizations::{Level, Minifier};
use maud::{DOCTYPE, Markup, PreEscaped, html};

/// 歯車ロゴSVG（ローディング表示用）
const RUST_GEAR_SVG: &str = r##"<svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
    <defs>
        <linearGradient id="rustGradient" x1="0%" y1="0%" x2="100%" y2="100%">
            <stop offset="0%" style="stop-color:#f74c00"/>
            <stop offset="100%" style="stop-color:#ce412b"/>
        </linearGradient>
    </defs>
    <!-- 外側の歯車 -->
    <path fill="url(#rustGradient)" d="
        M50 5
        L54 5 L56 12 L60 11 L65 6 L69 8 L67 15 L71 17 L78 14 L80 18 L75 23 L78 27 L86 26 L87 31 L80 34 L81 38 L89 40 L89 45 L81 46 L80 50
        L81 54 L89 55 L89 60 L81 62 L80 66 L87 69 L86 74 L78 73 L75 77 L80 82 L78 86 L71 83 L67 85 L69 92 L65 94 L60 89 L56 88 L54 95 L50 95
        L46 95 L44 88 L40 89 L35 94 L31 92 L33 85 L29 83 L22 86 L20 82 L25 77 L22 73 L14 74 L13 69 L20 66 L19 62 L11 60 L11 55 L19 54 L20 50
        L19 46 L11 45 L11 40 L19 38 L20 34 L13 31 L14 26 L22 27 L25 23 L20 18 L22 14 L29 17 L33 15 L31 8 L35 6 L40 11 L44 12 L46 5 Z
    "/>
    <!-- 中央の穴 -->
    <circle cx="50" cy="50" r="20" fill="#1a1b26"/>
    <!-- 中央のD -->
    <text x="50" y="58" text-anchor="middle" fill="url(#rustGradient)" font-family="monospace" font-size="24" font-weight="bold">D</text>
</svg>"##;

pub struct PageConfig<'a> {
    pub page_title: &'a str,
    pub canonical_url: &'a str,
    pub metadata: Option<&'a MetaData>,
    pub ogp_image_path: Option<&'a str>,
    pub structured_data_html: Option<&'a str>,
    pub robots_directive: Option<&'a str>,
    pub article_dates: Option<(&'a str, &'a str)>,
}

// 記事ページで使用する拡張設定
pub struct ArticlePageConfig<'a> {
    pub base: PageConfig<'a>,
    pub toc_html: Option<&'a str>,
}

pub fn minified_stylesheet() -> String {
    Minifier::default()
        .minify(BASE_STYLESHEET, Level::Two)
        .unwrap_or_else(|_| BASE_STYLESHEET.to_string())
}

pub fn layout(
    config: PageConfig,
    sidebar_left_markup: Markup,
    main_content_markup: Markup,
    _sidebar_right_markup: Markup, // Neovim風UIでは右サイドバー廃止
) -> Markup {
    layout_with_toc(
        ArticlePageConfig {
            base: config,
            toc_html: None,
        },
        sidebar_left_markup,
        main_content_markup,
    )
}

pub fn layout_with_toc(
    config: ArticlePageConfig,
    sidebar_left_markup: Markup,
    main_content_markup: Markup,
) -> Markup {
    let description = config
        .base
        .metadata
        .and_then(|m| m.description.as_ref())
        .map(|d| d.as_str())
        .unwrap_or("プログラムを良く書く人の個人サイトです。");

    let keywords = config
        .base
        .metadata
        .and_then(|m| m.taxonomies.as_ref())
        .and_then(|t| t.tags.as_ref())
        .map(|tags| tags.join(", "))
        .unwrap_or_else(|| "ポエム".to_string());

    // ファイルタイプの判定（記事ページかホームか）
    let file_type = if config.base.canonical_url.contains("/posts/") {
        "markdown"
    } else {
        "home"
    };

    // git tagからバージョンを取得（ビルド時に埋め込み）
    const GIT_VERSION: &str = env!("GIT_VERSION");

    // タブのファイル名を生成
    let tab_filename = if config.base.canonical_url.contains("/posts/") {
        format!(
            "{}.md",
            config.base.page_title.chars().take(30).collect::<String>()
        )
    } else if config.base.canonical_url == "https://dnfolio.me/" {
        "index.md".to_string()
    } else {
        format!("{}.md", config.base.page_title)
    };

    html! {
        (DOCTYPE)
        html lang="ja" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                meta name="theme-color" content="#22272e";

                // Content Security Policy
                // XSS攻撃からの保護を強化
                // 注意: frame-ancestorsはmeta要素では無効（HTTPヘッダーでのみ有効）
                // 注意: 'unsafe-inline'はGoogle Analytics等のインラインスクリプトに必要
                //       SSGではnonce/hashを動的生成できないため必要
                meta http-equiv="Content-Security-Policy" content="default-src 'self'; script-src 'self' 'unsafe-inline' 'wasm-unsafe-eval' https://www.googletagmanager.com https://www.google-analytics.com https://static.cloudflareinsights.com https://bst.heion.net https://blueskytimeline.com; style-src 'self' 'unsafe-inline'; font-src 'self'; img-src 'self' data: https:; connect-src 'self' https://www.google-analytics.com https://region1.google-analytics.com https://cloudflareinsights.com https://bst.heion.net https://blueskytimeline.com; base-uri 'self'; form-action 'self';";

                // Referrer Policy - 外部サイトにはオリジンのみ送信
                meta name="referrer" content="strict-origin-when-cross-origin";

                title { (config.base.page_title) }
                link rel="canonical" href=(config.base.canonical_url);
                meta name="description" content=(description);
                meta name="keywords" content=(keywords);
                meta name="author" content="Daiki Nakashima";


                @if let Some(robots_directive) = config.base.robots_directive {
                    meta name="robots" content=(robots_directive);
                }

                meta property="og:title" content=(config.base.page_title);
                meta property="og:description" content=(description);
                meta property="og:type" content=(if config.base.article_dates.is_some() { "article" } else { "website" });
                meta property="og:site_name" content="dnfolio";
                meta property="og:url" content=(config.base.canonical_url);
                @if let Some((published_time, modified_time)) = config.base.article_dates {
                    meta property="article:published_time" content=(published_time);
                    meta property="article:modified_time" content=(modified_time);
                }
                @if let Some(image_path) = config.base.ogp_image_path {
                    meta property="og:image" content=(format!("https://dnfolio.me{image_path}"));
                    meta property="og:image:width" content="1200";
                    meta property="og:image:height" content="630";
                    meta property="og:image:type" content="image/png";
                } @else {
                    meta property="og:image" content=(format!("https://dnfolio.me/icons/icon.png"));
                    meta property="og:image:width" content="1200";
                    meta property="og:image:height" content="630";
                    meta property="og:image:type" content="image/png";
                }

                meta name="twitter:card" content="summary_large_image";
                meta name="twitter:title" content=(config.base.page_title);
                meta name="twitter:description" content=(description);
                meta name="twitter:site" content="@dnfolio_me";
                @if let Some(image_path) = config.base.ogp_image_path {
                    meta name="twitter:image" content=(format!("https://dnfolio.me{image_path}"));
                } @else {
                    meta name="twitter:image" content=(format!("https://dnfolio.me/icons/icon.png"));
                }

                link rel="shortcut icon" href="/icons/favicon.ico" type="image/x-icon";
                link rel="apple-touch-icon" href="/icons/icon.png";
                link rel="alternate" type="application/rss+xml" title="dnfolio" href="/feed.xml";

                @if let Some(json_ld) = config.base.structured_data_html {
                    (PreEscaped(json_ld))
                }

                link rel="stylesheet" href="/app.css";
            }
            body data-version=(GIT_VERSION) {
                // ローディングオーバーレイ（Rust歯車）- 初期表示、WASM初期化完了後に非表示
                div id="loading-overlay" class="loading-overlay" {
                    div class="loading-spinner" {
                        (PreEscaped(RUST_GEAR_SVG))
                    }
                    span class="loading-text" { "Loading..." }
                }

                // タイトルバー（Neovim風）
                header class="titlebar" {
                    div class="titlebar-left" {
                        button id="hamburger-btn" class="hamburger-btn" aria-label="Menu" {
                            span {}
                            span {}
                            span {}
                        }
                        h1 {
                            a href="/" { "dnfolio" }
                        }
                    }
                    div class="window-buttons" {
                        span class="window-button btn-minimize" {}
                        span class="window-button btn-maximize" {}
                        span class="window-button btn-close" {}
                    }
                }

                div id="overlay" class="overlay" {}

                div class="container" {
                    // 左サイドバー（EXPLORER）
                    aside id="sidebar-left" class="sidebar-left" {
                        div class="sidebar-header" {
                            h2 { "EXPLORER" }
                            button id="sidebar-close-btn" class="sidebar-close-btn" aria-label="Close sidebar" {
                                (PreEscaped(icons::arrow_right(16)))
                            }
                        }
                        div class="sidebar-content" {
                            // 目次（記事ページの場合）- トップに配置
                            @if let Some(toc) = config.toc_html {
                                div class="toc-section" {
                                    (PreEscaped(toc))
                                }
                            }
                            (sidebar_left_markup)
                        }
                    }

                    // メインエリア
                    div class="main-area" {
                        // タブバー
                        div class="tab-bar" {
                            div class="tab active" {
                                span class="tab-icon" { (PreEscaped(icons::file_markdown(14))) }
                                span { (tab_filename) }
                                span class="tab-close" { "×" }
                            }
                        }

                        // エディターコンテンツ
                        div class="editor-wrapper" {
                            // 行番号は非表示（必要に応じてJSで動的に追加可能）
                            main class=(format!("main-content {}", if file_type == "markdown" { "page-article" } else { "page-home" })) tabindex="-1" {
                                (main_content_markup)
                            }
                        }
                    }
                }

                // トースト通知コンテナ
                div id="toast-container" class="toast-container" {}

                // モバイル用ハイライトナビゲーション
                div id="highlight-nav" class="highlight-nav" {
                    button id="highlight-nav-prev" class="highlight-nav-btn" title="前へ (N)" { "↑" }
                    button id="highlight-nav-next" class="highlight-nav-btn" title="次へ (n)" { "↓" }
                }

                // 固定フッター（ステータスライン + コマンドライン）
                div class="footer-fixed" {
                    // ステータスライン（左右分離）
                    footer class="statusline" {
                        div class="statusline-left" {
                            span class="statusline-mode" { "NORMAL" }
                            // ペインインジケーター（EXPLORERフォーカス時に表示）
                            span class="statusline-section statusline-pane" id="pane-indicator" {}
                            // ペンディングキー（Ctrl+w等の2キーコマンド用）
                            span class="statusline-section statusline-pending" id="pending-key" {}
                            span class="statusline-section statusline-branch" { "main" }
                            a href="/privacy/" class="statusline-section statusline-privacy" title="Privacy Policy" {
                                (PreEscaped(icons::shield(14)))
                            }
                        }
                        div class="statusline-right" {
                            span class="statusline-section search-count" id="search-count" {}
                            span class="statusline-section statusline-encoding" { "UTF-8" }
                            span class="statusline-section statusline-filetype" { (file_type) }
                            span class="statusline-section statusline-position" id="scroll-position" { "Top" }
                        }
                    }

                    // コマンドライン（検索トリガー + ハイライト管理）
                    div class="commandline" {
                        span class="commandline-prefix" { "/" }
                        input type="text" id="commandline-input" placeholder="Type :command or /pattern (:help for list)" autocomplete="off" readonly;
                        span class="commandline-cursor" { "_" }
                    }
                }

                // モバイル用コマンドパレット（ボトムシート）
                div id="bottomsheet-overlay" class="bottomsheet-overlay" {}
                div id="command-bottomsheet" class="command-bottomsheet" {
                    div class="bottomsheet-handle" {
                        span class="bottomsheet-handle-bar" {}
                    }
                    div class="bottomsheet-header" {
                        span class="bottomsheet-title" { "Command Palette" }
                    }
                    div class="bottomsheet-presets" {
                        button class="bottomsheet-preset-btn" data-command=":search" {
                            span class="preset-icon" { (PreEscaped(icons::search(16))) }
                            span class="preset-cmd" { ":search" }
                            span class="preset-desc" { "記事をGrep検索" }
                        }
                        button class="bottomsheet-preset-btn" data-command=":tags" {
                            span class="preset-icon" { (PreEscaped(icons::tag(16))) }
                            span class="preset-cmd" { ":tags" }
                            span class="preset-desc" { "タグで絞り込み" }
                        }
                        button class="bottomsheet-preset-btn" data-command="/search" {
                            span class="preset-icon" { (PreEscaped(icons::text_search(16))) }
                            span class="preset-cmd" { "/pattern" }
                            span class="preset-desc" { "ページ内検索" }
                        }
                        button class="bottomsheet-preset-btn" data-command=":help" {
                            span class="preset-icon" { (PreEscaped(icons::help_circle(16))) }
                            span class="preset-cmd" { ":help" }
                            span class="preset-desc" { "キーバインド一覧" }
                        }
                        button class="bottomsheet-preset-btn" data-command=":version" {
                            span class="preset-icon" { (PreEscaped(icons::info(16))) }
                            span class="preset-cmd" { ":version" }
                            span class="preset-desc" { "バージョン情報" }
                        }
                        button class="bottomsheet-preset-btn" data-command=":noh" {
                            span class="preset-icon" { (PreEscaped(icons::x_circle(16))) }
                            span class="preset-cmd" { ":noh" }
                            span class="preset-desc" { "検索ハイライトを消す" }
                        }
                        button class="bottomsheet-preset-btn" data-command=":smile" {
                            span class="preset-icon" { (PreEscaped(icons::smile(16))) }
                            span class="preset-cmd" { ":smile" }
                            span class="preset-desc" { "イースターエッグ" }
                        }
                    }
                }

                // 検索結果（旧、後方互換用）
                div id="search-results" {}

                // snacks.nvim grep風検索モーダル
                div id="search-modal" class="search-modal" {
                    div class="search-modal-container" {
                        div class="search-modal-header" {
                            span class="search-modal-title" {
                                "󰍉 Grep Articles"
                            }
                            button id="search-modal-close" class="search-modal-close" { "×" }
                        }
                        div class="search-input-wrapper" {
                            input type="text" id="grep-search-input" placeholder="/pattern..." autocomplete="off";
                        }
                        div class="search-modal-body" {
                            div class="search-results-pane" {
                                div id="grep-results-list" class="search-results-list" {}
                            }
                            div class="search-preview-pane" {
                                div id="grep-preview" {}
                            }
                        }
                        div class="search-modal-footer" {
                            div class="search-modal-footer-left" {
                                span id="search-mode-indicator" class="search-mode-indicator mode-insert" { "INSERT" }
                                span id="grep-results-count" { "0 results" }
                            }
                            span {
                                kbd { "j/k" } "移動  "
                                kbd { "Enter" } "選択  "
                                kbd { "i/a" } "INSERT  "
                                kbd { "Esc" } "NORMAL→閉じる"
                            }
                        }
                    }
                }

                // タグ一覧モーダル（検索モーダルと同じ構造）
                div id="tags-modal" class="tags-modal" {
                    div class="tags-modal-container" {
                        div class="tags-modal-header" {
                            span class="tags-modal-title" {
                                (PreEscaped(icons::hash(14)))
                                " Tags"
                            }
                            button id="tags-modal-close" class="tags-modal-close" { "×" }
                        }
                        div class="tags-input-wrapper" {
                            input type="text" id="tags-filter-input" placeholder="Filter tags..." autocomplete="off";
                        }
                        div class="tags-modal-body" {
                            div class="tags-list-pane" {
                                div id="tags-list" class="tags-list" {}
                            }
                            div class="tags-preview-pane" {
                                div id="tags-preview" {}
                            }
                        }
                        div class="tags-modal-footer" {
                            div class="tags-modal-footer-left" {
                                span id="tags-mode-indicator" class="tags-mode-indicator mode-insert" { "INSERT" }
                                span id="tags-count" { "0 tags" }
                            }
                            span {
                                kbd { "j/k" } "移動  "
                                kbd { "Enter" } "選択  "
                                kbd { "Esc" } "モード切替/閉じる"
                            }
                        }
                    }
                }

                // WASM モジュールロード（JSは100% Rustに移行）
                script type="module" {
                    (PreEscaped(r#"
                        import init from '/dnfolio_wasm.js';
                        init();
                    "#))
                }

                script {
                    (PreEscaped(r#"
                        (window.requestIdleCallback || function(cb) { setTimeout(cb, 2000); })(function() {
                            var s = document.createElement('script');
                            s.src = 'https://www.googletagmanager.com/gtag/js?id=G-S0DTM6WBVT';
                            s.async = true;
                            document.head.appendChild(s);
                            window.dataLayer = window.dataLayer || [];
                            function gtag(){dataLayer.push(arguments);}
                            gtag('js', new Date());
                            gtag('config', 'G-S0DTM6WBVT');
                        });
                    "#))
                }

                // Cloudflare Analytics（環境変数CF_ANALYTICS_TOKENが設定されている場合のみ有効）
                @if let Some(cf_token) = option_env!("CF_ANALYTICS_TOKEN") {
                    script defer src="https://static.cloudflareinsights.com/beacon.min.js" data-cf-beacon=(format!(r#"{{"token": "{}"}}"#, cf_token)) {}
                }
            }
        }
    }
}
