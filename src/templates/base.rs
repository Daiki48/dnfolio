use crate::models::MetaData;
use maud::{DOCTYPE, Markup, PreEscaped, html};

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
            --sidebar-width: 340px;

            /* フォント */
            --font-mono: 'JetBrains Mono', 'Fira Code', 'SF Mono', monospace;
            --font-body: 'Inter', 'Noto Sans JP', system-ui, sans-serif;
            --font-code: 'JetBrains Mono', 'Fira Code', 'Consolas', monospace;
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
            font-size: 0.8rem;
            width: 12px;
            text-align: center;
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
            padding-left: 2em;
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
            margin-bottom: 16px;
            padding-bottom: 8px;
            border-bottom: 1px solid var(--border-color);
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

        .preview-line .line-num {
            flex: 0 0 40px;
            color: var(--text-secondary);
            text-align: right;
            padding-right: 12px;
            user-select: none;
        }

        .preview-line .line-text {
            flex: 1;
            color: var(--text-primary);
            word-break: break-all;
        }

        .preview-line .line-text mark {
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
            color: var(--text-secondary);
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
            color: var(--text-muted);
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
        .hamburger-btn, .overlay { display: none; }

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
                left: 0;
                top: 0;
                height: 100%;
                width: var(--sidebar-width);
                z-index: 1000;
                transform: translateX(-100%);
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
            gap: 4px;
            font-family: var(--font-mono);
        }

        .highlight-nav-count {
            background: var(--bg-secondary);
            border: 1px solid var(--border-color);
            border-radius: 4px;
            padding: 4px 8px;
            font-size: 0.75rem;
            color: var(--accent-yellow);
            font-weight: 600;
        }

        .highlight-nav-buttons {
            display: flex;
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

    let js = r#"
        document.addEventListener('DOMContentLoaded', () => {
            // 検索結果からの遷移時：キーワードハイライトとスクロール
            const urlParams = new URLSearchParams(window.location.search);
            const highlightQuery = urlParams.get('highlight');
            const lineText = urlParams.get('lineText');
            const lineNum = parseInt(urlParams.get('lineNum') || '0', 10);
            const commandlineInput = document.getElementById('commandline-input');
            const searchCountDisplay = document.getElementById('search-count');
            const highlightNav = document.getElementById('highlight-nav');
            const highlightNavCount = document.getElementById('highlight-nav-count');
            const highlightNavPrev = document.getElementById('highlight-nav-prev');
            const highlightNavNext = document.getElementById('highlight-nav-next');

            // n/Nナビゲーション用のグローバル状態
            let allHighlights = [];
            let currentHighlightIndex = 0;

            // カウント表示を更新する関数
            const updateSearchCount = () => {
                if (allHighlights.length === 0) {
                    if (searchCountDisplay) searchCountDisplay.textContent = '';
                    if (highlightNav) highlightNav.classList.remove('visible');
                } else {
                    const countText = `[${currentHighlightIndex + 1}/${allHighlights.length}]`;
                    if (searchCountDisplay) searchCountDisplay.textContent = countText;
                    if (highlightNavCount) highlightNavCount.textContent = countText;
                    if (highlightNav) highlightNav.classList.add('visible');
                }
            };

            // 現在のハイライトにcurrentクラスを設定
            const setCurrentHighlight = (index) => {
                allHighlights.forEach((el, i) => {
                    el.classList.toggle('current', i === index);
                });
                currentHighlightIndex = index;
                updateSearchCount();
                // 中央にスクロール
                if (allHighlights[index]) {
                    allHighlights[index].scrollIntoView({ behavior: 'smooth', block: 'center' });
                }
            };

            // 次/前のハイライトに移動
            const navigateHighlight = (direction) => {
                if (allHighlights.length === 0) return;
                let newIndex = currentHighlightIndex + direction;
                // ループ
                if (newIndex < 0) newIndex = allHighlights.length - 1;
                if (newIndex >= allHighlights.length) newIndex = 0;
                setCurrentHighlight(newIndex);
            };

            // ハイライトを適用する関数
            const applyHighlight = (query, targetLineText, targetLineNum) => {
                const mainContent = document.querySelector('.main-content');
                if (!mainContent || !query) return;

                // 配列をリセット
                allHighlights = [];
                currentHighlightIndex = 0;

                // CSSカウンター対象のブロック要素を取得（行番号と対応）
                // .badge-listはcounter-increment: noneなので除外
                const blockElements = Array.from(mainContent.querySelectorAll(
                    ':scope > h1, :scope > h2, :scope > h3, :scope > h4, :scope > p, :scope > ul:not(.badge-list), :scope > ol, :scope > blockquote, :scope > pre, :scope > table, :scope > hr, :scope > div.code-block-wrapper'
                ));

                // ターゲット行の要素を特定（lineNumは1始まり）
                const targetElement = targetLineNum > 0 && targetLineNum <= blockElements.length
                    ? blockElements[targetLineNum - 1]
                    : null;

                const walker = document.createTreeWalker(
                    mainContent,
                    NodeFilter.SHOW_TEXT,
                    null,
                    false
                );

                const nodesToHighlight = [];
                let node;
                const lowerQuery = query.toLowerCase();

                while (node = walker.nextNode()) {
                    if (node.textContent.toLowerCase().includes(lowerQuery)) {
                        // このノードがターゲット要素内にあるかチェック
                        const isInTargetElement = targetElement && targetElement.contains(node);
                        nodesToHighlight.push({
                            node: node,
                            isInTargetElement: isInTargetElement
                        });
                    }
                }

                let targetHighlightIndex = -1;
                let highlightCount = 0;

                nodesToHighlight.forEach(item => {
                    const textNode = item.node;
                    const text = textNode.textContent;
                    const regex = new RegExp(`(${query.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')})`, 'gi');
                    const parts = text.split(regex);

                    if (parts.length > 1) {
                        const fragment = document.createDocumentFragment();
                        parts.forEach(part => {
                            if (part.toLowerCase() === lowerQuery) {
                                const mark = document.createElement('mark');
                                mark.className = 'search-highlight';
                                mark.textContent = part;
                                fragment.appendChild(mark);
                                // ターゲット要素内の最初のハイライトを記録
                                if (item.isInTargetElement && targetHighlightIndex === -1) {
                                    targetHighlightIndex = highlightCount;
                                }
                                allHighlights.push(mark);
                                highlightCount++;
                            } else {
                                fragment.appendChild(document.createTextNode(part));
                            }
                        });
                        textNode.parentNode.replaceChild(fragment, textNode);
                    }
                });

                // 初期インデックスを設定（ターゲットがあればそこ、なければ0）
                const initialIndex = targetHighlightIndex >= 0 ? targetHighlightIndex : 0;
                if (allHighlights.length > 0) {
                    setCurrentHighlight(initialIndex);
                }
            };

            // ハイライトを削除する関数
            const removeHighlights = () => {
                const highlights = document.querySelectorAll('.search-highlight');
                highlights.forEach(mark => {
                    const parent = mark.parentNode;
                    const text = document.createTextNode(mark.textContent);
                    parent.replaceChild(text, mark);
                    parent.normalize();
                });
                allHighlights = [];
                currentHighlightIndex = 0;
                updateSearchCount();
            };

            // n/Nキーハンドラ（グローバル）
            document.addEventListener('keydown', (e) => {
                // 入力欄でなく、検索モーダルが閉じている時のみ
                if (['INPUT', 'TEXTAREA'].includes(document.activeElement.tagName)) return;
                const searchModal = document.getElementById('search-modal');
                if (searchModal && searchModal.classList.contains('open')) return;

                // DOM上のハイライトを確認（search.jsでDOMが変更された場合に対応）
                const currentDomHighlights = document.querySelectorAll('.search-highlight');
                if (currentDomHighlights.length === 0) {
                    // DOMにハイライトがない場合は状態をリセット
                    if (allHighlights.length > 0) {
                        allHighlights = [];
                        currentHighlightIndex = 0;
                        updateSearchCount();
                    }
                    return;
                }

                // DOM上のハイライトと配列が不一致の場合は再構築
                if (allHighlights.length !== currentDomHighlights.length) {
                    allHighlights = Array.from(currentDomHighlights);
                    currentHighlightIndex = Math.min(currentHighlightIndex, allHighlights.length - 1);
                    updateSearchCount();
                }

                if (e.key === 'n') {
                    navigateHighlight(1);
                    e.preventDefault();
                } else if (e.key === 'N') {
                    navigateHighlight(-1);
                    e.preventDefault();
                }
            });

            // 検索結果からの遷移時
            if (highlightQuery) {
                // コマンドラインにキーワードを表示
                if (commandlineInput) {
                    commandlineInput.value = highlightQuery;
                    commandlineInput.removeAttribute('readonly');
                }

                applyHighlight(highlightQuery, lineText, lineNum);

                // URLからパラメータを削除（履歴は残さない）
                const cleanUrl = window.location.pathname;
                window.history.replaceState({}, '', cleanUrl);
            }

            // コマンドライン入力の変更を監視
            if (commandlineInput) {
                commandlineInput.addEventListener('input', () => {
                    if (commandlineInput.value.trim() === '') {
                        removeHighlights();
                        commandlineInput.setAttribute('readonly', '');
                    }
                });

                // Escapeでコマンドラインをクリア
                commandlineInput.addEventListener('keydown', (e) => {
                    if (e.key === 'Escape') {
                        commandlineInput.value = '';
                        commandlineInput.blur();
                        commandlineInput.setAttribute('readonly', '');
                        removeHighlights();
                        e.preventDefault();
                        e.stopPropagation();
                    }
                });

            }

            // モバイル用フローティングナビゲーションボタン
            if (highlightNavPrev) {
                highlightNavPrev.addEventListener('click', () => navigateHighlight(-1));
            }
            if (highlightNavNext) {
                highlightNavNext.addEventListener('click', () => navigateHighlight(1));
            }

            const hamburgerBtn = document.getElementById('hamburger-btn');
            const sidebarLeft = document.getElementById('sidebar-left');
            const overlay = document.getElementById('overlay');

            const toggleSidebar = () => {
                sidebarLeft.classList.toggle('is-open');
                overlay.classList.toggle('is-open');
            };

            if (hamburgerBtn && sidebarLeft && overlay) {
                hamburgerBtn.addEventListener('click', toggleSidebar);
                overlay.addEventListener('click', toggleSidebar);
            }

            // '/' キーでコマンドラインにフォーカス
            document.addEventListener('keydown', (e) => {
                if (e.key === '/' && document.activeElement.tagName !== 'INPUT') {
                    e.preventDefault();
                    const searchInput = document.getElementById('search-input');
                    if (searchInput) {
                        searchInput.focus();
                    }
                }
                // Escapeで検索をクリア（検索モーダルが開いていない時のみ）
                if (e.key === 'Escape') {
                    const searchModal = document.getElementById('search-modal');
                    if (searchModal && searchModal.classList.contains('open')) {
                        // 検索モーダルが開いている時は何もしない（search.jsで処理）
                        return;
                    }
                    const searchInput = document.getElementById('search-input');
                    const searchResults = document.getElementById('search-results');
                    if (searchInput) {
                        searchInput.value = '';
                        searchInput.blur();
                    }
                    if (searchResults) {
                        searchResults.style.display = 'none';
                    }
                }
            });

            // 折りたたみ機能
            const toggleBtns = document.querySelectorAll('.folder-toggle');
            toggleBtns.forEach(btn => {
                btn.addEventListener('click', (e) => {
                    e.preventDefault();
                    const target = btn.closest('.folder-item');
                    if (target) {
                        target.classList.toggle('collapsed');
                        const icon = btn.querySelector('.tree-icon');
                        if (icon) {
                            icon.textContent = target.classList.contains('collapsed') ? '>' : 'v';
                        }
                    }
                });
            });

            // スクロール位置のパーセンテージ表示
            const scrollPosition = document.getElementById('scroll-position');
            const updateScrollPosition = () => {
                if (!scrollPosition) return;
                const scrollTop = window.scrollY;
                const docHeight = document.documentElement.scrollHeight - window.innerHeight;
                if (docHeight <= 0) {
                    scrollPosition.textContent = 'All';
                    return;
                }
                const percent = Math.round((scrollTop / docHeight) * 100);
                if (percent <= 0) {
                    scrollPosition.textContent = 'Top';
                } else if (percent >= 100) {
                    scrollPosition.textContent = 'Bot';
                } else {
                    scrollPosition.textContent = percent + '%';
                }
            };
            window.addEventListener('scroll', updateScrollPosition);
            updateScrollPosition();

            // noice.nvim風トースト通知
            const toastContainer = document.getElementById('toast-container');

            // 閉じるボタン用メッセージ
            const closeMessages = [
                { title: 'E32: No file name', message: 'バッファを保存してから終了してください...というのは冗談です', icon: '!' },
                { title: 'Nice try!', message: 'このウィンドウは閉じられません。Neovimではないので。', icon: '😏' },
                { title: ':q!', message: '本当に閉じたいなら :q! を入力してください（嘘です）', icon: '📝' },
                { title: 'SIGTERM rejected', message: 'dnfolioは永遠に動き続けます。たぶん。', icon: '🔄' },
                { title: 'Permission denied', message: 'sudo rm -rf / は実行しないでください', icon: '🚫' },
                { title: 'Process killed', message: '...と見せかけて生きています', icon: '💀' },
                { title: 'Segmentation fault', message: 'Core dumped...していません。Rustなので。', icon: '🦀' },
                { title: 'vim.notify()', message: '閉じるボタンはNeovimでは :q です', icon: '📢' },
            ];

            // 最小化ボタン用メッセージ
            const minimizeMessages = [
                { title: 'window.minimize is not a function', message: 'TypeError: ブラウザAPIにそんなメソッドはありません', icon: '🔧' },
                { title: 'CSS: display: none;', message: '本当に消えたら困るので実装しませんでした', icon: '👻' },
                { title: ':hide', message: 'Neovimならバッファを隠せますが、ここはWebです', icon: '📦' },
                { title: 'Uncaught ReferenceError', message: 'minimize is not defined (このボタンは飾りです)', icon: '🎀' },
                { title: 'npm install minimize', message: 'パッケージが見つかりません。そもそも存在しません。', icon: '📦' },
                { title: '418 I\'m a teapot', message: 'このボタンはティーポットなので最小化できません', icon: '🫖' },
                { title: 'git stash', message: '変更を退避しますか？...しません。飾りなので。', icon: '📁' },
                { title: 'cargo build --release', message: 'この機能はリリースビルドに含まれていません', icon: '🦀' },
            ];

            // 最大化ボタン用メッセージ
            const maximizeMessages = [
                { title: 'F11', message: 'フルスクリーンはブラウザにお任せしています（F11キー推奨）', icon: '⌨️' },
                { title: ':wincmd =', message: 'ウィンドウサイズの均等化はNeovimでどうぞ', icon: '📐' },
                { title: 'width: 100vw', message: 'すでに最大幅です。これ以上は物理的に無理です。', icon: '📏' },
                { title: 'Buffer overflow', message: '画面サイズがオーバーフローしました（嘘）', icon: '💥' },
                { title: 'sudo make me a sandwich', message: '権限があっても画面は大きくなりません', icon: '🥪' },
                { title: ':set columns=999', message: 'Neovimのコマンドはここでは使えません', icon: '🖥️' },
                { title: 'resize: none;', message: 'CSSでリサイズを禁止されています（本当は飾り）', icon: '🎨' },
                { title: 'panic!()', message: 'thread \'main\' panicked at \'not implemented\'', icon: '🦀' },
            ];

            const showToast = (title, message, icon, type = 'info') => {
                if (!toastContainer) return;
                const toast = document.createElement('div');
                toast.className = 'toast toast-' + type;

                // XSS対策: textContentを使用してDOM構築
                const iconSpan = document.createElement('span');
                iconSpan.className = 'toast-icon';
                iconSpan.textContent = icon;

                const contentDiv = document.createElement('div');
                contentDiv.className = 'toast-content';

                const titleDiv = document.createElement('div');
                titleDiv.className = 'toast-title';
                titleDiv.textContent = title;

                const messageDiv = document.createElement('div');
                messageDiv.className = 'toast-message';
                messageDiv.textContent = message;

                contentDiv.appendChild(titleDiv);
                contentDiv.appendChild(messageDiv);

                const closeBtn = document.createElement('button');
                closeBtn.className = 'toast-close';
                closeBtn.textContent = '×';

                toast.appendChild(iconSpan);
                toast.appendChild(contentDiv);
                toast.appendChild(closeBtn);
                toastContainer.appendChild(toast);

                const hideToast = () => {
                    toast.classList.add('hiding');
                    setTimeout(() => toast.remove(), 300);
                };
                closeBtn.addEventListener('click', hideToast);
                setTimeout(hideToast, 5000);
            };

            // ×ボタンのクリックイベント
            const btnClose = document.querySelector('.btn-close');
            if (btnClose) {
                btnClose.addEventListener('click', () => {
                    const msg = closeMessages[Math.floor(Math.random() * closeMessages.length)];
                    showToast(msg.title, msg.message, msg.icon, 'warn');
                });
            }

            // 最小化ボタンのクリックイベント
            const btnMinimize = document.querySelector('.btn-minimize');
            if (btnMinimize) {
                btnMinimize.addEventListener('click', () => {
                    const msg = minimizeMessages[Math.floor(Math.random() * minimizeMessages.length)];
                    showToast(msg.title, msg.message, msg.icon, 'info');
                });
            }

            // 最大化ボタンのクリックイベント
            const btnMaximize = document.querySelector('.btn-maximize');
            if (btnMaximize) {
                btnMaximize.addEventListener('click', () => {
                    const msg = maximizeMessages[Math.floor(Math.random() * maximizeMessages.length)];
                    showToast(msg.title, msg.message, msg.icon, 'info');
                });
            }

            // 行番号付き要素を取得
            const getLineElements = () => {
                const mainContent = document.querySelector('.main-content');
                if (!mainContent) return [];
                return Array.from(mainContent.querySelectorAll(':scope > h1, :scope > h2, :scope > h3, :scope > h4, :scope > p, :scope > ul, :scope > ol, :scope > blockquote, :scope > pre, :scope > table, :scope > hr'));
            };

            // 指定行にジャンプ
            const jumpToLine = (lineNum) => {
                const elements = getLineElements();
                if (lineNum < 1 || lineNum > elements.length) {
                    showToast('E20: Mark not set', `行 ${lineNum} は存在しません (1-${elements.length})`, '!', 'warn');
                    return false;
                }
                const target = elements[lineNum - 1];
                if (target) {
                    // 現在の行をハイライト
                    elements.forEach(el => el.classList.remove('current-line'));
                    target.classList.add('current-line');
                    target.scrollIntoView({ behavior: 'smooth', block: 'center' });
                    showToast(':' + lineNum, '行 ' + lineNum + ' にジャンプしました', '📍', 'info');
                    return true;
                }
                return false;
            };

            // 行番号クリックでジャンプ
            const setupLineNumberClicks = () => {
                const elements = getLineElements();
                elements.forEach((el, index) => {
                    el.addEventListener('click', (e) => {
                        // 行番号部分（::before）のクリック判定（左端50px）
                        const rect = el.getBoundingClientRect();
                        if (e.clientX < rect.left) {
                            elements.forEach(el => el.classList.remove('current-line'));
                            el.classList.add('current-line');
                        }
                    });
                });
            };
            setupLineNumberClicks();

            // ヘッダーアンカーリンク機能（記事ページのみ）
            const setupHeaderAnchors = () => {
                const mainContent = document.querySelector('.main-content.page-article');
                if (!mainContent) return;

                const headers = mainContent.querySelectorAll('h1[id], h2[id], h3[id], h4[id], h5[id], h6[id]');
                headers.forEach(header => {
                    // 見出しレベルに応じたマーカー
                    const level = parseInt(header.tagName.charAt(1), 10);
                    const marker = '#'.repeat(level);

                    // アンカーリンク要素を作成
                    const anchor = document.createElement('a');
                    anchor.className = 'header-anchor-link';
                    anchor.href = '#' + header.id;
                    anchor.textContent = marker;
                    anchor.title = 'セクションへのリンク';

                    // クリックでURLにハッシュを追加してナビゲーション
                    anchor.addEventListener('click', (e) => {
                        e.preventDefault();
                        // URLを更新
                        history.pushState(null, '', '#' + header.id);
                        // スクロール
                        header.scrollIntoView({ behavior: 'smooth', block: 'start' });
                    });

                    // ヘッダーの先頭に挿入
                    header.insertBefore(anchor, header.firstChild);
                });
            };
            setupHeaderAnchors();

            // コードブロックのコピー機能
            const setupCodeCopy = () => {
                const copyButtons = document.querySelectorAll('.code-copy-btn');
                copyButtons.forEach(btn => {
                    btn.addEventListener('click', async () => {
                        const code = btn.dataset.code
                            .replace(/&amp;/g, '&')
                            .replace(/&lt;/g, '<')
                            .replace(/&gt;/g, '>')
                            .replace(/&quot;/g, '"');

                        try {
                            await navigator.clipboard.writeText(code);
                            btn.classList.add('copied');
                            const textSpan = btn.querySelector('.copy-text');
                            const originalText = textSpan.textContent;
                            textSpan.textContent = 'Copied!';

                            setTimeout(() => {
                                btn.classList.remove('copied');
                                textSpan.textContent = originalText;
                            }, 2000);
                        } catch (err) {
                            console.error('Failed to copy:', err);
                        }
                    });
                });
            };
            setupCodeCopy();

            // OUTLINE スクロール追跡（Scroll Spy）
            const setupScrollSpy = () => {
                const tocSection = document.querySelector('.toc-section');
                if (!tocSection) return;

                const tocItems = tocSection.querySelectorAll('.toc-item');
                const mainContent = document.querySelector('.main-content');
                if (!mainContent || tocItems.length === 0) return;

                // 見出し要素を取得
                const headings = mainContent.querySelectorAll('h2[id], h3[id]');
                if (headings.length === 0) return;

                // 見出しIDとTOCアイテムのマッピング
                const tocMap = new Map();
                tocItems.forEach(item => {
                    const link = item.querySelector('a');
                    if (link) {
                        const id = link.getAttribute('href').replace('#', '');
                        tocMap.set(id, item);
                    }
                });

                // スクロール位置から現在の見出しを判定
                const updateActiveHeading = () => {
                    const scrollTop = window.scrollY;
                    const offset = 100; // タイトルバー + 余裕

                    let activeHeading = null;
                    headings.forEach(heading => {
                        const rect = heading.getBoundingClientRect();
                        const headingTop = rect.top + scrollTop;
                        // 見出しがスクロール位置を超えたらアクティブ候補
                        if (headingTop <= scrollTop + offset) {
                            activeHeading = heading;
                        }
                    });

                    // 最初の見出しより上にいる場合は最初をアクティブに
                    if (!activeHeading && headings.length > 0) {
                        activeHeading = headings[0];
                    }

                    if (activeHeading) {
                        const newActiveId = activeHeading.id;
                        // 前のアクティブを解除
                        tocItems.forEach(item => item.classList.remove('active'));
                        // 新しいアクティブを設定
                        const activeItem = tocMap.get(newActiveId);
                        if (activeItem) {
                            activeItem.classList.add('active');
                            // アクティブなアイテムが見えるようにスクロール
                            activeItem.scrollIntoView({ block: 'nearest', behavior: 'smooth' });
                        }
                    }
                };

                // スクロールイベントで更新（throttle付き）
                let ticking = false;
                window.addEventListener('scroll', () => {
                    if (!ticking) {
                        requestAnimationFrame(() => {
                            updateActiveHeading();
                            ticking = false;
                        });
                        ticking = true;
                    }
                });

                // 初期表示時にも実行
                updateActiveHeading();
            };
            setupScrollSpy();

            // Neovim風 : コマンド処理
            const executeVimCommand = (cmd) => {
                const trimmed = cmd.trim().toLowerCase();

                // 行番号ジャンプ :123
                const lineMatch = cmd.trim().match(/^:(\d+)$/);
                if (lineMatch) {
                    const lineNum = parseInt(lineMatch[1], 10);
                    jumpToLine(lineNum);
                    return true;
                }

                // コマンド定義
                const commands = {
                    ':q': () => showToast('E37: No write since last change', 'add ! to override (冗談です、ここはWebです)', '!', 'warn'),
                    ':q!': () => showToast('E37: No write since last change', '...だから、Webサイトなんですって', '!', 'warn'),
                    ':wq': () => showToast('Already saved', 'This is a static site. すでに保存済みです。', '💾', 'info'),
                    ':w': () => showToast(':w', 'Static siteなので保存する必要がありません', '📝', 'info'),
                    ':x': () => showToast(':x', ':wqと同じですが、ここはWebです', '📝', 'info'),
                    ':help': () => {
                        showToast('Help - Keybindings',
                            '/ or Ctrl+K: 検索\\ngg: ページトップ\\nG: ページボトム\\nn/N: 次/前のハイライト\\n:行番号 で行ジャンプ\\n:q :wq :help :version',
                            '❓', 'info');
                    },
                    ':h': () => commands[':help'](),
                    ':version': () => showToast('dnfolio ' + document.body.dataset.version, 'Built with Rust + maud\\nTheme: sakurajima.nvim\\nby Daiki Nakashima', '🦀', 'info'),
                    ':ver': () => commands[':version'](),
                    ':smile': () => showToast(':)', 'Have a nice day!', '😊', 'info'),
                    ':qa': () => showToast('E37: No write since last change', '全部閉じようとしても無駄です', '!', 'warn'),
                    ':qa!': () => showToast('Bye!', 'window.close()は動きませんけどね', '👋', 'info'),
                    ':set number': () => showToast(':set number', '行番号は既に表示されています！', '🔢', 'info'),
                    ':set nonumber': () => showToast(':set nonumber', '行番号を非表示にする機能はまだありません', '🔢', 'info'),
                    ':colorscheme': () => showToast(':colorscheme', '現在: sakurajima.nvim (変更不可)', '🎨', 'info'),
                    ':$': () => {
                        const elements = getLineElements();
                        if (elements.length > 0) jumpToLine(elements.length);
                    },
                };

                // コマンド実行
                if (commands[trimmed]) {
                    commands[trimmed]();
                    return true;
                }

                // 未知のコマンド
                if (trimmed.startsWith(':')) {
                    showToast('E492: Not an editor command', cmd.trim(), '!', 'warn');
                    return true;
                }

                return false;
            };

            // コマンドライン入力でEnter押下時の処理
            if (commandlineInput) {
                commandlineInput.addEventListener('keydown', (e) => {
                    if (e.key === 'Enter') {
                        const value = commandlineInput.value.trim();
                        if (value.startsWith(':')) {
                            executeVimCommand(value);
                            commandlineInput.value = '';
                            commandlineInput.blur();
                            commandlineInput.setAttribute('readonly', '');
                            e.preventDefault();
                        }
                    }
                });

                // : キーでコマンドモード開始
                commandlineInput.addEventListener('input', (e) => {
                    if (commandlineInput.value === ':') {
                        commandlineInput.removeAttribute('readonly');
                    }
                });
            }

            // gg/G モーション用の状態
            let lastKeyTime = 0;
            let lastKey = '';

            // グローバルキーイベント: gg/G モーション
            document.addEventListener('keydown', (e) => {
                // 入力欄では無効
                if (['INPUT', 'TEXTAREA'].includes(document.activeElement.tagName)) return;
                // 検索モーダルが開いている時は無効
                const searchModal = document.getElementById('search-modal');
                if (searchModal && searchModal.classList.contains('open')) return;

                const now = Date.now();

                // G (Shift+g) でページボトムへ
                if (e.key === 'G') {
                    window.scrollTo({ top: document.documentElement.scrollHeight, behavior: 'smooth' });
                    e.preventDefault();
                    return;
                }

                // gg でページトップへ（500ms以内に2回g）
                if (e.key === 'g') {
                    if (lastKey === 'g' && now - lastKeyTime < 500) {
                        window.scrollTo({ top: 0, behavior: 'smooth' });
                        lastKey = '';
                        e.preventDefault();
                    } else {
                        lastKey = 'g';
                        lastKeyTime = now;
                    }
                    return;
                }

                // : キーでコマンドラインにフォーカス
                if (e.key === ':') {
                    if (commandlineInput) {
                        commandlineInput.value = ':';
                        commandlineInput.removeAttribute('readonly');
                        commandlineInput.focus();
                        e.preventDefault();
                    }
                    return;
                }

                // 他のキーが押されたらggシーケンスをリセット
                lastKey = '';
            });
        });
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
                title { (config.base.page_title) }
                link rel="canonical" href=(config.base.canonical_url);
                meta name="description" content=(description);
                meta name="keywords" content=(keywords);
                meta name="author" content="Daiki Nakashima";

                // Google Fonts（JetBrains Mono, Inter, Noto Sans JP）
                link rel="preconnect" href="https://fonts.googleapis.com";
                link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous";
                link href="https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@400;600;700&family=Inter:wght@400;600&family=Noto+Sans+JP:wght@400;700&display=swap" rel="stylesheet";

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
                link rel="alternate" type="application/rss+xml" title="dnfolio" href="/feed.xml";

                @if let Some(json_ld) = config.base.structured_data_html {
                    (PreEscaped(json_ld))
                }

                style { (PreEscaped(css)) }
            }
            body data-version=(GIT_VERSION) {
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
                                span class="tab-icon" { "📄" }
                                span { (tab_filename) }
                                span class="tab-close" { "×" }
                            }
                        }

                        // エディターコンテンツ
                        div class="editor-wrapper" {
                            // 行番号は非表示（必要に応じてJSで動的に追加可能）
                            main class=(format!("main-content {}", if file_type == "markdown" { "page-article" } else { "page-home" })) {
                                (main_content_markup)
                            }
                        }
                    }
                }

                // トースト通知コンテナ
                div id="toast-container" class="toast-container" {}

                // モバイル用ハイライトナビゲーション
                div id="highlight-nav" class="highlight-nav" {
                    div id="highlight-nav-count" class="highlight-nav-count" {}
                    div class="highlight-nav-buttons" {
                        button id="highlight-nav-prev" class="highlight-nav-btn" title="前へ (N)" { "↑" }
                        button id="highlight-nav-next" class="highlight-nav-btn" title="次へ (n)" { "↓" }
                    }
                }

                // 固定フッター（ステータスライン + コマンドライン）
                div class="footer-fixed" {
                    // ステータスライン（左右分離）
                    footer class="statusline" {
                        div class="statusline-left" {
                            span class="statusline-mode" { "NORMAL" }
                            span class="statusline-section statusline-branch" { "main" }
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
                        input type="text" id="commandline-input" placeholder="Press / or Ctrl+K to search..." autocomplete="off" readonly;
                        span class="commandline-cursor" { "_" }
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

                script { (PreEscaped(js)) }
                script src="/search.js" defer {}

                script async src="https://www.googletagmanager.com/gtag/js?id=G-S0DTM6WBVT" {}
                script {
                    (PreEscaped(r#"
                        window.dataLayer = window.dataLayer || [];
                        function gtag(){dataLayer.push(arguments);}
                        gtag('js', new Date());
                        gtag('config', 'G-S0DTM6WBVT');
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
