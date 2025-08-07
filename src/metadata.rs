use serde::Deserialize;

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
    pub draft: Option<bool>,
    #[serde(default)]
    pub taxonomies: Option<Taxonomies>,
}
