use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::Result;
use gray_matter::{Matter, ParsedEntity};
use maud::{Markup, html};
use pulldown_cmark::{CowStr, Event, Parser, Tag, TagEnd};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use slug::slugify;
use walkdir::WalkDir;

use crate::models::{MetaData, Article, Page, TagInfo, Heading};
use crate::ogp;
use crate::templates::{base, privacy};

fn parse_markdown_file(
    input_path: &Path,
    content_dir: &Path,
    output_content_dir: &Path,
) -> anyhow::Result<Article> {
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

    let mut current_heading_text_buffer = String::new();
    let mut is_in_heading = false;
    let mut processed_events: Vec<Event> = Vec::new();

    for event in parser {
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
                if is_in_heading {
                    current_heading_text_buffer.push_str(&text);
                }
                processed_events.push(Event::Text(text));
            }
            Event::Code(text) => {
                if is_in_heading {
                    current_heading_text_buffer.push_str(&text);
                }
                processed_events.push(Event::Code(text));
            }
            other => {
                processed_events.push(other);
            }
        }
    }

    pulldown_cmark::html::push_html(&mut html_output, processed_events.into_iter());

    let toc_markup = html! {
        h2 { "目次" }
        ul {
            @for heading in &headings {
                @if heading.level == 2 {
                    li { a href=(format!("#{}", heading.id)) { (heading.text) } }
                } @else if heading.level == 3 {
                    li style="margin-left: 20px;" { a href=(format!("#{}", heading.id)) { (heading.text) } }
                }
            }
        }
    };

    let table_of_contents_html = toc_markup.into_string();

    println!("\n--- HTML Output with IDs for {input_path:?} ---\n{html_output}");
    println!("\n=============================================================\n");

    let relative_path = input_path.strip_prefix(content_dir)?.with_extension("html");
    let output_path = output_content_dir.join(&relative_path);
    let relative_url = PathBuf::from("/").join("content").join(&relative_path);

    Ok(Article {
        metadata,
        content_html: html_output,
        output_path,
        relative_url,
        table_of_contents_html,
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
    let output_path = dist_dir.join(format!("{file_stem}.html"));
    let relative_url = PathBuf::from(format!("/{file_stem}.html"));

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
        let tag_page_path = tags_dir.join(format!("{tag_slug}.html"));

        let tag_main_content_markup = html! {
            h1 { "タグ: " (tag_name) "(" (tag_info.count) "件)" }
            ul {
                @for article in &tag_info.articles {
                    li {
                        a href=(format!("../{}", article.relative_url.to_string_lossy())) {
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
                    a href="../index.html" { "ホームに戻る" }
                }
            }
        };

        let tag_html_output = base::layout(
            &format!("タグ: {tag_name}"),
            None,
            None,
            articles_list_markup.clone(),
            tag_main_content_markup,
            tag_sidebar_right_markup,
        )
        .into_string();
        fs::write(tag_page_path, tag_html_output)?;
    }
    Ok(())
}

pub async fn run() -> Result<()> {
    let content_dir = PathBuf::from("content");
    let dist_dir = Path::new("dist");
    let output_content_dir = dist_dir.join("content");
    let ogp_dir = dist_dir.join("ogp");
    let pages_dir = PathBuf::from("pages");

    if dist_dir.exists() {
        fs::remove_dir_all(dist_dir)?;
    }
    fs::create_dir_all(dist_dir)?;
    fs::create_dir_all(&output_content_dir)?;
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

            match parse_markdown_file(input_path, &content_dir, &output_content_dir) {
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
        a.metadata
            .as_ref()
            .map(|m| &m.title)
            .cmp(&b.metadata.as_ref().map(|m| &m.title))
    });

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

    let articles_list_markup: Markup = html! {
        h2 { "記事一覧" }
        ul {
            @for article in &articles {
                li {
                    a href=(article.relative_url.to_string_lossy()) {
                        @if let Some(meta) = &article.metadata {
                            (meta.title)
                        } @else {
                            (article.output_path.file_name().unwrap_or_default().to_string_lossy())
                        }
                    }
                }
            }
        }
    };

    generate_tag_pages(&tag_map, dist_dir, &articles_list_markup)?;

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

            let sidebar_right_markup = html! {
                (maud::PreEscaped(&article.table_of_contents_html))
            };

            let page_title = article
                .metadata
                .as_ref()
                .map(|m| m.title.as_str())
                .unwrap_or("記事");

            let ogp_image_path = ogp::generate_ogp_svg(page_title, &ogp_dir)
                .map_err(|e| anyhow::Error::msg(format!("OGP image generation failed: {e}")))?;

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
                ul style="display: flex;" {
                    @if let Some(meta) = &article.metadata
                    && let Some(ref taxonomies) = meta.taxonomies
                    && let Some(ref languages) = taxonomies.languages {
                        @for language in languages {
                            li style="padding: 2px 4px; margin: 2px; border: 1px solid gray; border-radius: 4px; list-style: none; background-color: #252525; color: #fff;" { (language_display_name(language)) }
                        }
                    }
                }
                ul style="display: flex;" {
                    @if let Some(meta) = &article.metadata
                    && let Some(ref taxonomies) = meta.taxonomies
                    && let Some(ref tags) = taxonomies.tags {
                        @for tag in tags {
                            li style="padding: 2px 6px; margin: 2px; border: 1px solid gray; border-radius: 10px; list-style: none; background-color: #9e9e9e; color: #000;" { (tag) }
                        }
                    }
                }
                (maud::PreEscaped(&article.content_html))
            };

            let full_article_html = base::layout(
                page_title,
                article.metadata.as_ref(),
                Some(&ogp_image_path),
                articles_list_markup.clone(),
                main_content_markup,
                sidebar_right_markup,
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
            @if let Some(privacy_page) = pages.iter().find(|page| page.filename == "privacy") {
                li {
                    a href=(privacy_page.relative_url.to_string_lossy()) target="_blank" { "プライバシーポリシー" }
                }
            } @else {
                li {
                    a href="privacy.html" target="_blank" { "プライバシーポリシー" }
                }
            }
        }
        h2 { "タグ一覧" }
        ul {
            @for tag_info in &sorted_tags {
                li {
                    a href=(format!("tags/{}.html", slugify(&tag_info.name))) {
                        (tag_info.name) " " span style="color: #666;" { "(" (tag_info.count) ")" }
                    }
                }
            }
        }
    };

    let index_ogp_path = ogp::generate_ogp_svg("dnfolio", &ogp_dir)?;

    let index_html_output = base::layout(
        "dnfolio",
        None,
        Some(&index_ogp_path),
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

        let privacy_html_output = base::layout(
            "プライバシーポリシー",
            None,
            None,
            articles_list_markup.clone(),
            privacy_main_content_markup,
            privacy_sidebar_right_markup,
        )
        .into_string();
        fs::write(&privacy_page.output_path, privacy_html_output)?;
    } else {
        fs::write(
            dist_dir.join("privacy.html"),
            privacy::layout().into_string(),
        )?;
    }
    Ok(())
}
