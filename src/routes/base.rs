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
                        "body { font-family: sans-serif; margin: 0; display: flex; flex-direction: column; min-height: 100vh; }"
                        "header, footer { background-color: #f0f0f0; padding: 1em; text-align: center; }"
                        ".container { display: flex; flex: 1; }"
                        ".sidebar-left { flex: 0 0 250px; background-color: #e0e0e0; padding: 1em; overflow-y: auto; }"
                        ".main-content { flex: 1; padding: 1em; overflow-y: auto; }"
                        ".sidebar-right { flex: 0 0 200px; background-color: #f5f5f5; padding: 1em; overflow-y: auto; }"
                        "ul { list-style: none; padding: 0; }"
                        "li { margin-bottom: 0.5em; }"
                        "a { text-decoration: none; color: blue; }"
                        "a:hover { text-decoration: underline; }"
                    }
                }
                body {
                    header {
                        h1 {
                            a href="/" {
                                "dnfolio"
                            }
                        }
                    }

                    .container {
                        aside class="sidebar-left" {
                            (sidebar_left_markup)
                        }
                        main class="main-content" {
                            (main_content_markup)
                        }
                        aside class="sidebar-right" {
                            (sidebar_right_markup)
                        }
                    }
                }
                footer {
                    p { "this is footer" }
                }
            }
    }
}
