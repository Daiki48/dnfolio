use std::fs;
use std::path::Path;

use crate::dates;
use crate::models::{Article, Page};
use anyhow::Result;
use chrono::{FixedOffset, Utc};
use maud::{Markup, PreEscaped, html};

fn build_sitemap_markup(base_url: &str, articles: &[Article], pages: &[Page]) -> Markup {
    let jst = FixedOffset::east_opt(9 * 3600).unwrap();
    let build_time = Utc::now().with_timezone(&jst).to_rfc3339();
    let home_lastmod = dates::latest_article_lastmod(articles)
        .map(|lastmod| lastmod.to_rfc3339())
        .unwrap_or_else(|| build_time.clone());

    html! {
        (PreEscaped("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"))
        urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9" {
            url {
                loc { (base_url) }
                lastmod { (home_lastmod) }
            }

            @for article in articles {
                @if let Some(article_dates) = dates::resolve_article_dates(article) {
                    url {
                        loc { (format!("{}{}", base_url, article.relative_url.to_string_lossy())) }
                        lastmod { (article_dates.modified.to_rfc3339()) }
                    }
                }
            }

            @for page in pages.iter().filter(|page| page.filename != "about") {
                url {
                    loc { (format!("{}{}", base_url, page.relative_url.to_string_lossy())) }
                    lastmod { (build_time.clone()) }
                }
            }
        }
    }
}

pub fn generate_and_write_sitemap(
    base_url: &str,
    articles: &[Article],
    pages: &[Page],
    dist_dir: &Path,
) -> Result<()> {
    let sitemap_xml = build_sitemap_markup(base_url, articles, pages).into_string();
    let sitemap_path = dist_dir.join("sitemap.xml");
    fs::write(sitemap_path, sitemap_xml)?;
    println!("✅ Sitemap generated successfully (self-implemented).");
    Ok(())
}
