use std::fs;
use std::path::Path;

use crate::dates;
use crate::models::Article;
use anyhow::Result;
use rss::{CategoryBuilder, ChannelBuilder, GuidBuilder, ItemBuilder};

const SITE_URL: &str = "https://dnfolio.me";
const SITE_TITLE: &str = "dnfolio";
const SITE_DESCRIPTION: &str = "Daikiの個人サイト。技術ブログを公開しています。";

pub fn generate_rss(articles: &[Article], dist_dir: &Path) -> Result<()> {
    let items: Vec<rss::Item> = articles
        .iter()
        .take(20)
        .filter_map(|article| {
            let meta = article.metadata.as_ref()?;
            let full_url = format!("{}{}", SITE_URL, article.relative_url.to_string_lossy());
            let article_dates = dates::resolve_article_dates(article);

            // タグからカテゴリを生成
            let categories: Vec<rss::Category> = meta
                .taxonomies
                .as_ref()
                .and_then(|t| t.tags.as_ref())
                .map(|tags| {
                    tags.iter()
                        .map(|tag| CategoryBuilder::default().name(tag.clone()).build())
                        .collect()
                })
                .unwrap_or_default();

            Some(
                ItemBuilder::default()
                    .title(Some(meta.title.clone()))
                    .link(Some(full_url.clone()))
                    .description(meta.description.clone())
                    .pub_date(article_dates.map(|resolved| resolved.published.to_rfc2822()))
                    .categories(categories)
                    .guid(Some(
                        GuidBuilder::default()
                            .value(full_url)
                            .permalink(true)
                            .build(),
                    ))
                    .build(),
            )
        })
        .collect();

    let channel = ChannelBuilder::default()
        .title(SITE_TITLE)
        .link(SITE_URL)
        .description(SITE_DESCRIPTION)
        .language(Some("ja".to_string()))
        .items(items)
        .build();

    let rss_xml = channel.to_string();
    fs::write(dist_dir.join("feed.xml"), rss_xml)?;
    println!("Generated: feed.xml");

    Ok(())
}
