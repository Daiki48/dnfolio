use maud::{DOCTYPE, Markup, html};

pub fn layout(
    page_title: &str,
    sidebar_left_markup: Markup,
    main_content_markup: Markup,
    sidebar_right_markup: Markup,
) -> Markup {
    html! {
        (DOCTYPE)
        html lang="ja" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { (page_title) }
                style {
                    "body { font-family: sans-serif; margin: 0; display: flex; flex-direction: column; min-height: 100vh; lihe-height: 20px; }"
                    "header, footer { background-color: #f0f0f0; padding: 1em; text-align: center; }"
                    ".container { display: flex; flex: 1; align-items: flex-start; }"

                    ".sidebar-left {
                        flex: 0 0 250px; 
                        background-color: #e0e0e0; 
                        padding: 1em; 
                        position: sticky;
                        top: 0;
                        align-self: flex-start;
                        height: 100vh;
                        overflow-y: auto;
                    }"

                    ".main-content { flex: 1; padding: 1em; }"

                    ".sidebar-right {
                        flex: 0 0 200px; 
                        background-color: #f5f5f5; 
                        padding: 1em; 
                        position: sticky;
                        top: 0;
                        align-self: flex-start;
                        height: 100vh;
                        overflow-y: auto;
                    }"

                    "ul { list-style: none; padding: 4px 0; }"
                    "li { margin-bottom: 0.5em; }"
                    "a { text-decoration: none; color: blue; }"
                    "a:hover { text-decoration: underline; }"
                    "p { margin: 4px 0; }"
                    "code {
                        font-family: 'SFMono-Regular', Consolas, 'Liberation Mono', Menlo, Courier, monospace;
                        background-color: #eee;
                        color: #c41a16;
                        padding: 0.2em 0.4em;
                        margin: 0 0.1em;
                        font-size: 85%;
                        border-radius: 3px;
                    }"
                    "pre {
                        background-color: #2d2d2d;
                        color: #f8f8f2;
                        font-family: 'SFMono-Regular', Consolas, 'Liberation Mono', Menlo, Courier, monospace;
                        padding: 1em;
                        margin: 1.5em 0;
                        overflow-x: auto;
                        border-radius: 6px;
                    }"
                    "pre code {
                        background: transparent;
                        color: inherit;
                        font-size: inherit;
                        padding: 0;
                        margin: 0;
                    }"
                    "blockquote {
                        background-color: #f9f9f9;
                        border-left: 10px solid #ccc;
                        margin: 1.5em 10px;
                        padding: 0.5em 10px;
                        color: #666;
                    }"
                    "blockquote p {
                        margin: 0;
                    }"
                }
            }
            body {
                header {
                    h1 {
                        a href="/" { "dnfolio" }
                    }
                }

                .container {
                    aside class="sidebar-left" { (sidebar_left_markup) }
                    main class="main-content" { (main_content_markup) }
                    aside class="sidebar-right" { (sidebar_right_markup) }
                }
            }
            footer {
                span { "© 2024 - 2025" }
                a href="/" { "dnfolio" }
                span { " " }
                a href="/privacy.html" target="_blank" { "Privacy Policy" }
            }
        }
    }
}
