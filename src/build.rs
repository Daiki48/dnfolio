use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::Result;
use gray_matter::{Matter, ParsedEntity};
use maud::{Markup, html};
use pulldown_cmark::{CowStr, Event, Parser, Tag, TagEnd};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde::Deserialize;
use slug::slugify;
use walkdir::WalkDir;

use crate::routes::base;

#[derive(Debug, Deserialize, Clone)]
struct Taxonomies {
    #[serde(default)]
    tags: Option<Vec<String>>,
    #[serde(default)]
    languages: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Clone)]
struct MetaData {
    title: String,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    slug: Option<String>,
    #[serde(default)]
    draft: Option<bool>,
    #[serde(default)]
    taxonomies: Option<Taxonomies>,
}

#[derive(Debug, Clone)]
struct Article {
    metadata: Option<MetaData>,
    content_html: String,
    output_path: PathBuf,
    relative_url: PathBuf,
    table_of_contents_html: String,
}

#[derive(Debug, Clone)]
struct Heading {
    level: u8,
    id: String,
    text: String,
}

fn parse_markdown_file(
    input_path: &Path,
    content_dir: &Path,
    output_content_dir: &Path,
    _dist_dir: &Path,
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

    let parser = Parser::new_ext(&markdown_content,pulldown_options);
    let mut headings: Vec<Heading> = Vec::new();
    let mut id_counts: HashMap<String, usize> = HashMap::new();
    let mut html_output = String::new();

    let mut current_heading_text_buffer = String::new();
    let mut is_in_heading = false;
    let mut processed_events: Vec<Event> = Vec::new();

    for event in parser {
        match event {
            Event::Start(Tag::Heading { level, id, classes, attrs }) => {
                is_in_heading = true;
                current_heading_text_buffer.clear();
                processed_events.push(Event::Start(Tag::Heading { level, id, classes, attrs }));
            },
            Event::End(TagEnd::Heading(level)) => {
                is_in_heading = false;
                let text_content = current_heading_text_buffer.trim().to_string();
                let mut final_id_to_use: Option<CowStr> = None;

                for i in (0..processed_events.len()).rev() {
                    if let Event::Start(Tag::Heading { level: h_level, id: existing_id_in_event, .. }) = &processed_events[i] {
                        if *h_level == level {
                            if existing_id_in_event.is_some() {
                                final_id_to_use = existing_id_in_event.clone();
                                break;
                            } else {
                                break;
                            }
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
                    if let Event::Start(Tag::Heading { level: h_level, id: h_id, .. }) = &mut processed_events[i] {
                        if *h_level == level && h_id.is_none() {
                            *h_id = Some(CowStr::from(id_string.clone()));
                            found_start = true;
                            break;
                        }
                    }
                }
                if !found_start {
                    eprintln!("Warning: Could not find matching Start(Heading) event to assign ID for heading: {text_content:?}");
                }
                processed_events.push(Event::End(Tag::Heading { level, id: Some(CowStr::from(id_string)), classes: Vec::new(), attrs: Vec::new() }.into()));
            },
            Event::Text(text) => {
                if is_in_heading {
                    current_heading_text_buffer.push_str(&text);
                }
                processed_events.push(Event::Text(text));
            },
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

pub async fn run() -> Result<()> {
    println!("cargo:rerun-if-chaged=static/*");
    println!("cargo:rerun-if-chaged=content/*");

    let content_dir = PathBuf::from("content");
    let dist_dir = Path::new("dist");
    let output_content_dir = dist_dir.join("content");

    if dist_dir.exists() {
        fs::remove_dir_all(dist_dir)?;
    }
    fs::create_dir_all(dist_dir)?;
    fs::create_dir_all(&output_content_dir)?;

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

            match parse_markdown_file(input_path, &content_dir, &output_content_dir, dist_dir) {
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

            // println!("Generating HTML for {:?}", article.output_path);

            let main_content_markup = html! {
                h1 {
                    @if let Some(meta) = &article.metadata {
                        (meta.title)
                    } @else {
                        (article.output_path.file_name().unwrap_or_default().to_string_lossy())
                    }
                }
                (maud::PreEscaped(&article.content_html))
            };

            let sidebar_right_markup = html! {
                (maud::PreEscaped(&article.table_of_contents_html))
            };

            let page_title = article
                .metadata
                .as_ref()
                .map(|m| m.title.as_str())
                .unwrap_or("記事");
            let full_article_html = base::layout(
                page_title,
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
        h1 { "dnfolioへようこそ" }
        p { "これは私の個人サイトです。プログラミングや日々の出来事について書いています。" }
        p { "左サイドバーから記事を選択してください。" }
    };

    let index_sidebar_right_markup = html! {
        h2 { "サイト情報" }
        p { "サイト全体の概要やリンクなど" }
    };

    let index_html_output = base::layout(
        "dnfolio",
        articles_list_markup,
        index_main_content_markup,
        index_sidebar_right_markup,
    )
    .into_string();

    fs::write(dist_dir.join("index.html"), index_html_output)?;
    Ok(())
}
