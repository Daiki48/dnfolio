use crate::metadata::MetaData;
use maud::{DOCTYPE, Markup, PreEscaped, html};

pub fn layout(
    page_title: &str,
    metadata: Option<&MetaData>,
    ogp_image_path: Option<&str>,
    sidebar_left_markup: Markup,
    main_content_markup: Markup,
    sidebar_right_markup: Markup,
) -> Markup {
    let description = metadata
        .and_then(|m| m.description.as_ref())
        .map(|d| d.as_str())
        .unwrap_or("プログラムを良く書く人の個人サイトです。");

    let keywords = metadata
        .and_then(|m| m.taxonomies.as_ref())
        .and_then(|t| t.tags.as_ref())
        .map(|tags| tags.join(", "))
        .unwrap_or_else(|| "ポエム".to_string());

    let css = r#"
        :root { --header-height: 60px; }
        *, *::before, *::after { box-sizing: border-box; }
        html { font-size: 16px; scroll-padding-top: var(--header-height); }
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', 'Oxygen', 'Ubuntu', 'Cantarell', 'Fira Sans', 'Droid Sans', 'Helvetica Neue', sans-serif;
            margin: 0;
            display: flex;
            flex-direction: column;
            min-height: 100vh;
            line-height: 1.7;
            background-color: #fff;
            color: #333;
            overflow-x: hidden;
        }
        header {
            background-color: #f8f9fa;
            padding: 0 1em;
            text-align: center;
            border-bottom: 1px solid #dee2e6;
            position: sticky;
            top: 0;
            z-index: 900;
            height: var(--header-height);
        }
        .header-content {
            display: flex;
            justify-content: center;
            align-items: center;
            max-width: 1400px;
            margin: 0 auto;
            height: 100%;
        }
        header h1 { font-size: 1.5rem; margin: 0; }
        footer {
            background-color: #f8f9fa;
            border-top: 1px solid #dee2e6;
            padding: 2em 1em;
            text-align: center;
            font-size: 0.9rem;
            color: #6c757d;
        }
        .container {
            display: flex;
            flex: 1;
            align-items: flex-start;
            max-width: 1400px;
            width: 100%;
            margin: 0 auto;
        }
        .sidebar-left {
            flex: 0 0 250px;
            padding: 1.5em;
            position: sticky;
            top: var(--header-height);
            height: calc(100vh - var(--header-height));
            overflow-y: auto;
            background-color: #fff;
            border-right: 1px solid #dee2e6;
        }
        .main-content {
            flex: 1;
            padding: 1.5em;
            min-width: 0;
            overflow-wrap: break-word;
        }
        .main-content img {
            max-width: 100%;
            height: auto;
        }
        .sidebar-right {
            flex: 0 0 220px;
            padding: 1.5em;
            position: sticky;
            top: var(--header-height);
            height: calc(100vh - var(--header-height));
            overflow-y: auto;
            background-color: #f8f9fa;
            border-left: 1px solid #dee2e6;
            font-size: 0.9rem;
        }
        ul { list-style: none; padding: 0; margin: 0; }
        li { margin-bottom: 0.5em; }
        a {
            text-decoration: none;
            color: #007bff;
            overflow-wrap: break-word;
        }
        a:hover { text-decoration: underline; }
        h1, h2, h3, h4, h5, h6 { margin-top: 1.5em; margin-bottom: 0.5em; line-height: 1.3; }
        p {
            margin: 0 0 1em;
            overflow-wrap: break-word;
        }
        code {
            font-family: 'SFMono-Regular', Consolas, 'Liberation Mono', Menlo, Courier, monospace;
            background-color: #e9ecef;
            color: #c41a16;
            padding: 0.2em 0.4em;
            margin: 0 0.1em;
            font-size: 85%;
            border-radius: 3px;
            word-break: break-all;
        }
        pre {
            background-color: #2d2d2d;
            color: #f8f8f2;
            font-family: 'SFMono-Regular', Consolas, 'Liberation Mono', Menlo, Courier, monospace;
            padding: 1em;
            margin: 1.5em 0;
            overflow-x: auto;
            border-radius: 6px;
            font-size: 0.9rem;
            max-width: 100%;
        }
        pre code {
            background: transparent;
            color: inherit;
            font-size: inherit;
            padding: 0;
            margin: 0;
            word-break: normal;
        }
        blockquote {
            background-color: #f9f9f9;
            border-left: 5px solid #ccc;
            margin: 1.5em 0;
            padding: 1em 1.5em;
            color: #666;
            overflow-wrap: break-word;
        }
        blockquote p:last-child { margin-bottom: 0; }
        .hamburger-btn, .overlay { display: none; }

        /* --- Tablet & Mobile Styles --- */
        @media screen and (max-width: 992px) {
            body { padding: 0; }
            .header-content { justify-content: space-between; }
            .container { flex-direction: column; }
            .main-content {
                order: 2;
                width: 100%;
                padding: 1.5em 1em;
            }
            .sidebar-right {
                order: 1;
                width: 100%;
                position: static;
                height: auto;
                border-left: none;
                border-bottom: 1px solid #dee2e6;
                background-color: #fff;
                padding: 1em;
            }
            .sidebar-left {
                order: 3;
                position: fixed;
                left: 0;
                top: 0;
                height: 100%;
                width: 280px;
                background-color: #fff;
                z-index: 1000;
                transform: translateX(-100%);
                transition: transform 0.3s ease-in-out;
                border-right: 1px solid #dee2e6;
                padding-top: calc(var(--header-height) + 1em);
            }
            .sidebar-left.is-open { transform: translateX(0); }
            .hamburger-btn {
                display: flex;
                flex-direction: column;
                justify-content: space-around;
                width: 30px;
                height: 24px;
                background: transparent;
                border: none;
                cursor: pointer;
                padding: 0;
                z-index: 1001;
            }
            .hamburger-btn span {
                width: 100%;
                height: 3px;
                background-color: #333;
                border-radius: 2px;
                transition: all 0.3s linear;
            }
            .overlay {
                position: fixed;
                top: 0;
                left: 0;
                width: 100%;
                height: 100%;
                background: rgba(0, 0, 0, 0.5);
                z-index: 999;
            }
            .overlay.is-open { display: block; }
        }
    "#;

    let js = r#"
        document.addEventListener('DOMContentLoaded', () => {
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
        });
    "#;

    html! {
        (DOCTYPE)
        html lang="ja" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { (page_title) }
                meta name="description" content=(description);
                meta name="keywords" content=(keywords);
                meta name="author" content="Daiki Nakashima";

                meta property="og:title" content=(page_title);
                meta property="og:description" content=(description);
                meta property="og:type" content="website";
                meta property="og:site_name" content="dnfolio";
                @if let Some(image_path) = ogp_image_path {
                    meta property="og:image" content=(format!("https://dnfolio.me{image_path}"));
                    meta property="og:image:width" content="1200";
                    meta property="og:image:height" content="630";
                    meta property="og:image:type" content="image/svg+xml";
                }

                meta name="twitter:card" content="summary";
                meta name="twitter:title" content=(page_title);
                meta name="twitter:description" content=(description);
                @if let Some(image_path) = ogp_image_path {
                    meta name="twitter:image" content=(format!("https://dnfolio.me{image_path}"));
                }

                style { (PreEscaped(css)) }
            }
            body {
                header {
                    div class="header-content" {
                        button id="hamburger-btn" class="hamburger-btn" aria-label="Menu" {
                            span {}
                            span {}
                            span {}
                        }
                        h1 {
                            a href="/" { "dnfolio" }
                        }
                        div style="width: 30px;" {}
                    }
                }

                div id="overlay" class="overlay" {}

                div class="container" {
                    aside id="sidebar-left" class="sidebar-left" { (sidebar_left_markup) }
                    main class="main-content" { (main_content_markup) }
                    aside class="sidebar-right" { (sidebar_right_markup) }
                }

                footer {
                    span { "© 2024 - 2025 " }
                    a href="/" { "dnfolio" }
                    span { " | " }
                    a href="/privacy.html" target="_blank" { "Privacy Policy" }
                }

                script { (PreEscaped(js)) }
            }
        }
    }
}
