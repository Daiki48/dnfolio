//! JSON-LD構造化データ生成モジュール
//!
//! Google検索エンジンがページ内容を正確に理解するための
//! schema.org準拠の構造化データを生成する

use crate::models::MetaData;

/// サイトの基本情報
const SITE_NAME: &str = "dnfolio";
const SITE_URL: &str = "https://dnfolio.me";
const AUTHOR_NAME: &str = "Daiki";

/// ページの種類に応じた構造化データ生成用の列挙型
pub enum PageType<'a> {
    /// トップページ
    Home,
    /// 記事ページ
    Article {
        url: &'a str,
        ogp_image_url: &'a str,
    },
    /// タグ一覧ページ
    TagPage { tag_name: &'a str, url: &'a str },
}

/// 構造化データのHTMLを生成する
///
/// # Arguments
/// * `page_type` - ページの種類
/// * `metadata` - 記事のメタデータ（記事ページの場合のみ使用）
///
/// # Returns
/// `<script type="application/ld+json">...</script>` 形式のHTML文字列
pub fn generate_structured_data_html(page_type: PageType, metadata: Option<&MetaData>) -> String {
    let json_ld = match page_type {
        PageType::Home => generate_website_json_ld(),
        PageType::Article { url, ogp_image_url } => {
            generate_article_json_ld(metadata, url, ogp_image_url)
        }
        PageType::TagPage { tag_name, url } => generate_tag_page_json_ld(tag_name, url),
    };

    format!(r#"<script type="application/ld+json">{}</script>"#, json_ld)
}

/// WebSite構造化データ（トップページ用）
fn generate_website_json_ld() -> String {
    serde_json::json!({
        "@context": "https://schema.org",
        "@type": "WebSite",
        "name": SITE_NAME,
        "url": SITE_URL,
        "author": {
            "@type": "Person",
            "name": AUTHOR_NAME
        },
        "description": "Daikiの個人サイト。技術ブログや作品を公開しています。"
    })
    .to_string()
}

/// BlogPosting構造化データ（記事ページ用）
fn generate_article_json_ld(metadata: Option<&MetaData>, url: &str, ogp_image_url: &str) -> String {
    let meta = match metadata {
        Some(m) => m,
        None => return generate_website_json_ld(), // フォールバック
    };

    let full_url = format!("{}{}", SITE_URL, url);
    let full_image_url = format!("{}{}", SITE_URL, ogp_image_url);

    // 公開日（createdフィールド）
    let date_published = meta.created.as_deref().unwrap_or("2025-01-01");

    // 更新日（updatedフィールドがあれば使用、なければ公開日）
    let date_modified = meta.updated.as_deref().unwrap_or(date_published);

    let description = meta.description.as_deref().unwrap_or(&meta.title);

    // パンくずリスト
    let breadcrumb = serde_json::json!({
        "@type": "BreadcrumbList",
        "itemListElement": [
            {
                "@type": "ListItem",
                "position": 1,
                "name": "ホーム",
                "item": SITE_URL
            },
            {
                "@type": "ListItem",
                "position": 2,
                "name": "記事",
                "item": format!("{}/posts/", SITE_URL)
            },
            {
                "@type": "ListItem",
                "position": 3,
                "name": &meta.title,
                "item": &full_url
            }
        ]
    });

    // メインの記事構造化データ
    let article = serde_json::json!({
        "@context": "https://schema.org",
        "@type": "BlogPosting",
        "headline": &meta.title,
        "description": description,
        "image": full_image_url,
        "url": full_url,
        "datePublished": date_published,
        "dateModified": date_modified,
        "author": {
            "@type": "Person",
            "name": AUTHOR_NAME,
            "url": SITE_URL
        },
        "publisher": {
            "@type": "Person",
            "name": AUTHOR_NAME
        },
        "mainEntityOfPage": {
            "@type": "WebPage",
            "@id": full_url
        }
    });

    // 複数の構造化データを配列で返す
    serde_json::json!([article, breadcrumb]).to_string()
}

/// タグページ用構造化データ
fn generate_tag_page_json_ld(tag_name: &str, url: &str) -> String {
    let full_url = format!("{}{}", SITE_URL, url);

    let breadcrumb = serde_json::json!({
        "@context": "https://schema.org",
        "@type": "BreadcrumbList",
        "itemListElement": [
            {
                "@type": "ListItem",
                "position": 1,
                "name": "ホーム",
                "item": SITE_URL
            },
            {
                "@type": "ListItem",
                "position": 2,
                "name": "タグ",
                "item": format!("{}/tags/", SITE_URL)
            },
            {
                "@type": "ListItem",
                "position": 3,
                "name": tag_name,
                "item": full_url
            }
        ]
    });

    let collection = serde_json::json!({
        "@context": "https://schema.org",
        "@type": "CollectionPage",
        "name": format!("{}の記事一覧", tag_name),
        "url": full_url
    });

    serde_json::json!([collection, breadcrumb]).to_string()
}
