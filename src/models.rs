use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Clone)]
pub struct Taxonomies {
    #[serde(default)]
    pub tags: Option<Vec<String>>,
    #[serde(default)]
    pub languages: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MetaData {
    pub title: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub created: Option<String>,
    #[serde(default)]
    pub updated: Option<String>,
    #[serde(default)]
    pub draft: Option<bool>,
    #[serde(default)]
    pub taxonomies: Option<Taxonomies>,
}

#[derive(Debug)]
pub struct Page {
    pub content_html: String,
    pub output_path: PathBuf,
    pub relative_url: PathBuf,
    pub filename: String,
}

#[derive(Debug, Clone)]
pub struct Article {
    pub metadata: Option<MetaData>,
    pub content_html: String,
    pub plain_content: String,
    pub output_path: PathBuf,
    pub relative_url: PathBuf,
    pub table_of_contents_html: String,
    pub source_path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct Heading {
    pub level: u8,
    pub id: String,
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct TagInfo {
    pub name: String,
    pub count: usize,
    pub articles: Vec<Article>,
}
