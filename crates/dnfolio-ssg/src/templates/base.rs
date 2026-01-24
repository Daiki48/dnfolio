use crate::models::MetaData;
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
}

// 記事ページで使用する拡張設定
pub struct ArticlePageConfig<'a> {
    pub base: PageConfig<'a>,
    pub toc_html: Option<&'a str>,
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

    let css = r#"
        @font-face {
            font-family: 'UDEV Gothic';
            font-style: normal;
            font-weight: 400;
            font-display: swap;
            src: url('/fonts/UDEVGothic-Regular.woff2') format('woff2');
        }
        @font-face {
            font-family: 'UDEV Gothic';
            font-style: normal;
            font-weight: 700;
            font-display: swap;
            src: url('/fonts/UDEVGothic-Bold.woff2') format('woff2');
        }

        /* ========================================
           sakurajima.nvim カラーパレット
           ======================================== */
        :root {
            /* 背景系 */
            --bg-primary: #22272e;
            --bg-secondary: #2D333B;
            --bg-elevated: #1D3A64;

            /* テキスト系 */
            --text-primary: #8b9aaa;
            --text-secondary: #7c6f64;
            --text-bright: #ebdbb2;
            --text-muted: #7D7D7D;

            /* アクセント（シアン系） */
            --accent-cyan: #3B7B7D;
            --accent-cyan-light: #5F9D9C;
            --accent-cyan-bright: #2BB6BA;

            /* アクセント（イエロー系） */
            --accent-yellow: #B3AF78;
            --accent-yellow-bright: #E3D92C;

            /* アクセント（グリーン系） */
            --accent-green: #658D50;
            --accent-green-bright: #3DA163;

            /* アクセント（オレンジ系） */
            --accent-orange: #97812C;
            --accent-orange-bright: #E38D2C;

            /* アクセント（ブルー系） */
            --accent-blue: #4F6981;
            --accent-blue-light: #82ade0;

            /* アクセント（レッド系） */
            --accent-red: #A77169;
            --accent-red-bright: #E34C36;

            /* ステータス */
            --status-error: #8f3231;
            --status-warn: #C7A252;
            --status-info: #CEB4A8;
            --status-hint: #717375;

            /* ボーダー */
            --border-color: #3D4450;

            /* レイアウト */
            --titlebar-height: 40px;
            --statusline-height: 28px;
            --commandline-height: 32px;
            --sidebar-width: 400px;

            /* フォント */
            --font-mono: 'UDEV Gothic', monospace;
            --font-body: 'UDEV Gothic', monospace;
            --font-code: 'UDEV Gothic', monospace;
        }

        /* ========================================
           ベーススタイル
           ======================================== */
        *, *::before, *::after { box-sizing: border-box; }

        html {
            font-size: 16px;
            scroll-padding-top: var(--titlebar-height);
            scrollbar-width: thin;
            scrollbar-color: var(--accent-cyan) var(--bg-secondary);
        }

        body {
            font-family: var(--font-body);
            margin: 0;
            display: flex;
            flex-direction: column;
            min-height: 100vh;
            line-height: 1.7;
            background-color: var(--bg-primary);
            color: var(--text-primary);
            overflow-x: hidden;
            /* 固定フッター分のpadding */
            padding-bottom: calc(var(--statusline-height) + var(--commandline-height));
        }

        /* ========================================
           ローディングオーバーレイ
           ======================================== */
        .loading-overlay {
            position: fixed;
            top: 0;
            left: 0;
            width: 100%;
            height: 100%;
            background: var(--bg-primary);
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            z-index: 10000;
            opacity: 0;
            visibility: hidden;
            transition: opacity 0.2s ease;
        }

        .loading-overlay.visible {
            opacity: 1;
            visibility: visible;
        }

        .loading-spinner {
            width: 80px;
            height: 80px;
            aspect-ratio: 1;
            animation: spin 1.5s linear infinite;
        }

        .loading-spinner svg {
            width: 100%;
            height: 100%;
            aspect-ratio: 1;
        }

        .loading-text {
            margin-top: 16px;
            font-family: var(--font-mono);
            font-size: 0.9rem;
            color: var(--accent-orange);
        }

        @keyframes spin {
            from { transform: rotate(0deg); }
            to { transform: rotate(360deg); }
        }

        /* ========================================
           タイトルバー（Neovim風ウィンドウ）
           ======================================== */
        .titlebar {
            background: linear-gradient(180deg, var(--bg-secondary) 0%, var(--bg-primary) 100%);
            border-bottom: 1px solid var(--border-color);
            height: var(--titlebar-height);
            display: flex;
            align-items: center;
            justify-content: space-between;
            padding: 0 12px;
            position: sticky;
            top: 0;
            z-index: 900;
        }

        .titlebar-left {
            display: flex;
            align-items: center;
            gap: 12px;
        }

        .titlebar h1 {
            font-family: var(--font-mono);
            font-size: 0.95rem;
            font-weight: 600;
            margin: 0;
            color: var(--text-bright);
        }

        .titlebar h1 a {
            color: inherit;
            text-decoration: none;
        }

        .titlebar h1 a:hover {
            color: var(--accent-cyan-bright);
        }

        /* Linux Mint Cinnamon (Mint-Y) 風ウィンドウボタン */
        .window-buttons {
            display: flex;
            gap: 4px;
        }

        .window-button {
            width: 28px;
            height: 28px;
            border-radius: 4px;
            border: none;
            cursor: default;
            background: transparent;
            display: flex;
            align-items: center;
            justify-content: center;
            font-family: var(--font-mono);
            font-size: 14px;
            font-weight: 400;
            color: var(--text-muted);
            transition: background 0.15s ease, color 0.15s ease;
        }

        .window-button:hover {
            background: var(--bg-elevated);
            color: var(--text-bright);
        }

        .btn-close:hover {
            background: var(--accent-red-bright);
            color: #fff;
        }

        .btn-minimize::before { content: "\2212"; }
        .btn-maximize::before { content: "\25A1"; }
        .btn-close::before { content: "\00D7"; }

        /* ========================================
           メインコンテナ（2カラム）
           ======================================== */
        .container {
            display: flex;
            flex: 1;
            align-items: stretch;
            width: 100%;
        }

        /* ========================================
           左サイドバー（EXPLORER）
           ======================================== */
        .sidebar-left {
            flex: 0 0 var(--sidebar-width);
            background-color: var(--bg-secondary);
            border-right: 1px solid var(--border-color);
            display: flex;
            flex-direction: column;
            position: sticky;
            top: var(--titlebar-height);
            height: calc(100vh - var(--titlebar-height) - var(--statusline-height) - var(--commandline-height));
            overflow: hidden;
        }

        .sidebar-header {
            padding: 8px 12px;
            border-bottom: 1px solid var(--border-color);
            position: relative;
        }

        .sidebar-header h2 {
            color: var(--text-muted);
            font-family: var(--font-mono);
            font-size: 0.7rem;
            font-weight: 600;
            text-transform: uppercase;
            letter-spacing: 0.15em;
            margin: 0;
        }

        .sidebar-content {
            flex: 1;
            overflow-y: auto;
            padding: 8px 0;
        }

        .sidebar-left ul {
            list-style: none;
            padding: 0;
            margin: 0;
        }

        .sidebar-left li {
            margin: 0;
        }

        /* ファイルツリーアイテム */
        .file-tree-item {
            display: flex;
            align-items: center;
            color: var(--text-primary);
            padding: 4px 12px;
            cursor: pointer;
            font-family: var(--font-mono);
            font-size: 0.85rem;
            text-decoration: none;
            transition: background 0.15s ease;
        }

        .file-tree-item:hover {
            background: var(--bg-primary);
            text-decoration: none;
        }

        .file-tree-item.active,
        .file-tree-item.current {
            background: linear-gradient(90deg, var(--bg-secondary) 0%, var(--bg-elevated) 100%);
            border-left: 2px solid var(--accent-cyan);
            color: var(--text-bright);
        }

        .tree-icon {
            color: var(--text-muted);
            margin-right: 6px;
            width: 12px;
            height: 12px;
            display: inline-flex;
            align-items: center;
            justify-content: center;
            flex-shrink: 0;
        }

        .tree-icon svg {
            width: 12px;
            height: 12px;
        }

        .tree-icon-folder {
            color: var(--accent-yellow);
        }

        .tree-icon-file {
            color: var(--accent-cyan-light);
        }

        .current .tree-icon-file {
            color: var(--accent-green-bright);
        }

        /* 年/月のフォルダ */
        .folder-year,
        .folder-month {
            color: var(--accent-yellow);
        }

        /* 目次アイテム（旧スタイル - ファイルツリー内用） */
        .toc-expanded .toc-item {
            color: var(--text-muted);
            padding: 2px 12px 2px 32px;
            font-size: 0.8rem;
            font-family: var(--font-mono);
            text-decoration: none;
            display: block;
            transition: color 0.15s ease;
        }

        .toc-expanded .toc-item:hover {
            color: var(--text-primary);
            text-decoration: none;
        }

        .toc-expanded .toc-item::before {
            content: "+-";
            color: var(--text-muted);
            margin-right: 6px;
        }

        /* ========================================
           メインエリア（Editor）
           ======================================== */
        .main-area {
            flex: 1;
            display: flex;
            flex-direction: column;
            min-width: 0;
            background-color: var(--bg-primary);
        }

        /* タブバー */
        .tab-bar {
            background: var(--bg-secondary);
            border-bottom: 1px solid var(--border-color);
            display: flex;
            align-items: stretch;
            height: 36px;
        }

        .tab {
            background: var(--bg-primary);
            color: var(--text-primary);
            border-right: 1px solid var(--border-color);
            padding: 0 16px;
            display: flex;
            align-items: center;
            gap: 8px;
            font-family: var(--font-mono);
            font-size: 0.8rem;
            border-top: 2px solid transparent;
        }

        .tab.active {
            border-top-color: var(--accent-cyan);
            color: var(--text-bright);
        }

        .tab-icon {
            color: var(--accent-blue-light);
        }

        .tab-close {
            color: var(--text-muted);
            font-size: 0.7rem;
            margin-left: 4px;
        }

        /* エディターコンテンツ */
        .editor-wrapper {
            display: flex;
            flex: 1;
            overflow: hidden;
        }

        .line-numbers {
            flex: 0 0 50px;
            background: var(--bg-primary);
            border-right: 1px solid var(--border-color);
            text-align: right;
            padding: 1em 8px 1em 0;
            color: var(--text-secondary);
            font-family: var(--font-mono);
            font-size: 0.85rem;
            line-height: 1.7;
            user-select: none;
            overflow-y: auto;
        }

        .line-numbers span {
            display: block;
        }

        .main-content {
            flex: 1;
            padding: 1em 1.5em 1em 4em;
            overflow-y: auto;
            overflow-wrap: break-word;
            position: relative;
            counter-reset: line-number;
        }

        /* カーソル設定 */
        .main-content {
            caret-color: transparent; /* ネイティブキャレットを隠す */
            outline: none;
            cursor: text;
        }

        /* Vimブロックカーソル */
        .vim-cursor {
            background: var(--accent-cyan-bright);
            color: var(--bg-primary);
            animation: cursor-blink 1s step-end infinite;
        }

        @keyframes cursor-blink {
            0%, 100% { opacity: 1; }
            50% { opacity: 0.5; }
        }

        /* ビジュアルモード時のカーソル */
        .visual-mode .vim-cursor {
            background: var(--accent-yellow);
        }

        /* ノーマルモード時の選択色 */
        .main-content::selection {
            background: var(--accent-cyan);
            color: var(--bg-primary);
        }

        /* ビジュアルモード選択色 */
        .main-content.visual-mode::selection {
            background: var(--accent-yellow);
            color: var(--bg-primary);
        }

        /* ステータスラインのモード表示色 */
        .statusline-mode.mode-normal {
            background: var(--accent-blue-light);
            color: var(--bg-primary);
        }

        .statusline-mode.mode-visual {
            background: var(--accent-yellow);
            color: var(--bg-primary);
        }

        .statusline-mode.mode-visual-line {
            background: var(--accent-orange-bright);
            color: var(--bg-primary);
        }

        .statusline-mode.mode-insert {
            background: var(--accent-green-bright);
            color: var(--bg-primary);
        }

        /* 行番号付きブロック要素 */
        .main-content > h1,
        .main-content > h2,
        .main-content > h3,
        .main-content > h4,
        .main-content > p,
        .main-content > ul,
        .main-content > ol,
        .main-content > blockquote,
        .main-content > pre,
        .main-content > table,
        .main-content > hr,
        .main-content > .code-block-wrapper {
            position: relative;
            counter-increment: line-number;
        }

        .main-content > h1::before,
        .main-content > h2::before,
        .main-content > h3::before,
        .main-content > h4::before,
        .main-content > p::before,
        .main-content > ul::before,
        .main-content > ol::before,
        .main-content > blockquote::before,
        .main-content > pre::before,
        .main-content > table::before,
        .main-content > hr::before,
        .main-content > .code-block-wrapper::before {
            content: counter(line-number);
            position: absolute;
            left: -3.5em;
            top: 0;
            width: 2.5em;
            height: 1.7em;
            line-height: 1.7;
            text-align: right;
            color: var(--text-secondary);
            font-family: var(--font-mono);
            font-size: 0.85rem;
            user-select: none;
            cursor: pointer;
            transition: color 0.15s ease;
        }

        /* 見出しは高さに合わせて中央配置 */
        .main-content > h1::before,
        .main-content > h2::before,
        .main-content > h3::before,
        .main-content > h4::before {
            top: 50%;
            transform: translateY(-50%);
            height: auto;
            line-height: 1;
        }

        .main-content > h1:hover::before,
        .main-content > h2:hover::before,
        .main-content > h3:hover::before,
        .main-content > h4:hover::before,
        .main-content > p:hover::before,
        .main-content > ul:hover::before,
        .main-content > ol:hover::before,
        .main-content > blockquote:hover::before,
        .main-content > pre:hover::before,
        .main-content > table:hover::before,
        .main-content > hr:hover::before,
        .main-content > .code-block-wrapper:hover::before {
            color: var(--accent-cyan);
        }

        /* 現在の行（cursorline風） - 要素を移動させずに背景のみ */
        .main-content > .current-line {
            background: linear-gradient(90deg, rgba(59, 123, 125, 0.2) 0%, rgba(59, 123, 125, 0.05) 50%, transparent 100%);
            border-radius: 2px;
        }

        /* current-lineの行番号スタイル（すべての要素に対応） */
        .main-content > .current-line::before,
        .main-content > h1.current-line::before,
        .main-content > h2.current-line::before,
        .main-content > h3.current-line::before,
        .main-content > h4.current-line::before,
        .main-content > p.current-line::before,
        .main-content > ul.current-line::before,
        .main-content > ol.current-line::before,
        .main-content > blockquote.current-line::before,
        .main-content > pre.current-line::before,
        .main-content > table.current-line::before,
        .main-content > .code-block-wrapper.current-line::before {
            color: #2BB6BA !important;
            font-weight: 700 !important;
            text-shadow: 0 0 8px rgba(43, 182, 186, 0.5) !important;
        }

        /* ヘッダーアンカーリンク - 記事ページのh2以降のみパディング追加 */
        .main-content.page-article > h2,
        .main-content.page-article > h3,
        .main-content.page-article > h4 {
            padding-left: 1.2em;
            position: relative;
        }

        .header-anchor-link {
            color: var(--accent-cyan);
            text-decoration: none;
            font-weight: normal;
            font-family: var(--font-mono);
            font-size: 0.85em;
            opacity: 0.4;
            transition: opacity 0.15s ease, color 0.15s ease;
            position: absolute;
            left: 0;
            top: 50%;
            transform: translateY(-50%);
            cursor: pointer;
            z-index: 10;
            padding: 0.2em 0.3em;
        }

        /* h1のアンカーは非表示（ページタイトルなので不要） */
        .main-content > h1 .header-anchor-link {
            display: none;
        }

        .main-content > h2:hover .header-anchor-link,
        .main-content > h3:hover .header-anchor-link,
        .main-content > h4:hover .header-anchor-link {
            opacity: 0.8;
        }

        .header-anchor-link:hover {
            color: var(--accent-cyan-bright);
            opacity: 1;
        }

        .main-content img {
            max-width: 100%;
            height: auto;
            border-radius: 4px;
            border: 1px solid var(--border-color);
        }

        /* ========================================
           タイポグラフィ
           ======================================== */
        h1, h2, h3, h4, h5, h6 {
            font-family: var(--font-mono);
            color: var(--accent-yellow);
            margin-top: 1.5em;
            margin-bottom: 0.5em;
            line-height: 1.3;
        }

        /* 見出しマーカーはdata属性で表現 */
        h1[data-marker]::before,
        h2[data-marker]::before,
        h3[data-marker]::before,
        h4[data-marker]::before {
            color: var(--accent-cyan);
            margin-right: 8px;
        }

        p {
            margin: 0 0 1em;
            overflow-wrap: break-word;
        }

        a {
            color: var(--accent-blue-light);
            text-decoration: none;
            transition: color 0.2s ease;
        }

        a:hover {
            color: var(--text-bright);
            text-decoration: underline;
        }

        /* コードブロック */
        .code-block-wrapper {
            margin: 1.5em 0;
            border-radius: 6px;
            overflow: hidden;
            border: 1px solid var(--border-color);
        }

        .code-block-header {
            display: flex;
            justify-content: space-between;
            align-items: center;
            background-color: var(--bg-secondary);
            padding: 0.4em 1em;
            border-bottom: 1px solid var(--border-color);
        }

        .code-lang {
            font-family: var(--font-mono);
            font-size: 0.75rem;
            color: var(--text-muted);
            text-transform: lowercase;
        }

        .code-copy-btn {
            display: flex;
            align-items: center;
            gap: 4px;
            background: transparent;
            border: none;
            color: var(--text-muted);
            font-family: var(--font-mono);
            font-size: 0.75rem;
            cursor: pointer;
            padding: 4px 8px;
            border-radius: 4px;
            transition: all 0.15s ease;
        }

        .code-copy-btn:hover {
            background: var(--bg-elevated);
            color: var(--text-bright);
        }

        .code-copy-btn.copied {
            color: var(--accent-green-bright);
        }

        .code-copy-btn svg {
            width: 14px;
            height: 14px;
        }

        .code-block-wrapper pre {
            background-color: #1a1f26;
            border: none;
            border-left: 3px solid var(--accent-cyan);
            font-family: var(--font-code);
            padding: 1em;
            margin: 0;
            overflow-x: auto;
            border-radius: 0;
            font-size: 0.9rem;
        }

        /* 通常のpre（ラッパーなし） */
        pre:not(.code-block-wrapper pre) {
            background-color: #1a1f26;
            border: 1px solid var(--border-color);
            border-left: 3px solid var(--accent-cyan);
            font-family: var(--font-code);
            padding: 1em;
            margin: 1.5em 0;
            overflow-x: auto;
            border-radius: 4px;
            font-size: 0.9rem;
        }

        code {
            font-family: var(--font-code);
            background-color: var(--bg-secondary);
            color: var(--accent-orange-bright);
            padding: 0.2em 0.4em;
            margin: 0 0.1em;
            font-size: 85%;
            border-radius: 3px;
            word-break: break-all;
        }

        pre code {
            background: transparent;
            color: inherit;
            font-size: inherit;
            padding: 0;
            margin: 0;
            word-break: normal;
        }

        /* 引用 */
        blockquote {
            background-color: var(--bg-secondary);
            border-left: 4px solid var(--accent-cyan);
            margin: 1.5em 0;
            padding: 1em 1.5em;
            color: var(--text-secondary);
        }

        blockquote p:last-child { margin-bottom: 0; }

        /* テーブル */
        .main-content table {
            border-collapse: collapse;
            width: 100%;
            margin: 1.5em 0;
        }

        .main-content th,
        .main-content td {
            border: 1px solid var(--border-color);
            padding: 0.75em;
            text-align: left;
        }

        .main-content th {
            background-color: var(--bg-secondary);
            color: var(--accent-yellow);
            font-weight: 600;
        }

        /* リスト */
        .main-content ul {
            list-style: none;
            padding-left: 1.5em;
            margin: 1em 0;
        }

        .main-content ul:not(.badge-list) li::before {
            content: "-";
            color: var(--accent-cyan);
            margin-right: 8px;
        }

        .main-content ol {
            list-style: decimal;
            padding-left: 1.5em;
            margin: 1em 0;
            color: var(--accent-cyan);
        }

        .main-content ol li {
            color: var(--text-primary);
        }

        .main-content li {
            margin-bottom: 0.5em;
        }

        .main-content ul ul li::before { content: "*"; }
        .main-content ul ul ul li::before { content: "+"; }

        /* ========================================
           フッター固定エリア（ステータスライン + コマンドライン）
           ======================================== */
        .footer-fixed {
            position: fixed;
            bottom: 0;
            left: 0;
            right: 0;
            z-index: 800;
        }

        /* ステータスライン（lualine風） */
        .statusline {
            background: var(--bg-secondary);
            border-top: 1px solid var(--border-color);
            height: var(--statusline-height);
            display: flex;
            align-items: center;
            justify-content: space-between;
            font-family: var(--font-mono);
            font-size: 0.75rem;
        }

        .statusline-left,
        .statusline-right {
            display: flex;
            align-items: center;
            height: 100%;
        }

        .statusline-mode {
            background: var(--text-muted);
            color: var(--bg-primary);
            padding: 0 12px;
            font-weight: 700;
            height: 100%;
            display: flex;
            align-items: center;
        }

        .statusline-section {
            padding: 0 12px;
            height: 100%;
            display: flex;
            align-items: center;
        }

        .statusline-left .statusline-section {
            border-right: 1px solid var(--border-color);
        }

        .statusline-right .statusline-section {
            border-left: 1px solid var(--border-color);
        }

        .statusline-branch {
            color: var(--accent-green-bright);
        }

        .statusline-branch::before {
            content: " ";
            margin-right: 4px;
        }

        .statusline-privacy {
            text-decoration: none;
            font-size: 0.85em;
            opacity: 0.6;
            transition: opacity 0.2s, transform 0.2s;
        }

        .statusline-privacy:hover {
            opacity: 1;
            transform: scale(1.1);
        }

        .statusline-encoding {
            color: var(--text-muted);
        }

        .statusline-filetype {
            color: var(--accent-orange-bright);
        }

        .statusline-position {
            color: var(--accent-cyan-light);
            min-width: 48px;
            text-align: center;
        }

        /* ペインインジケーター */
        .statusline-pane {
            color: var(--accent-magenta);
            font-weight: bold;
        }
        .statusline-pane:empty {
            display: none;
        }

        /* ペンディングキー（Ctrl+w等）*/
        .statusline-pending {
            color: var(--accent-yellow);
            font-weight: bold;
        }
        .statusline-pending:empty {
            display: none;
        }

        /* ========================================
           ウィンドウペイン・フォーカス
           ======================================== */

        /* ペインにフォーカス時の視覚的フィードバック */
        .pane-focused {
            outline: 2px solid var(--accent-cyan);
            outline-offset: -2px;
        }

        /* EXPLORER内の選択アイテム */
        .explorer-selected {
            background: var(--bg-elevated) !important;
            color: var(--accent-cyan-bright) !important;
            border-left: 2px solid var(--accent-cyan) !important;
            padding-left: 6px !important;
        }

        /* サイドバーがフォーカス時 */
        .sidebar-left.pane-focused {
            border-right-color: var(--accent-cyan);
        }

        /* ========================================
           noice.nvim風トースト通知
           ======================================== */
        .toast-container {
            position: fixed;
            top: 60px;
            right: 20px;
            z-index: 9999;
            display: flex;
            flex-direction: column;
            gap: 8px;
        }

        .toast {
            background: var(--bg-secondary);
            border: 1px solid var(--border-color);
            border-left: 3px solid var(--accent-cyan);
            border-radius: 4px;
            padding: 12px 16px;
            font-family: var(--font-mono);
            font-size: 0.85rem;
            color: var(--text-bright);
            box-shadow: 0 4px 12px rgba(0, 0, 0, 0.4);
            max-width: 350px;
            animation: toast-slide-in 0.3s ease-out;
            display: flex;
            align-items: flex-start;
            gap: 10px;
        }

        .toast.toast-warn {
            border-left-color: var(--status-warn);
        }

        .toast.toast-error {
            border-left-color: var(--accent-red-bright);
        }

        .toast.toast-info {
            border-left-color: var(--accent-blue-light);
        }

        .toast-icon {
            font-size: 1.1rem;
            flex-shrink: 0;
        }

        .toast-content {
            flex: 1;
        }

        .toast-title {
            font-weight: 600;
            margin-bottom: 4px;
            color: var(--accent-yellow);
        }

        .toast-message {
            color: var(--text-primary);
            line-height: 1.4;
        }

        .toast-close {
            background: none;
            border: none;
            color: var(--text-muted);
            cursor: pointer;
            padding: 0;
            font-size: 1rem;
            line-height: 1;
        }

        .toast-close:hover {
            color: var(--text-bright);
        }

        @keyframes toast-slide-in {
            from {
                transform: translateX(100%);
                opacity: 0;
            }
            to {
                transform: translateX(0);
                opacity: 1;
            }
        }

        @keyframes toast-fade-out {
            from {
                transform: translateX(0);
                opacity: 1;
            }
            to {
                transform: translateX(100%);
                opacity: 0;
            }
        }

        .toast.hiding {
            animation: toast-fade-out 0.3s ease-in forwards;
        }

        /* コマンドライン（検索トリガー） */
        .commandline {
            background: var(--bg-primary);
            border-top: 1px solid var(--border-color);
            height: var(--commandline-height);
            display: flex;
            align-items: center;
            padding: 0 12px;
            font-family: var(--font-mono);
            font-size: 0.85rem;
            cursor: pointer;
            transition: background 0.15s ease;
        }

        .commandline:hover {
            background: var(--bg-secondary);
        }

        .commandline-prefix {
            color: var(--accent-cyan);
            margin-right: 4px;
        }

        .commandline input {
            flex: 1;
            background: transparent;
            border: none;
            color: var(--text-bright);
            font-family: inherit;
            font-size: inherit;
            outline: none;
            cursor: pointer;
        }

        .commandline input::placeholder {
            color: var(--text-muted);
        }

        .commandline input:not(:placeholder-shown) {
            cursor: text;
        }

        .commandline-cursor {
            color: var(--text-muted);
            margin-left: auto;
            animation: blink 1s step-end infinite;
        }

        @keyframes blink {
            0%, 100% { opacity: 1; }
            50% { opacity: 0; }
        }

        /* 検索結果オーバーレイ */
        #search-results {
            position: fixed;
            top: calc(var(--titlebar-height) + 50px);
            left: 50%;
            transform: translateX(-50%);
            background: var(--bg-secondary);
            border: 1px solid var(--border-color);
            border-radius: 4px;
            width: 600px;
            max-width: 90vw;
            box-shadow: 0 8px 32px rgba(0,0,0,0.4);
            z-index: 1000;
            max-height: 400px;
            overflow-y: auto;
            display: none;
        }

        #search-results ul {
            list-style: none;
            padding: 0;
            margin: 0;
        }

        #search-results li a {
            display: block;
            padding: 10px 16px;
            color: var(--text-primary);
            text-decoration: none;
            font-family: var(--font-mono);
            font-size: 0.85rem;
            border-bottom: 1px solid var(--border-color);
        }

        #search-results li a:hover {
            background: var(--bg-elevated);
            color: var(--text-bright);
        }

        #search-results li:last-child a {
            border-bottom: none;
        }

        /* ========================================
           snacks.nvim grep風検索モーダル
           ======================================== */
        .search-modal {
            position: fixed;
            top: 0;
            left: 0;
            right: 0;
            bottom: 0;
            background: rgba(0, 0, 0, 0.7);
            display: none;
            align-items: center;
            justify-content: center;
            z-index: 2000;
            padding: 20px;
        }

        .search-modal.open {
            display: flex;
        }

        .search-modal-container {
            background: var(--bg-secondary);
            border: 1px solid var(--border-color);
            border-radius: 8px;
            width: 1100px;
            max-width: 95vw;
            height: 85vh;
            max-height: 85vh;
            display: flex;
            flex-direction: column;
            box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.6);
        }

        .search-modal-header {
            display: flex;
            align-items: center;
            justify-content: space-between;
            padding: 12px 16px;
            border-bottom: 1px solid var(--border-color);
            background: linear-gradient(180deg, var(--bg-secondary) 0%, var(--bg-primary) 100%);
            border-radius: 8px 8px 0 0;
        }

        .search-modal-title {
            font-family: var(--font-mono);
            font-size: 0.85rem;
            color: var(--accent-yellow);
            display: flex;
            align-items: center;
            gap: 8px;
        }

        .search-modal-close {
            background: none;
            border: none;
            color: var(--text-muted);
            cursor: pointer;
            font-size: 1.2rem;
            padding: 4px 8px;
            border-radius: 4px;
            transition: all 0.15s ease;
        }

        .search-modal-close:hover {
            background: var(--accent-red-bright);
            color: white;
        }

        .search-input-wrapper {
            padding: 12px 16px;
            border-bottom: 1px solid var(--border-color);
            background: var(--bg-primary);
        }

        .search-input-wrapper input {
            width: 100%;
            background: var(--bg-secondary);
            border: 1px solid var(--border-color);
            border-radius: 4px;
            padding: 10px 12px;
            color: var(--text-bright);
            font-family: var(--font-mono);
            font-size: 0.9rem;
            outline: none;
        }

        .search-input-wrapper input:focus {
            border-color: var(--accent-cyan);
        }

        .search-input-wrapper input::placeholder {
            color: var(--text-muted);
        }

        .search-modal-body {
            display: flex;
            flex: 1;
            overflow: hidden;
            min-height: 300px;
        }

        /* 左ペイン: 検索結果リスト */
        .search-results-pane {
            flex: 1;
            display: flex;
            flex-direction: column;
            border-right: 1px solid var(--border-color);
            min-width: 0;
        }

        .search-results-list {
            flex: 1;
            overflow-y: auto;
            padding: 8px 0;
        }

        .search-result-item {
            padding: 8px 16px;
            cursor: pointer;
            transition: background 0.1s ease;
            border-left: 2px solid transparent;
        }

        .search-result-item:hover {
            background: var(--bg-primary);
        }

        .search-result-item.selected {
            background: var(--bg-elevated);
            border-left-color: var(--accent-cyan);
        }

        /* grep形式: ヘッダー（タイトル:行番号） */
        .search-result-header {
            font-family: var(--font-mono);
            font-size: 0.75rem;
            margin-bottom: 4px;
            white-space: nowrap;
            overflow: hidden;
            text-overflow: ellipsis;
        }

        .search-result-title {
            color: var(--accent-green-bright);
        }

        .search-result-line-num {
            color: var(--accent-cyan);
        }

        /* grep形式: 行テキスト */
        .search-result-text {
            font-family: var(--font-mono);
            font-size: 0.8rem;
            color: var(--text-primary);
            white-space: nowrap;
            overflow: hidden;
            text-overflow: ellipsis;
            line-height: 1.4;
        }

        /* キーワードハイライト */
        .search-keyword-highlight {
            background: var(--accent-yellow);
            color: var(--bg-primary);
            padding: 0 2px;
            border-radius: 2px;
            font-weight: 600;
        }

        .search-results-empty {
            padding: 32px 16px;
            text-align: center;
            color: var(--text-muted);
            font-family: var(--font-mono);
            font-size: 0.85rem;
        }

        .result-location {
            font-family: var(--font-mono);
            font-size: 0.75rem;
            color: var(--accent-green-bright);
            margin-bottom: 2px;
        }

        .result-text {
            font-family: var(--font-mono);
            font-size: 0.8rem;
            color: var(--text-primary);
            white-space: nowrap;
            overflow: hidden;
            text-overflow: ellipsis;
        }

        .result-text mark {
            background: var(--accent-yellow);
            color: var(--bg-primary);
            padding: 0 2px;
            border-radius: 2px;
        }

        .search-no-results {
            padding: 32px 16px;
            text-align: center;
            color: var(--text-muted);
            font-family: var(--font-mono);
            font-size: 0.85rem;
        }

        /* 右ペイン: プレビュー */
        .search-preview-pane {
            flex: 1;
            overflow-y: auto;
            background: var(--bg-primary);
            padding: 16px;
            min-width: 0;
        }

        .preview-title {
            font-family: var(--font-mono);
            font-size: 0.9rem;
            color: var(--accent-yellow);
            margin-bottom: 4px;
        }

        .preview-url {
            font-family: var(--font-mono);
            font-size: 0.75rem;
            color: var(--text-muted);
            margin-bottom: 8px;
        }

        .preview-separator {
            border-top: 1px solid var(--border-color);
            margin-bottom: 12px;
        }

        .preview-empty {
            padding: 32px 16px;
            text-align: center;
            color: var(--text-muted);
            font-family: var(--font-mono);
            font-size: 0.85rem;
        }

        .preview-content {
            font-family: var(--font-mono);
            font-size: 0.8rem;
        }

        .preview-line {
            display: flex;
            line-height: 1.6;
            padding: 2px 0;
        }

        .preview-line.match {
            background: rgba(59, 123, 125, 0.2);
            margin: 0 -16px;
            padding: 2px 16px;
            border-left: 2px solid var(--accent-cyan);
        }

        .preview-line .line-num,
        .preview-line-num {
            flex: 0 0 auto;
            color: var(--text-secondary);
            padding-right: 12px;
            user-select: none;
        }

        .preview-line .line-text,
        .preview-line-text {
            flex: 1;
            color: var(--text-primary);
            word-break: break-all;
        }

        .preview-line .line-text mark,
        .preview-line-text mark,
        .preview-line-text .search-keyword-highlight {
            background: var(--accent-yellow);
            color: var(--bg-primary);
            padding: 0 2px;
            border-radius: 2px;
        }

        /* 左ペインフォーカス状態（NORMALモード） */
        .search-results-list.focused {
            outline: 2px solid var(--accent-cyan);
            outline-offset: -2px;
        }

        /* フッター（モード、件数、ショートカット） */
        .search-modal-footer {
            padding: 8px 16px;
            border-top: 1px solid var(--border-color);
            background: var(--bg-secondary);
            font-family: var(--font-mono);
            font-size: 0.75rem;
            color: var(--text-muted);
            display: flex;
            justify-content: space-between;
            align-items: center;
            border-radius: 0 0 8px 8px;
        }

        .search-modal-footer-left {
            display: flex;
            align-items: center;
            gap: 12px;
        }

        .search-mode-indicator {
            padding: 2px 8px;
            border-radius: 3px;
            font-weight: 600;
            font-size: 0.7rem;
        }

        .search-mode-indicator.mode-insert {
            background: var(--accent-green-bright);
            color: var(--bg-primary);
        }

        .search-mode-indicator.mode-normal {
            background: var(--accent-blue-light);
            color: var(--bg-primary);
        }

        .search-modal-footer kbd {
            background: var(--bg-primary);
            border: 1px solid var(--border-color);
            border-radius: 3px;
            padding: 2px 6px;
            margin: 0 2px;
        }

        /* モバイル対応: プレビュー非表示 */
        @media screen and (max-width: 768px) {
            .search-modal-container {
                max-height: 90vh;
            }

            .search-preview-pane {
                display: none;
            }

            .search-results-pane {
                border-right: none;
            }

            .search-modal-footer span:last-child {
                display: none;
            }
        }

        /* ========================================
           タグ一覧モーダル（検索モーダルと同じ構造）
           ======================================== */
        .tags-modal {
            position: fixed;
            top: 0;
            left: 0;
            right: 0;
            bottom: 0;
            background: rgba(0, 0, 0, 0.7);
            display: none;
            align-items: center;
            justify-content: center;
            z-index: 2000;
            padding: 20px;
        }

        .tags-modal.open {
            display: flex;
        }

        .tags-modal-container {
            background: var(--bg-secondary);
            border: 1px solid var(--border-color);
            border-radius: 8px;
            width: 1100px;
            max-width: 95vw;
            height: 85vh;
            max-height: 85vh;
            display: flex;
            flex-direction: column;
            box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.6);
        }

        .tags-modal-header {
            display: flex;
            align-items: center;
            justify-content: space-between;
            padding: 12px 16px;
            border-bottom: 1px solid var(--border-color);
            background: linear-gradient(180deg, var(--bg-secondary) 0%, var(--bg-primary) 100%);
            border-radius: 8px 8px 0 0;
        }

        .tags-modal-title {
            font-family: var(--font-mono);
            font-size: 0.85rem;
            color: var(--accent-cyan);
            display: flex;
            align-items: center;
            gap: 8px;
        }

        .tags-modal-close {
            background: none;
            border: none;
            color: var(--text-muted);
            cursor: pointer;
            font-size: 1.2rem;
            padding: 4px 8px;
            border-radius: 4px;
            transition: all 0.15s ease;
        }

        .tags-modal-close:hover {
            background: var(--accent-red-bright);
            color: white;
        }

        .tags-input-wrapper {
            padding: 12px 16px;
            border-bottom: 1px solid var(--border-color);
            background: var(--bg-primary);
        }

        .tags-input-wrapper input {
            width: 100%;
            background: var(--bg-secondary);
            border: 1px solid var(--border-color);
            border-radius: 4px;
            padding: 10px 12px;
            color: var(--text-bright);
            font-family: var(--font-mono);
            font-size: 0.9rem;
            outline: none;
        }

        .tags-input-wrapper input:focus {
            border-color: var(--accent-cyan);
        }

        .tags-input-wrapper input::placeholder {
            color: var(--text-muted);
        }

        .tags-modal-body {
            display: flex;
            flex: 1;
            overflow: hidden;
            min-height: 300px;
        }

        /* 左ペイン: タグリスト */
        .tags-list-pane {
            flex: 1;
            display: flex;
            flex-direction: column;
            border-right: 1px solid var(--border-color);
            min-width: 0;
        }

        .tags-list {
            flex: 1;
            overflow-y: auto;
            padding: 8px 0;
        }

        .tag-result-item {
            padding: 8px 16px;
            cursor: pointer;
            transition: background 0.1s ease;
            border-left: 2px solid transparent;
            display: flex;
            align-items: center;
            justify-content: space-between;
        }

        .tag-result-item:hover {
            background: var(--bg-primary);
        }

        .tag-result-item.selected {
            background: var(--bg-primary);
            border-left-color: var(--accent-cyan);
        }

        .tag-name {
            font-family: var(--font-mono);
            font-size: 0.9rem;
            color: var(--text-bright);
        }

        .tag-count {
            background: var(--accent-blue-light);
            color: var(--bg-primary);
            padding: 2px 8px;
            border-radius: 10px;
            font-size: 0.75rem;
            font-weight: 600;
        }

        .tags-no-results {
            padding: 16px;
            color: var(--text-muted);
            font-family: var(--font-mono);
            text-align: center;
        }

        /* 右ペイン: プレビュー */
        .tags-preview-pane {
            flex: 1.5;
            overflow-y: auto;
            padding: 16px;
            background: var(--bg-primary);
        }

        .tags-preview-pane .preview-title {
            font-family: var(--font-mono);
            font-size: 1rem;
            color: var(--accent-cyan);
            margin-bottom: 12px;
            padding-bottom: 8px;
            border-bottom: 1px solid var(--border-color);
        }

        .tags-preview-pane .preview-content {
            display: flex;
            flex-direction: column;
            gap: 4px;
        }

        .tags-preview-pane .preview-line {
            display: flex;
            align-items: baseline;
            gap: 12px;
            padding: 6px 8px;
            border-radius: 4px;
            transition: background 0.15s ease;
        }

        .tags-preview-pane .preview-line:hover {
            background: var(--bg-secondary);
        }

        .tags-preview-pane .line-num {
            font-size: 0.75rem;
            color: var(--text-muted);
            min-width: 80px;
            flex-shrink: 0;
        }

        .tags-preview-pane .line-text {
            color: var(--text-bright);
            font-size: 0.9rem;
        }

        .tags-preview-pane .article-link {
            text-decoration: none;
        }

        .tags-preview-pane .article-link:hover {
            color: var(--accent-cyan);
        }

        .tags-modal-footer {
            padding: 8px 16px;
            border-top: 1px solid var(--border-color);
            background: var(--bg-secondary);
            font-family: var(--font-mono);
            font-size: 0.75rem;
            color: var(--text-muted);
            display: flex;
            justify-content: space-between;
            align-items: center;
            border-radius: 0 0 8px 8px;
        }

        .tags-modal-footer-left {
            display: flex;
            align-items: center;
            gap: 12px;
        }

        .tags-modal-footer kbd {
            background: var(--bg-primary);
            border: 1px solid var(--border-color);
            border-radius: 3px;
            padding: 2px 6px;
            margin: 0 2px;
        }

        .tags-mode-indicator {
            padding: 2px 8px;
            border-radius: 3px;
            font-weight: bold;
            font-size: 0.7rem;
        }

        .tags-mode-indicator.mode-insert {
            background: var(--accent-green);
            color: var(--bg-primary);
        }

        .tags-mode-indicator.mode-normal {
            background: var(--accent-blue);
            color: var(--bg-primary);
        }

        .tags-list.focused .tag-result-item.selected {
            background: var(--bg-primary);
            border-left-color: var(--accent-green);
        }

        /* モバイル対応: プレビュー非表示 */
        @media screen and (max-width: 768px) {
            .tags-modal-container {
                max-height: 90vh;
            }

            .tags-preview-pane {
                display: none;
            }

            .tags-list-pane {
                border-right: none;
            }

            .tags-modal-footer span:last-child {
                display: none;
            }
        }

        /* ========================================
           サイドバー内リスト（デフォルト）
           ======================================== */
        .sidebar-left ul { list-style: none; padding: 0; margin: 0; }
        .sidebar-left li { margin-bottom: 0; }

        /* フォルダ折りたたみ機能 */
        .folder-item > ul {
            padding-left: 12px;
        }

        .folder-item.collapsed > ul {
            display: none;
        }

        .folder-toggle {
            cursor: pointer;
            user-select: none;
        }

        /* 目次セクション（EXPLORERトップ） */
        .toc-section {
            padding: 6px 12px;
            border-bottom: 1px solid var(--border-color);
            margin-bottom: 4px;
        }

        .toc-header {
            display: flex;
            align-items: center;
            gap: 6px;
            font-size: 0.85rem;
            color: var(--text-muted);
            text-transform: uppercase;
            letter-spacing: 0.05em;
            margin-bottom: 6px;
        }

        .toc-icon {
            font-size: 0.9rem;
        }

        .toc-list {
            list-style: none;
            padding: 0;
            margin: 0;
        }

        .toc-section .toc-item {
            margin: 0;
        }

        .toc-section .toc-item a {
            display: block;
            padding: 4px 0;
            font-size: 0.8rem;
            color: var(--text-primary);
            text-decoration: none;
            white-space: nowrap;
            overflow: hidden;
            text-overflow: ellipsis;
            transition: color 0.15s ease;
        }

        .toc-section .toc-item a:hover {
            color: var(--text-bright);
        }

        .toc-section .toc-h3 a {
            font-size: 0.75rem;
            color: var(--text-secondary);
        }

        /* 現在表示中の見出し（スクロール追跡） */
        .toc-section .toc-item.active a {
            color: var(--accent-cyan-bright);
            font-weight: 600;
        }

        .toc-section .toc-item.active::before {
            content: "";
            position: absolute;
            left: -8px;
            top: 2px;
            bottom: 2px;
            width: 2px;
            background: var(--accent-cyan-bright);
            border-radius: 1px;
        }

        .toc-section .toc-item {
            position: relative;
            padding-left: 8px;
        }

        .toc-section .toc-h3 {
            padding-left: 20px;
        }

        /* 目次展開エリア（旧スタイル - 必要なら削除可） */
        .toc-expanded {
            padding-left: 20px;
            border-left: 1px solid var(--border-color);
            margin-left: 12px;
            margin-top: 4px;
        }

        .toc-expanded .toc-item {
            padding: 2px 8px;
        }

        /* ========================================
           ハンバーガーメニュー・オーバーレイ
           ======================================== */
        .hamburger-btn, .overlay, .sidebar-close-btn { display: none; }

        /* ========================================
           モバイル用コマンドパレット（ボトムシート）
           ======================================== */
        .bottomsheet-overlay {
            display: none;
            position: fixed;
            top: 0;
            left: 0;
            width: 100%;
            height: 100%;
            background: rgba(0, 0, 0, 0.5);
            z-index: 900;
            opacity: 0;
            transition: opacity 0.3s ease;
        }

        .bottomsheet-overlay.is-open {
            display: block;
            opacity: 1;
        }

        .command-bottomsheet {
            position: fixed;
            bottom: 0;
            left: 0;
            right: 0;
            background: var(--bg-secondary);
            border-top: 1px solid var(--border-color);
            border-radius: 16px 16px 0 0;
            z-index: 901;
            transform: translateY(100%);
            transition: transform 0.3s cubic-bezier(0.4, 0, 0.2, 1);
            padding-bottom: env(safe-area-inset-bottom, 0);
            max-height: 60vh;
            overflow-y: auto;
        }

        .command-bottomsheet.is-open {
            transform: translateY(0);
        }

        .bottomsheet-handle {
            display: flex;
            justify-content: center;
            padding: 12px 0 8px;
        }

        .bottomsheet-handle-bar {
            width: 40px;
            height: 4px;
            background: var(--text-muted);
            border-radius: 2px;
            opacity: 0.5;
        }

        .bottomsheet-header {
            padding: 0 16px 12px;
            border-bottom: 1px solid var(--border-color);
        }

        .bottomsheet-title {
            font-family: var(--font-mono);
            font-size: 0.85rem;
            color: var(--accent-cyan);
            font-weight: 600;
        }

        .bottomsheet-presets {
            padding: 8px 0;
        }

        .bottomsheet-preset-btn {
            display: flex;
            align-items: center;
            width: 100%;
            padding: 14px 16px;
            background: transparent;
            border: none;
            cursor: pointer;
            transition: background 0.15s ease;
            gap: 12px;
            text-align: left;
        }

        .bottomsheet-preset-btn:hover,
        .bottomsheet-preset-btn:active {
            background: rgba(255, 255, 255, 0.05);
        }

        .preset-icon {
            color: var(--accent-cyan);
            width: 24px;
            height: 16px;
            display: flex;
            align-items: center;
            justify-content: center;
            flex-shrink: 0;
        }

        .preset-icon svg {
            stroke: var(--accent-cyan);
        }

        .preset-cmd {
            font-family: var(--font-mono);
            font-size: 0.85rem;
            color: var(--text-bright);
            min-width: 80px;
            flex-shrink: 0;
        }

        .preset-desc {
            font-size: 0.8rem;
            color: var(--text-muted);
        }

        /* ========================================
           レスポンシブ（モバイル）
           ======================================== */
        @media screen and (max-width: 992px) {
            :root {
                --sidebar-width: 260px;
            }

            .titlebar {
                padding: 0 8px;
            }

            .titlebar h1 {
                font-size: 0.85rem;
            }

            .window-buttons {
                display: none;
            }

            .container {
                flex-direction: column;
            }

            .sidebar-left {
                position: fixed;
                right: 0;
                left: auto;
                top: 0;
                height: 100%;
                width: 100%;
                z-index: 1000;
                transform: translateX(100%);
                transition: transform 0.3s ease-in-out;
            }

            .sidebar-left.is-open {
                transform: translateX(0);
            }

            .main-area {
                width: 100%;
            }

            .tab-bar {
                display: none;
            }

            .line-numbers {
                display: none;
            }

            .main-content {
                padding: 1em;
                overflow-x: hidden;
            }

            .main-content table {
                display: block;
                overflow-x: auto;
                -webkit-overflow-scrolling: touch;
            }

            /* モバイルでは行番号を非表示 */
            .main-content > h1::before,
            .main-content > h2::before,
            .main-content > h3::before,
            .main-content > h4::before,
            .main-content > p::before,
            .main-content > ul::before,
            .main-content > ol::before,
            .main-content > blockquote::before,
            .main-content > pre::before,
            .main-content > table::before,
            .main-content > hr::before,
            .main-content > .code-block-wrapper::before {
                display: none;
            }

            /* モバイルでは見出しのパディングも調整（記事ページのみ） */
            .main-content.page-article > h2,
            .main-content.page-article > h3,
            .main-content.page-article > h4 {
                padding-left: 1.5em;
            }

            .hamburger-btn {
                display: flex;
                flex-direction: column;
                justify-content: space-around;
                width: 24px;
                height: 20px;
                background: transparent;
                border: none;
                cursor: pointer;
                padding: 0;
                z-index: 1001;
                position: absolute;
                right: 16px;
                top: 50%;
                transform: translateY(-50%);
            }

            .hamburger-btn span {
                width: 100%;
                height: 2px;
                background-color: var(--text-bright);
                border-radius: 2px;
                transition: all 0.3s linear;
            }

            .overlay {
                position: fixed;
                top: 0;
                left: 0;
                width: 100%;
                height: 100%;
                background: rgba(0, 0, 0, 0.6);
                z-index: 999;
            }

            .overlay.is-open {
                display: block;
            }

            .sidebar-close-btn {
                display: flex;
                align-items: center;
                justify-content: center;
                width: 28px;
                height: 28px;
                background: transparent;
                border: 1px solid var(--border-color);
                border-radius: 4px;
                cursor: pointer;
                color: var(--text-bright);
                font-size: 1.1rem;
                position: absolute;
                right: 12px;
                top: 50%;
                transform: translateY(-50%);
                transition: background 0.15s ease;
            }

            .sidebar-close-btn:hover {
                background: var(--bg-elevated);
            }

            .statusline {
                font-size: 0.7rem;
            }

            .statusline-section {
                padding: 0 8px;
            }

            .commandline {
                font-size: 0.8rem;
            }

            #search-results {
                width: 95vw;
                top: calc(var(--titlebar-height) + 10px);
            }
        }

        /* ========================================
           スクロールバーカスタマイズ
           ======================================== */
        ::-webkit-scrollbar {
            width: 8px;
            height: 8px;
        }

        ::-webkit-scrollbar-track {
            background: var(--bg-secondary);
        }

        ::-webkit-scrollbar-thumb {
            background: var(--accent-cyan);
            border-radius: 4px;
        }

        ::-webkit-scrollbar-thumb:hover {
            background: var(--accent-cyan-light);
        }

        /* ========================================
           タグ・言語バッジ
           ======================================== */
        .badge-list {
            display: flex;
            flex-wrap: wrap;
            gap: 6px;
            margin: 0.5em 0;
            list-style: none;
            padding: 0;
        }

        /* バッジリストは行番号を非表示にする */
        .main-content > ul.badge-list {
            counter-increment: none;
        }

        .main-content > ul.badge-list::before {
            content: none;
        }

        .badge-list li {
            margin: 0;
        }

        .badge-list li::before {
            content: none;
        }

        .badge {
            display: inline-block;
            padding: 2px 8px;
            border-radius: 4px;
            font-family: var(--font-mono);
            font-size: 0.75rem;
        }

        .badge-lang {
            background: var(--bg-elevated);
            color: var(--text-bright);
            border: 1px solid var(--accent-blue);
        }

        .badge-tag {
            background: var(--bg-secondary);
            color: var(--accent-cyan-light);
            border: 1px solid var(--accent-cyan);
        }

        /* 検索ハイライト（ページ遷移後） */
        .search-highlight {
            background: var(--accent-yellow);
            color: var(--bg-primary);
            padding: 1px 3px;
            border-radius: 2px;
        }

        /* 現在選択中のハイライト */
        .search-highlight.current {
            background: var(--accent-orange-bright);
            box-shadow: 0 0 0 2px var(--accent-orange-bright);
            animation: highlight-pulse 0.3s ease-out;
        }

        @keyframes highlight-pulse {
            0% { box-shadow: 0 0 0 4px rgba(227, 141, 44, 0.6); }
            100% { box-shadow: 0 0 0 2px var(--accent-orange-bright); }
        }

        /* 検索カウント表示 */
        .search-count {
            color: var(--accent-yellow);
            font-weight: 600;
        }

        .search-count:empty {
            display: none;
        }

        /* モバイル用ハイライトナビゲーション */
        .highlight-nav {
            display: none;
            position: fixed;
            bottom: 80px;
            right: 16px;
            z-index: 850;
            flex-direction: column;
            align-items: center;
            gap: 8px;
        }

        .highlight-nav-btn {
            width: 44px;
            height: 44px;
            border-radius: 50%;
            border: 1px solid var(--border-color);
            background: var(--bg-secondary);
            color: var(--text-bright);
            font-size: 1.2rem;
            display: flex;
            align-items: center;
            justify-content: center;
            cursor: pointer;
            transition: all 0.15s ease;
            box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
        }

        .highlight-nav-btn:active {
            background: var(--bg-elevated);
            transform: scale(0.95);
        }

        /* モバイル時のみ表示（ハイライトがある時） */
        @media screen and (max-width: 992px) {
            .highlight-nav.visible {
                display: flex;
            }
        }
    "#;

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


                meta property="og:title" content=(config.base.page_title);
                meta property="og:description" content=(description);
                meta property="og:type" content="website";
                meta property="og:site_name" content="dnfolio";
                meta property="og:url" content=(config.base.canonical_url);
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

                style {
                    (PreEscaped(
                        Minifier::default()
                            .minify(css, Level::Two)
                            .unwrap_or_else(|_| css.to_string())
                    ))
                }
            }
            body data-version=(GIT_VERSION) {
                // ローディングオーバーレイ（Rust歯車）- 初期表示、WASM初期化完了後に非表示
                div id="loading-overlay" class="loading-overlay visible" {
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
