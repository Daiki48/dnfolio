use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::models::{Article, Page, TagInfo};
use anyhow::Result;
use chrono::{DateTime, FixedOffset, NaiveDate, TimeZone, Utc};
use maud::{Markup, PreEscaped, html};
use slug::slugify;

fn build_sitemap_markup(
    base_url: &str,
    articles: &[Article],
    pages: &[Page],
    tag_map: &HashMap<String, TagInfo>,
) -> Markup {
    let jst = FixedOffset::east_opt(9 * 3600).unwrap();
    let build_time = Utc::now().with_timezone(&jst);

    html! {
        (PreEscaped("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"))
        urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9" {
            url {
                loc { (base_url) }
                lastmod { (build_time.to_rfc3339()) }
            }

            @for article in articles {
                url {
                    loc { (format!("{}{}", base_url, article.relative_url.to_string_lossy())) }
                    lastmod { (get_article_lastmod(article, &build_time)) }
                }
            }

            @for page in pages {
                url {
                    loc { (format!("{}{}", base_url, page.relative_url.to_string_lossy())) }
                    lastmod { (build_time.to_rfc3339()) }
                }
            }

            @for tag_name in tag_map.keys() {
                url {
                    loc { (format!("{}/tags/{}/", base_url, slugify(tag_name))) }
                    lastmod { (build_time.to_rfc3339()) }
                }
            }
        }
    }
}

fn get_article_lastmod(article: &Article, default_time: &DateTime<FixedOffset>) -> String {
    let jst = FixedOffset::east_opt(9 * 3600).unwrap();
    let lastmod_date = article
        .metadata
        .as_ref()
        .and_then(|m| m.updated.as_ref().or(m.created.as_ref()))
        .and_then(|date_str| NaiveDate::parse_from_str(date_str, "%Y-%m-%d").ok())
        .and_then(|d| d.and_hms_opt(0, 0, 0))
        .and_then(|naive_dt| jst.from_local_datetime(&naive_dt).single());

    lastmod_date.unwrap_or(*default_time).to_rfc3339()
}

pub fn generate_and_write_sitemap(
    base_url: &str,
    articles: &[Article],
    pages: &[Page],
    tag_map: &HashMap<String, TagInfo>,
    dist_dir: &Path,
) -> Result<()> {
    let sitemap_xml = build_sitemap_markup(base_url, articles, pages, tag_map).into_string();
    let sitemap_path = dist_dir.join("sitemap.xml");
    fs::write(sitemap_path, sitemap_xml)?;
    println!("✅ Sitemap generated successfully (self-implemented).");
    Ok(())
}
