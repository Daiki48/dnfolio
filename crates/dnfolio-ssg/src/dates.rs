use std::path::Path;

use chrono::{DateTime, FixedOffset, NaiveDate, TimeZone};

use crate::models::{Article, MetaData};

#[derive(Debug, Clone)]
pub struct ResolvedArticleDates {
    pub published: DateTime<FixedOffset>,
    pub modified: DateTime<FixedOffset>,
}

fn jst() -> FixedOffset {
    FixedOffset::east_opt(9 * 3600).unwrap()
}

fn parse_date(value: Option<&str>) -> Option<NaiveDate> {
    value.and_then(|date_str| NaiveDate::parse_from_str(date_str, "%Y-%m-%d").ok())
}

fn start_of_day(date: NaiveDate) -> Option<DateTime<FixedOffset>> {
    date.and_hms_opt(0, 0, 0)
        .and_then(|naive_dt| jst().from_local_datetime(&naive_dt).single())
}

pub fn extract_date_from_path(path: &Path) -> Option<NaiveDate> {
    let file_name = path.file_stem()?.to_str()?;
    let date_part = file_name.split('_').next()?;

    if date_part.len() < 10 {
        return None;
    }

    NaiveDate::parse_from_str(&date_part[0..10], "%Y-%m-%d").ok()
}

pub fn resolve_metadata_dates(
    metadata: Option<&MetaData>,
    source_path: &Path,
) -> Option<ResolvedArticleDates> {
    let published_date = metadata
        .and_then(|meta| parse_date(meta.created.as_deref()))
        .or_else(|| extract_date_from_path(source_path))?;
    let modified_date = metadata
        .and_then(|meta| parse_date(meta.updated.as_deref()))
        .unwrap_or(published_date);

    Some(ResolvedArticleDates {
        published: start_of_day(published_date)?,
        modified: start_of_day(modified_date)?,
    })
}

pub fn resolve_article_dates(article: &Article) -> Option<ResolvedArticleDates> {
    resolve_metadata_dates(article.metadata.as_ref(), &article.source_path)
}

pub fn latest_article_lastmod(articles: &[Article]) -> Option<DateTime<FixedOffset>> {
    articles
        .iter()
        .filter_map(resolve_article_dates)
        .map(|dates| dates.modified)
        .max()
}
