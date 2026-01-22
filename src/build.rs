use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock};

use anyhow::Result;
use chrono::{Datelike, NaiveDate};
use gray_matter::{Matter, ParsedEntity};
use maud::{Markup, html};
use pulldown_cmark::{CowStr, Event, Parser, Tag, TagEnd};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use resvg::usvg::{self, fontdb};
use slug::slugify;
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;
use walkdir::WalkDir;

use crate::models::{Article, Heading, MetaData, Page, TagInfo};
use crate::templates::base::{ArticlePageConfig, PageConfig};
use crate::templates::{base, privacy};
use crate::{ogp, rss, sitemap, structured_data};

// 年月別グループ化のためのヘルパー構造
struct YearGroup {
    year: i32,
    months: Vec<MonthGroup>,
}

struct MonthGroup {
    month: u32,
    articles: Vec<Article>,
}

// 記事を年月別にグループ化
fn group_articles_by_year_month(articles: &[Article]) -> Vec<YearGroup> {
    let mut year_map: HashMap<i32, HashMap<u32, Vec<Article>>> = HashMap::new();

    for article in articles {
        if let Some(date) = extract_date_from_filename(&article.source_path) {
            let year = date.year();
            let month = date.month();
            year_map
                .entry(year)
                .or_default()
                .entry(month)
                .or_default()
                .push(article.clone());
        }
    }

    let mut years: Vec<YearGroup> = year_map
        .into_iter()
        .map(|(year, month_map)| {
            let mut months: Vec<MonthGroup> = month_map
                .into_iter()
                .map(|(month, articles)| MonthGroup { month, articles })
                .collect();
            months.sort_by(|a, b| b.month.cmp(&a.month));
            YearGroup { year, months }
        })
        .collect();

    years.sort_by(|a, b| b.year.cmp(&a.year));
    years
}

// Neovim風ファイルツリー形式の記事一覧を生成
fn generate_file_tree_markup(
    year_groups: &[YearGroup],
    current_article_url: Option<&str>,
    toc_html: Option<&str>,
) -> Markup {
    html! {
        ul {
            @for year_group in year_groups {
                li class="folder-item" {
                    span class="file-tree-item folder-toggle folder-year" {
                        span class="tree-icon" { "v" }
                        (format!("{}", year_group.year))
                    }
                    ul {
                        @for month_group in &year_group.months {
                            li class="folder-item" {
                                span class="file-tree-item folder-toggle folder-month" {
                                    span class="tree-icon" { "v" }
                                    (format!("{:02}", month_group.month))
                                }
                                ul {
                                    @for article in &month_group.articles {
                                        @let article_url = article.relative_url.to_string_lossy().to_string();
                                        @let is_current = current_article_url.map(|u| u == article_url.as_str()).unwrap_or(false);
                                        @let class_name = if is_current { "file-tree-item current" } else { "file-tree-item" };
                                        @let tree_icon = if is_current { "v" } else { "-" };
                                        li {
                                            a href=(article_url) class=(class_name) {
                                                span class="tree-icon" { (tree_icon) }
                                                @if let Some(meta) = &article.metadata {
                                                    @let title_display: String = meta.title.chars().take(25).collect();
                                                    (title_display)
                                                    @if meta.title.chars().count() > 25 { "..." }
                                                } @else {
                                                    (article.output_path.file_name().unwrap_or_default().to_string_lossy())
                                                }
                                            }
                                            // 現在の記事の場合は目次を展開
                                            @if is_current && toc_html.is_some() {
                                                div class="toc-expanded" {
                                                    (maud::PreEscaped(toc_html.unwrap()))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

static SYNTAX_SET: OnceLock<SyntaxSet> = OnceLock::new();
static THEME_SET: OnceLock<ThemeSet> = OnceLock::new();

fn get_syntax_set() -> &'static SyntaxSet {
    SYNTAX_SET.get_or_init(SyntaxSet::load_defaults_newlines)
}

fn get_theme_set() -> &'static ThemeSet {
    THEME_SET.get_or_init(ThemeSet::load_defaults)
}

fn highlight_code(lang: &str, code: &str) -> String {
    let ss = get_syntax_set();
    let ts = get_theme_set();
    let theme = &ts.themes["base16-ocean.dark"];

    // 言語を検索、見つからなければPlainTextにフォールバック
    let syntax = ss
        .find_syntax_by_token(lang)
        .unwrap_or_else(|| ss.find_syntax_plain_text());

    // 言語表示名（空の場合は"text"）
    let display_lang = if lang.is_empty() { "text" } else { lang };

    // コードをdata属性用にエスケープ
    let escaped_code = code
        .replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
        .replace('>', "&gt;");

    let highlighted_html = match highlighted_html_for_string(code, ss, syntax, theme) {
        Ok(html) => html,
        Err(_) => {
            // エラー時はエスケープしてそのまま表示
            format!(
                "<pre><code>{}</code></pre>",
                code.replace('&', "&amp;")
                    .replace('<', "&lt;")
                    .replace('>', "&gt;")
            )
        }
    };

    // ヘッダーバー付きのコードブロックを生成
    format!(
        r#"<div class="code-block-wrapper">
<div class="code-block-header">
<span class="code-lang">{}</span>
<button class="code-copy-btn" data-code="{}" title="コピー">
<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
<rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect>
<path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path>
</svg>
<span class="copy-text">Copy</span>
</button>
</div>
{}</div>"#,
        display_lang, escaped_code, highlighted_html
    )
}

fn extract_date_from_filename(path: &Path) -> Option<NaiveDate> {
    let file_name = path.file_stem()?.to_str()?;

    if let Some(date_part) = file_name.split('_').next() {
        if date_part.len() >= 10 {
            let date_str = &date_part[0..10];
            NaiveDate::parse_from_str(date_str, "%Y-%m-%d").ok()
        } else {
            None
        }
    } else {
        None
    }
}

fn parse_markdown_file(input_path: &Path, dist_dir: &Path) -> anyhow::Result<Article> {
    let markdown_with_metadata = fs::read_to_string(input_path)?;

    let mut matter = Matter::<gray_matter::engine::TOML>::new();
    matter.delimiter = "+++".to_string();
    matter.close_delimiter = Some("+++".to_string());
    let parsed_matter: ParsedEntity<MetaData> =
        matter.parse::<MetaData>(&markdown_with_metadata)?;

    let metadata: Option<MetaData> = parsed_matter.data;

    if let Some(meta) = &metadata {
        println!("Meta Data for {input_path:?}: Title = {}", meta.title);

        if let Some(true) = meta.draft {
            println!("Article {input_path:?} is not draft. Skipping HTML generation.");
            return Err(anyhow::Error::msg("Draft article skipped"));
        }
    } else {
        println!("No metadata found for {input_path:?}.");
        eprintln!(
            "DEBUG: Failed to parse metadata for {input_path:?}. Raw matter content (if any): {:?}",
            parsed_matter.matter
        );
    }

    let markdown_content = parsed_matter.content;

    let mut pulldown_options = pulldown_cmark::Options::empty();
    pulldown_options.insert(pulldown_cmark::Options::ENABLE_TABLES);
    pulldown_options.insert(pulldown_cmark::Options::ENABLE_FOOTNOTES);
    pulldown_options.insert(pulldown_cmark::Options::ENABLE_STRIKETHROUGH);
    pulldown_options.insert(pulldown_cmark::Options::ENABLE_TASKLISTS);
    pulldown_options.insert(pulldown_cmark::Options::ENABLE_SMART_PUNCTUATION);
    pulldown_options.insert(pulldown_cmark::Options::ENABLE_HEADING_ATTRIBUTES);

    let parser = Parser::new_ext(&markdown_content, pulldown_options);
    let mut headings: Vec<Heading> = Vec::new();
    let mut id_counts: HashMap<String, usize> = HashMap::new();
    let mut html_output = String::new();

    // 検索用ブロック要素のトラッキング（DOMの行番号と対応）
    let mut content_blocks: Vec<crate::models::ContentBlock> = Vec::new();
    let mut current_block_text = String::new();
    let mut block_line_num: usize = 0;
    let mut in_block = false;
    let mut block_depth = 0; // ネストしたブロック要素（ul内のliなど）のトラッキング

    let mut current_heading_text_buffer = String::new();
    let mut is_in_heading = false;
    let mut processed_events: Vec<Event> = Vec::new();

    let mut in_code_block = false;
    let mut code_block_lang = String::new();
    let mut code_block_content = String::new();

    for event in parser {
        // ブロック要素のテキスト収集
        match &event {
            Event::Text(text) | Event::Code(text) => {
                if in_block {
                    current_block_text.push_str(text);
                    current_block_text.push(' ');
                }
            }
            _ => {}
        }

        // ブロック要素の開始・終了をトラッキング
        // CSSカウンター対象: h1-h4, p, ul, ol, blockquote, pre, table, hr
        match &event {
            Event::Start(Tag::Heading { .. })
            | Event::Start(Tag::Paragraph)
            | Event::Start(Tag::List(_))
            | Event::Start(Tag::BlockQuote(_))
            | Event::Start(Tag::CodeBlock(_))
            | Event::Start(Tag::Table(_)) => {
                if block_depth == 0 {
                    block_line_num += 1;
                    current_block_text.clear();
                    in_block = true;
                }
                block_depth += 1;
            }
            Event::End(TagEnd::Heading(_))
            | Event::End(TagEnd::Paragraph)
            | Event::End(TagEnd::List(_))
            | Event::End(TagEnd::BlockQuote(_))
            | Event::End(TagEnd::CodeBlock)
            | Event::End(TagEnd::Table) => {
                block_depth -= 1;
                if block_depth == 0 && in_block {
                    let text = current_block_text.trim().to_string();
                    if !text.is_empty() {
                        content_blocks.push(crate::models::ContentBlock {
                            line_num: block_line_num,
                            text,
                        });
                    }
                    in_block = false;
                }
            }
            Event::Rule => {
                // hr要素もカウント（テキストなし）
                block_line_num += 1;
            }
            _ => {}
        }

        match event {
            Event::Start(Tag::Heading {
                level,
                id,
                classes,
                attrs,
            }) => {
                is_in_heading = true;
                current_heading_text_buffer.clear();
                processed_events.push(Event::Start(Tag::Heading {
                    level,
                    id,
                    classes,
                    attrs,
                }));
            }
            Event::End(TagEnd::Heading(level)) => {
                is_in_heading = false;
                let text_content = current_heading_text_buffer.trim().to_string();
                let mut final_id_to_use: Option<CowStr> = None;

                for i in (0..processed_events.len()).rev() {
                    if let Event::Start(Tag::Heading {
                        level: h_level,
                        id: existing_id_in_event,
                        ..
                    }) = &processed_events[i]
                        && *h_level == level
                    {
                        if existing_id_in_event.is_some() {
                            final_id_to_use = existing_id_in_event.clone();
                            break;
                        } else {
                            break;
                        }
                    }
                }

                let id_string: String;
                if let Some(cow_id) = final_id_to_use {
                    id_string = cow_id.to_string();
                } else {
                    let base_id = slugify(&text_content);
                    let mut id_candidate = base_id.clone();
                    let mut counter = *id_counts.get(&base_id).unwrap_or(&0);
                    while headings.iter().any(|h| h.id == id_candidate) {
                        counter += 1;
                        id_candidate = format!("{base_id}-{counter}");
                    }
                    id_counts.insert(base_id, counter);
                    id_string = id_candidate;
                }

                headings.push(Heading {
                    level: level as u8,
                    id: id_string.clone(),
                    text: text_content.clone(),
                });
                let mut found_start = false;
                for i in (0..processed_events.len()).rev() {
                    if let Event::Start(Tag::Heading {
                        level: h_level,
                        id: h_id,
                        ..
                    }) = &mut processed_events[i]
                        && *h_level == level
                        && h_id.is_none()
                    {
                        *h_id = Some(CowStr::from(id_string.clone()));
                        found_start = true;
                        break;
                    }
                }
                if !found_start {
                    eprintln!(
                        "Warning: Could not find matching Start(Heading) event to assign ID for heading: {text_content:?}"
                    );
                }
                processed_events.push(Event::End(
                    Tag::Heading {
                        level,
                        id: Some(CowStr::from(id_string)),
                        classes: Vec::new(),
                        attrs: Vec::new(),
                    }
                    .into(),
                ));
            }
            Event::Text(text) => {
                if in_code_block {
                    code_block_content.push_str(&text);
                } else {
                    if is_in_heading {
                        current_heading_text_buffer.push_str(&text);
                    }
                    processed_events.push(Event::Text(text));
                }
            }
            Event::Code(text) => {
                if is_in_heading {
                    current_heading_text_buffer.push_str(&text);
                }
                processed_events.push(Event::Code(text));
            }
            Event::Start(Tag::CodeBlock(kind)) => {
                in_code_block = true;
                code_block_lang = match kind {
                    pulldown_cmark::CodeBlockKind::Fenced(lang) => lang.to_string(),
                    pulldown_cmark::CodeBlockKind::Indented => String::new(),
                };
                code_block_content.clear();
            }
            Event::End(TagEnd::CodeBlock) => {
                in_code_block = false;
                let highlighted = highlight_code(&code_block_lang, &code_block_content);
                processed_events.push(Event::Html(CowStr::from(highlighted)));
            }
            other => {
                processed_events.push(other);
            }
        }
    }

    pulldown_cmark::html::push_html(&mut html_output, processed_events.into_iter());

    let toc_markup = html! {
        div class="toc-header" {
            span class="toc-icon" { "≡" }
            span { "OUTLINE" }
        }
        ul class="toc-list" {
            @for heading in &headings {
                @if heading.level == 2 {
                    li class="toc-item toc-h2" { a href=(format!("#{}", heading.id)) { (heading.text) } }
                } @else if heading.level == 3 {
                    li class="toc-item toc-h3" { a href=(format!("#{}", heading.id)) { (heading.text) } }
                }
            }
        }
    };

    let table_of_contents_html = toc_markup.into_string();

    // println!("\n--- HTML Output with IDs for {input_path:?} ---\n{html_output}");
    // println!("\n=============================================================\n");

    let file_stem = input_path.file_stem().unwrap().to_string_lossy();
    let article_slug = metadata
        .as_ref()
        .and_then(|m| m.slug.as_ref())
        .map(|s| s.to_string())
        .unwrap_or_else(|| {
            let name_part = file_stem.split('_').skip(1).collect::<Vec<_>>().join("-");
            if name_part.is_empty() {
                slugify(&file_stem)
            } else {
                slugify(&name_part)
            }
        });

    // dist/posts/{slug}/index.html と出力される
    let output_path = dist_dir
        .join("posts")
        .join(&article_slug)
        .join("index.html");
    let relative_url = PathBuf::from("/posts").join(&article_slug).join("");

    Ok(Article {
        metadata,
        content_html: html_output,
        content_blocks,
        output_path,
        relative_url,
        table_of_contents_html,
        source_path: input_path.to_path_buf(),
    })
}

fn parse_page_file(input_path: &Path, _pages_dir: &Path, dist_dir: &Path) -> anyhow::Result<Page> {
    let markdown_content = fs::read_to_string(input_path)?;

    let mut pulldown_options = pulldown_cmark::Options::empty();
    pulldown_options.insert(pulldown_cmark::Options::ENABLE_TABLES);
    pulldown_options.insert(pulldown_cmark::Options::ENABLE_FOOTNOTES);
    pulldown_options.insert(pulldown_cmark::Options::ENABLE_STRIKETHROUGH);
    pulldown_options.insert(pulldown_cmark::Options::ENABLE_TASKLISTS);
    pulldown_options.insert(pulldown_cmark::Options::ENABLE_SMART_PUNCTUATION);
    pulldown_options.insert(pulldown_cmark::Options::ENABLE_HEADING_ATTRIBUTES);

    let parser = Parser::new_ext(&markdown_content, pulldown_options);
    let mut html_content = String::new();
    pulldown_cmark::html::push_html(&mut html_content, parser);

    let file_stem = input_path
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .to_string();
    let output_path = dist_dir.join(&file_stem).join("index.html");
    let relative_url = PathBuf::from(format!("/{file_stem}/"));

    Ok(Page {
        content_html: html_content,
        output_path,
        relative_url,
        filename: file_stem,
    })
}

fn language_display_name(language: &str) -> &str {
    match language {
        "en" => "English",
        "ja" => "日本語",
        _ => language,
    }
}

fn collect_tags(articles: &[Article]) -> HashMap<String, TagInfo> {
    let mut tag_map: HashMap<String, TagInfo> = HashMap::new();

    for article in articles {
        if let Some(metadata) = &article.metadata
            && let Some(taxonomies) = &metadata.taxonomies
            && let Some(tags) = &taxonomies.tags
        {
            for tag in tags {
                let tag_info = tag_map.entry(tag.clone()).or_insert_with(|| TagInfo {
                    name: tag.clone(),
                    count: 0,
                    articles: Vec::new(),
                });
                tag_info.count += 1;
                tag_info.articles.push(article.clone());
            }
        }
    }
    tag_map
}

fn generate_tag_pages(
    tag_map: &HashMap<String, TagInfo>,
    dist_dir: &Path,
    articles_list_markup: &Markup,
) -> Result<()> {
    let tags_dir = dist_dir.join("tags");
    fs::create_dir_all(&tags_dir)?;

    for (tag_name, tag_info) in tag_map {
        let tag_slug = slugify(tag_name);
        let tag_page_dir = tags_dir.join(&tag_slug);
        fs::create_dir_all(&tag_page_dir)?;
        let tag_page_path = tag_page_dir.join("index.html");

        let tag_main_content_markup = html! {
            h1 { "タグ: " (tag_name) "(" (tag_info.count) "件)" }
            ul {
                @for article in &tag_info.articles {
                    li {
                        a href=(article.relative_url.to_string_lossy().to_string()) {
                            @if let Some(meta) = &article.metadata {
                                (meta.title)
                            } @else {
                                (article.output_path.file_name().unwrap_or_default().to_string_lossy())
                            }
                        }
                        @if let Some(meta) = &article.metadata &&  let Some(ref taxonomies) = meta.taxonomies && let Some(ref tags) = taxonomies.tags {
                            " - "
                                @for (i, tag) in tags.iter().enumerate() {
                                    @if i > 0 { ", " }
                                    span style="font-size: 0.9em; color: #666;" { (tag) }
                                }
                        }
                    }
                }
            }
        };

        let tag_sidebar_right_markup = html! {
            h2 { "サイト情報" }
            ul {
                li {
                    a href="/" { "ホームに戻る" }
                }
            }
        };

        let tag_canonical_url = format!("https://dnfolio.me/tags/{tag_slug}/");

        let tag_url = format!("/tags/{}/", tag_slug);
        let structured_data = structured_data::generate_structured_data_html(
            structured_data::PageType::TagPage {
                tag_name,
                url: &tag_url,
            },
            None,
        );

        let tag_html_output = base::layout(
            PageConfig {
                page_title: &format!("タグ: {tag_name}"),
                canonical_url: &tag_canonical_url,
                metadata: None,
                ogp_image_path: None,
                structured_data_html: Some(&structured_data),
            },
            articles_list_markup.clone(),
            tag_main_content_markup,
            tag_sidebar_right_markup,
        )
        .into_string();
        fs::write(tag_page_path, tag_html_output)?;
    }
    Ok(())
}

fn generate_search_js() -> String {
    r#"
// snacks.nvim grep風検索
let searchIndex = [];
let searchModal = null;
let searchInput = null;
let resultsList = null;
let previewPane = null;
let resultsCount = null;
let modeIndicator = null;
let selectedIndex = 0;
let searchResults = [];
let currentMode = 'insert'; // 'insert' or 'normal'

async function loadSearchIndex() {
    try {
        const response = await fetch('/search-index.json');
        if (response.ok) {
            searchIndex = await response.json();
        }
    } catch (e) {
        console.error('Failed to load search index:', e);
    }
}

function escapeHtml(text) {
    return text
        .replace(/&/g, '&amp;')
        .replace(/</g, '&lt;')
        .replace(/>/g, '&gt;')
        .replace(/"/g, '&quot;');
}

function highlightMatch(text, query) {
    if (!query) return escapeHtml(text);
    const escaped = escapeHtml(text);
    const regex = new RegExp(`(${query.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')})`, 'gi');
    return escaped.replace(regex, '<mark>$1</mark>');
}

function performSearch(query) {
    if (!query || query.length < 2) {
        searchResults = [];
        return;
    }

    const lowerQuery = query.toLowerCase();
    const results = [];

    for (const article of searchIndex) {
        // タイトルマッチ
        if (article.title.toLowerCase().includes(lowerQuery)) {
            results.push({
                slug: article.slug,
                title: article.title,
                url: article.url,
                lineNum: 0,
                lineText: article.title,
                isTitle: true,
                context: article.lines.slice(0, 10)
            });
        }

        // 行マッチ
        for (const line of article.lines) {
            if (line.text.toLowerCase().includes(lowerQuery)) {
                const lineIdx = article.lines.indexOf(line);
                const contextStart = Math.max(0, lineIdx - 3);
                const contextEnd = Math.min(article.lines.length, lineIdx + 7);

                results.push({
                    slug: article.slug,
                    title: article.title,
                    url: article.url,
                    lineNum: line.num,
                    lineText: line.text,
                    isTitle: false,
                    context: article.lines.slice(contextStart, contextEnd),
                    matchLineIdx: lineIdx - contextStart
                });
            }
        }
    }

    searchResults = results.slice(0, 50);
}

function renderResults(query) {
    if (!resultsList) return;

    resultsList.innerHTML = '';

    if (searchResults.length === 0) {
        if (query && query.length >= 2) {
            resultsList.innerHTML = '<div class="search-no-results">マッチする結果がありません</div>';
        }
        resultsCount.textContent = '0 results';
        previewPane.innerHTML = '';
        return;
    }

    resultsCount.textContent = `${searchResults.length} results`;

    searchResults.forEach((result, index) => {
        const item = document.createElement('div');
        item.className = 'search-result-item' + (index === selectedIndex ? ' selected' : '');
        item.dataset.index = index;

        const location = result.isTitle
            ? `${result.slug}`
            : `${result.slug}:${result.lineNum}`;

        item.innerHTML = `
            <div class="result-location">${escapeHtml(location)}</div>
            <div class="result-text">${highlightMatch(result.lineText.substring(0, 60), query)}${result.lineText.length > 60 ? '...' : ''}</div>
        `;

        item.addEventListener('click', () => {
            // モバイル（プレビュー非表示）の場合はシングルタップで遷移
            const previewPaneContainer = document.querySelector('.search-preview-pane');
            const isMobile = previewPaneContainer && getComputedStyle(previewPaneContainer).display === 'none';
            if (isMobile) {
                navigateToResult(result);
            } else {
                selectedIndex = index;
                renderResults(query);
                renderPreview(query);
            }
        });

        item.addEventListener('dblclick', () => {
            navigateToResult(result);
        });

        resultsList.appendChild(item);
    });

    // スクロールして選択項目を表示
    const selectedItem = resultsList.querySelector('.selected');
    if (selectedItem) {
        selectedItem.scrollIntoView({ block: 'nearest' });
    }

    renderPreview(query);
}

function renderPreview(query) {
    if (!previewPane || searchResults.length === 0) {
        if (previewPane) previewPane.innerHTML = '';
        return;
    }

    const result = searchResults[selectedIndex];
    if (!result) return;

    let previewHtml = `<div class="preview-title">${escapeHtml(result.title)}</div>`;
    previewHtml += '<div class="preview-content">';

    result.context.forEach((line, idx) => {
        const isMatchLine = !result.isTitle && idx === result.matchLineIdx;
        const lineClass = isMatchLine ? 'preview-line match' : 'preview-line';
        const lineText = highlightMatch(line.text, query);
        previewHtml += `<div class="${lineClass}"><span class="line-num">${line.num}</span><span class="line-text">${lineText}</span></div>`;
    });

    previewHtml += '</div>';
    previewPane.innerHTML = previewHtml;
}

function navigateToResult(result) {
    closeSearchModal();
    // キーワードと選択した行のテキスト、行番号をURLパラメータで渡す
    const query = searchInput.value;
    const lineText = result.lineText.substring(0, 80); // 長すぎないように制限
    const lineNum = result.lineNum || 0;
    const url = `${result.url}?highlight=${encodeURIComponent(query)}&lineText=${encodeURIComponent(lineText)}&lineNum=${lineNum}`;
    window.location.href = url;
}

function setMode(mode) {
    currentMode = mode;
    if (modeIndicator) {
        modeIndicator.textContent = mode === 'insert' ? 'INSERT' : 'NORMAL';
        modeIndicator.className = 'search-mode-indicator mode-' + mode;
    }
    if (mode === 'insert') {
        searchInput.focus();
        resultsList.classList.remove('focused');
    } else {
        searchInput.blur();
        resultsList.classList.add('focused');
    }
}

function openSearchModal() {
    if (!searchModal) return;
    searchModal.classList.add('open');

    // コマンドラインに値があれば引き継ぐ
    const cmdInput = document.getElementById('commandline-input');
    const existingQuery = cmdInput ? cmdInput.value.trim() : '';

    if (existingQuery) {
        searchInput.value = existingQuery;
        selectedIndex = 0;
        performSearch(existingQuery);
        renderResults(existingQuery);
    } else {
        searchInput.value = '';
        searchResults = [];
        selectedIndex = 0;
        renderResults('');
    }

    setMode('insert');
}

function closeSearchModal() {
    if (!searchModal) return;
    searchModal.classList.remove('open');
    currentMode = 'insert';

    // 検索モーダルの入力が空ならコマンドラインもクリア＆ハイライト削除
    const cmdInput = document.getElementById('commandline-input');
    if (cmdInput && searchInput.value.trim() === '') {
        cmdInput.value = '';
        cmdInput.setAttribute('readonly', '');
        // ハイライトを削除
        const highlights = document.querySelectorAll('.search-highlight');
        highlights.forEach(mark => {
            const parent = mark.parentNode;
            const text = document.createTextNode(mark.textContent);
            parent.replaceChild(text, mark);
            parent.normalize();
        });
    }
}

function handleInsertModeKeydown(e) {
    switch (e.key) {
        case 'Escape':
            // INSERT -> NORMAL モードへ
            setMode('normal');
            e.preventDefault();
            e.stopPropagation();
            return false;
        case 'ArrowDown':
            selectedIndex = Math.min(selectedIndex + 1, searchResults.length - 1);
            renderResults(searchInput.value);
            e.preventDefault();
            break;
        case 'ArrowUp':
            selectedIndex = Math.max(selectedIndex - 1, 0);
            renderResults(searchInput.value);
            e.preventDefault();
            break;
        case 'Enter':
            if (searchResults[selectedIndex]) {
                navigateToResult(searchResults[selectedIndex]);
            }
            e.preventDefault();
            break;
    }
}

function handleNormalModeKeydown(e) {
    switch (e.key) {
        case 'Escape':
            // NORMALモードでEsc -> モーダルを閉じる
            closeSearchModal();
            e.preventDefault();
            break;
        case 'j':
        case 'ArrowDown':
            selectedIndex = Math.min(selectedIndex + 1, searchResults.length - 1);
            renderResults(searchInput.value);
            e.preventDefault();
            break;
        case 'k':
        case 'ArrowUp':
            selectedIndex = Math.max(selectedIndex - 1, 0);
            renderResults(searchInput.value);
            e.preventDefault();
            break;
        case 'Enter':
        case 'l':
            if (searchResults[selectedIndex]) {
                navigateToResult(searchResults[selectedIndex]);
            }
            e.preventDefault();
            break;
        case 'i':
        case 'a':
            // NORMALからINSERTモードへ
            setMode('insert');
            e.preventDefault();
            break;
        case 'g':
            // gg で先頭へ
            selectedIndex = 0;
            renderResults(searchInput.value);
            e.preventDefault();
            break;
        case 'G':
            // G で末尾へ
            selectedIndex = Math.max(0, searchResults.length - 1);
            renderResults(searchInput.value);
            e.preventDefault();
            break;
    }
}

function handleGlobalKeydown(e) {
    if (!searchModal.classList.contains('open')) return;

    if (currentMode === 'normal') {
        handleNormalModeKeydown(e);
    }
}

function initGrepSearch() {
    searchModal = document.getElementById('search-modal');
    searchInput = document.getElementById('grep-search-input');
    resultsList = document.getElementById('grep-results-list');
    previewPane = document.getElementById('grep-preview');
    resultsCount = document.getElementById('grep-results-count');
    modeIndicator = document.getElementById('search-mode-indicator');

    if (!searchModal || !searchInput) return;

    loadSearchIndex();

    // 検索入力イベント
    searchInput.addEventListener('input', (e) => {
        const query = e.target.value.trim();
        selectedIndex = 0;
        performSearch(query);
        renderResults(query);
    });

    // INSERTモード時のキーボードイベント
    searchInput.addEventListener('keydown', handleInsertModeKeydown);

    // NORMALモード時のキーボードイベント（グローバル）
    document.addEventListener('keydown', handleGlobalKeydown);

    // モーダル外クリックで閉じる
    searchModal.addEventListener('click', (e) => {
        if (e.target === searchModal) {
            closeSearchModal();
        }
    });

    // 閉じるボタン
    const closeBtn = document.getElementById('search-modal-close');
    if (closeBtn) {
        closeBtn.addEventListener('click', closeSearchModal);
    }

    // グローバルキーバインド（モーダルを開く）
    document.addEventListener('keydown', (e) => {
        if (searchModal.classList.contains('open')) return;

        // "/" キーで検索モーダルを開く（入力欄以外で）
        if (e.key === '/' && !['INPUT', 'TEXTAREA'].includes(document.activeElement.tagName)) {
            openSearchModal();
            e.preventDefault();
        }
        // Ctrl+K / Cmd+K で検索モーダルを開く
        if ((e.ctrlKey || e.metaKey) && e.key === 'k') {
            openSearchModal();
            e.preventDefault();
        }
    });

    // コマンドラインのクリックでも開く
    const commandline = document.querySelector('.commandline');
    if (commandline) {
        commandline.addEventListener('click', () => {
            openSearchModal();
        });
    }
}

// openSearchModalをグローバルに公開
window.openSearchModal = openSearchModal;

if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', initGrepSearch);
} else {
    initGrepSearch();
}
"#
    .to_string()
}

pub async fn run() -> Result<()> {
    let content_dir = PathBuf::from("content");
    let dist_dir = Path::new("dist");
    let ogp_dir = dist_dir.join("ogp");
    let pages_dir = PathBuf::from("pages");

    if dist_dir.exists() {
        fs::remove_dir_all(dist_dir)?;
    }
    fs::create_dir_all(dist_dir)?;
    fs::create_dir_all(&ogp_dir)?;

    for entry in WalkDir::new("static").into_iter().filter_map(|e| e.ok()) {
        let target_path = dist_dir.join(entry.path().strip_prefix("static")?);
        if entry.file_type().is_dir() {
            fs::create_dir_all(&target_path)?;
        } else {
            fs::copy(entry.path(), &target_path)?;
        }
    }

    let markdown_files: Vec<PathBuf> = WalkDir::new(&content_dir)
        .into_iter()
        .filter_map(|entry_result| {
            let entry = entry_result.ok()?;
            let path = entry.path().to_path_buf();
            if path.is_file() && path.extension().is_some_and(|ext| ext == "md") {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    let mut articles: Vec<Article> = markdown_files
        .par_iter()
        .filter_map(|input_path| {
            println!("Parsing {input_path:?}");

            match parse_markdown_file(input_path, dist_dir) {
                Ok(article) => Some(article),
                Err(e) => {
                    if e.to_string().contains("Draft article skipped") {
                        println!("Skipped draft article: {input_path:?}");
                    } else {
                        eprintln!("Error processing {input_path:?}: {e}");
                    }
                    None
                }
            }
        })
        .collect();

    articles.sort_by(|a, b| {
        let date_a = extract_date_from_filename(&a.source_path);
        let date_b = extract_date_from_filename(&b.source_path);

        match (date_a, date_b) {
            (Some(date_a), Some(date_b)) => date_b.cmp(&date_a),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => a
                .metadata
                .as_ref()
                .map(|m| &m.title)
                .cmp(&b.metadata.as_ref().map(|m| &m.title)),
        }
    });

    // grep風検索用のインデックス生成（行単位）
    #[derive(serde::Serialize)]
    struct SearchLine {
        num: usize,
        text: String,
    }

    #[derive(serde::Serialize)]
    struct SearchIndexEntry {
        slug: String,
        title: String,
        url: String,
        lines: Vec<SearchLine>,
    }

    let search_index: Vec<SearchIndexEntry> = articles
        .iter()
        .filter_map(|article| {
            let meta = article.metadata.as_ref()?;
            let url = article.relative_url.to_string_lossy().into_owned();
            let slug = url
                .trim_matches('/')
                .split('/')
                .last()
                .unwrap_or("")
                .to_string();

            // content_blocksから検索用の行データを生成（DOMの行番号と対応）
            // オフセット: main-content内でMarkdown本文の前にある要素
            // - h1タイトル: 1
            // - ul.badge-list（言語バッジ）: 1
            // - ul.badge-list（タグバッジ）: 1
            // 合計: 3
            const LINE_NUM_OFFSET: usize = 3;
            let lines: Vec<SearchLine> = article
                .content_blocks
                .iter()
                .map(|block| SearchLine {
                    num: block.line_num + LINE_NUM_OFFSET,
                    text: block.text.clone(),
                })
                .collect();

            Some(SearchIndexEntry {
                slug,
                title: meta.title.clone(),
                url,
                lines,
            })
        })
        .collect();

    let search_index_json = serde_json::to_string(&search_index)?;
    fs::write(dist_dir.join("search-index.json"), search_index_json)?;

    let search_js_code = generate_search_js();
    fs::write(dist_dir.join("search.js"), search_js_code)?;

    let tag_map = collect_tags(&articles);
    let mut sorted_tags: Vec<_> = tag_map.values().collect();
    sorted_tags.sort_by(|a, b| b.count.cmp(&a.count).then_with(|| a.name.cmp(&b.name)));

    let pages_files: Vec<PathBuf> = WalkDir::new(&pages_dir)
        .into_iter()
        .filter_map(|entry_result| {
            let entry = entry_result.ok()?;
            let path = entry.path().to_path_buf();
            if path.is_file() && path.extension().is_some_and(|ext| ext == "md") {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    let pages: Vec<Page> = pages_files
        .par_iter()
        .filter_map(|input_path| {
            println!("Parsing page {input_path:?}");
            match parse_page_file(input_path, &pages_dir, dist_dir) {
                Ok(page) => Some(page),
                Err(e) => {
                    eprintln!("Error processing page {input_path:?}: {e}");
                    None
                }
            }
        })
        .collect();

    let about_content = pages
        .iter()
        .find(|page| page.filename == "about")
        .map(|page| maud::PreEscaped(page.content_html.clone()))
        .unwrap_or_else(|| maud::PreEscaped("About content not found".to_string()));

    let articles_arc = Arc::new(articles.clone());

    // 記事を年月別にグループ化
    let year_groups = group_articles_by_year_month(&articles);

    // デフォルトの記事一覧（ホームページ用、目次なし）
    let articles_list_markup: Markup = generate_file_tree_markup(&year_groups, None, None);

    generate_tag_pages(&tag_map, dist_dir, &articles_list_markup)?;

    let mut font_db = fontdb::Database::new();
    font_db.load_font_file("assets/NotoSansJP-Regular.ttf")?;
    font_db.load_font_file("assets/NotoSansJP-Bold.ttf")?;

    let usvg_options = usvg::Options {
        fontdb: std::sync::Arc::new(font_db),
        ..Default::default()
    };

    articles_arc
        .par_iter()
        .map(|article| {
            if let Some(parent_dir) = article.output_path.parent() {
                fs::create_dir_all(parent_dir).map_err(|e| {
                    anyhow::Error::msg(format!(
                        "Error creating output directory {parent_dir:?}: {e}"
                    ))
                })?;
            }

            // 目次なしの右サイドバー（Neovim UIでは使わない）
            let _sidebar_right_markup = html! {};

            let page_title = article
                .metadata
                .as_ref()
                .map(|m| m.title.as_str())
                .unwrap_or("記事");

            let ogp_svg_url_path = ogp::generate_ogp_svg(page_title, &ogp_dir).map_err(|e| anyhow::Error::msg(format!("OGP SVG generation failed: {e}")))?;
            let ogp_svg_fs_path = dist_dir.join(ogp_svg_url_path.strip_prefix("/").unwrap_or(&ogp_svg_url_path));

            let svg_data = fs::read(&ogp_svg_fs_path)?;
            let tree = usvg::Tree::from_data(&svg_data, &usvg_options)?;
            let pixmap_size = tree.size().to_int_size();
            let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).ok_or_else(|| anyhow::Error::msg("Failed to create pixmap"))?;

            resvg::render(&tree, tiny_skia::Transform::identity(), &mut pixmap.as_mut());

            let ogp_png_url_path = ogp_svg_url_path.replace(".svg", ".png");
            let ogp_png_fs_path = dist_dir.join(ogp_png_url_path.strip_prefix('/').unwrap_or(&ogp_png_url_path));

            pixmap.save_png(&ogp_png_fs_path)?;

            let ogp_image_path = ogp_png_url_path;

            let canonical_url = format!("https://dnfolio.me{}", article.relative_url.to_string_lossy());

            // 新しいNeovim風のスタイルでメインコンテンツを生成
            let main_content_markup = html! {
                @if let Some(meta) = &article.metadata {
                    img src=(ogp_image_path) alt=(meta.title);
                }
                h1 {
                    @if let Some(meta) = &article.metadata {
                        (meta.title)
                    } @else {
                        (article.output_path.file_name().unwrap_or_default().to_string_lossy())
                    }
                }
                // 言語バッジ
                ul class="badge-list" {
                    @if let Some(meta) = &article.metadata
                    && let Some(ref taxonomies) = meta.taxonomies
                    && let Some(ref languages) = taxonomies.languages {
                        @for language in languages {
                            li { span class="badge badge-lang" { (language_display_name(language)) } }
                        }
                    }
                }
                // タグバッジ
                ul class="badge-list" {
                    @if let Some(meta) = &article.metadata
                    && let Some(ref taxonomies) = meta.taxonomies
                    && let Some(ref tags) = taxonomies.tags {
                        @for tag in tags {
                            li { span class="badge badge-tag" { (tag) } }
                        }
                    }
                }
                (maud::PreEscaped(&article.content_html))
            };

            // 現在の記事URL
            let article_url_str = article.relative_url.to_string_lossy().to_string();

            // 目次なしのサイドバーを生成（目次はconfig経由でトップに配置）
            let article_sidebar_markup = generate_file_tree_markup(&year_groups, Some(&article_url_str), None);

            let article_url = article.relative_url.to_string_lossy();
            let structured_data = structured_data::generate_structured_data_html(structured_data::PageType::Article { url: &article_url, ogp_image_url: &ogp_image_path }, article.metadata.as_ref());

            let full_article_html = base::layout_with_toc(
                ArticlePageConfig {
                    base: PageConfig {
                        page_title,
                        canonical_url: &canonical_url,
                        metadata: article.metadata.as_ref(),
                        ogp_image_path: Some(&ogp_image_path),
                        structured_data_html: Some(&structured_data),
                    },
                    toc_html: Some(&article.table_of_contents_html),
                },
                article_sidebar_markup,
                main_content_markup,
            )
            .into_string();

            fs::write(&article.output_path, full_article_html)?;
            Ok(())
        })
        .collect::<Result<Vec<()>>>()?;

    let index_main_content_markup = html! {
        (about_content)
    };

    let index_sidebar_right_markup = html! {
        h2 { "サイト情報" }
        ul {
            li {
                a href="index.html" { "ホーム" }
            }
            li {
                a href="/privacy/" target="_blank" { "プライバシーポリシー" }
            }
        }
        h2 { "タグ一覧" }
        ul {
            @for tag_info in &sorted_tags {
                li {
                    a href=(format!("/tags/{}/", slugify(&tag_info.name))) {
                        (tag_info.name) " " span style="color: #666;" { "(" (tag_info.count) ")" }
                    }
                }
            }
        }
    };

    let index_ogp_path = ogp::generate_ogp_svg("dnfolio", &ogp_dir)?;

    let index_canonical_url: &str = "https://dnfolio.me/";

    let home_structured_data =
        structured_data::generate_structured_data_html(structured_data::PageType::Home, None);

    let index_html_output = base::layout(
        PageConfig {
            page_title: "dnfolio",
            canonical_url: index_canonical_url,
            metadata: None,
            ogp_image_path: Some(&index_ogp_path),
            structured_data_html: Some(&home_structured_data),
        },
        articles_list_markup.clone(),
        index_main_content_markup,
        index_sidebar_right_markup,
    )
    .into_string();

    fs::write(dist_dir.join("index.html"), index_html_output)?;

    if let Some(privacy_page) = pages.iter().find(|page| page.filename == "privacy") {
        let privacy_main_content_markup = html! {
            h1 {
                "プライバシーポリシー"
            }
            (maud::PreEscaped(&privacy_page.content_html))
        };

        let privacy_sidebar_right_markup = html! {
            h2 { "サイト情報" }
            ul {
                li {
                    a href="index.html" { "ホームに戻る" }
                }
            }
        };

        let privacy_canonical_url = format!(
            "https://dnfolio.me{}",
            privacy_page.relative_url.to_string_lossy()
        );

        let privacy_html_output = base::layout(
            PageConfig {
                page_title: "プライバシーポリシー",
                canonical_url: &privacy_canonical_url,
                metadata: None,
                ogp_image_path: None,
                structured_data_html: None,
            },
            articles_list_markup.clone(),
            privacy_main_content_markup,
            privacy_sidebar_right_markup,
        )
        .into_string();
        if let Some(parent) = privacy_page.output_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&privacy_page.output_path, privacy_html_output)?;
    } else {
        fs::write(
            dist_dir.join("privacy.html"),
            privacy::layout().into_string(),
        )?;
    }

    // 404ページを生成
    let not_found_main_content = html! {
        div style="text-align: center; padding: 4rem 1rem;" {
            h1 style="font-size: 4rem; margin-bottom: 1rem; color: #6c757d;" { "404" }
            p style="font-size: 1.25rem; color: #6c757d; margin-bottom: 2rem;" {
                "お探しのページは見つかりませんでした。"
            }
            a href="/" style="color: #007bff; text-decoration: none;" { "ホームに戻る" }
        }
    };

    let not_found_sidebar_right = html! {
        h2 { "サイト情報" }
        ul {
            li {
                a href="/" { "ホームに戻る" }
            }
        }
    };

    let not_found_html = base::layout(
        PageConfig {
            page_title: "ページが見つかりません - dnfolio",
            canonical_url: "https://dnfolio.me/404",
            metadata: None,
            ogp_image_path: Some("/ogp/dnfolio.png"),
            structured_data_html: None,
        },
        articles_list_markup.clone(),
        not_found_main_content,
        not_found_sidebar_right,
    )
    .into_string();

    fs::write(dist_dir.join("404.html"), not_found_html)?;
    println!("Generated 404.html");

    let base_url = "https://dnfolio.me";
    sitemap::generate_and_write_sitemap(base_url, &articles, &pages, &tag_map, dist_dir)?;

    rss::generate_rss(&articles, dist_dir)?;

    Ok(())
}
