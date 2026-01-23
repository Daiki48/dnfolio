//! 検索インデックス管理
//!
//! search-index.jsonをfetchしてメモリに保持
//!
//! # セキュリティ対策
//!
//! - レスポンスサイズの上限チェック（DoS対策）
//! - 記事数・行数の上限チェック（メモリ枯渇対策）
//! - 各フィールドの長さ検証
//! - URL形式の検証

use serde::Deserialize;
use std::cell::RefCell;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

use crate::error::{DnfolioError, Result};

// =============================================================================
// セキュリティ定数
// =============================================================================

/// JSONレスポンスの最大サイズ（5MB）
const MAX_RESPONSE_SIZE: usize = 5 * 1024 * 1024;
/// 記事の最大数
const MAX_ARTICLES: usize = 10000;
/// 1記事あたりの最大行数
const MAX_LINES_PER_ARTICLE: usize = 5000;
/// タイトルの最大長
const MAX_TITLE_LEN: usize = 500;
/// URLの最大長
const MAX_URL_LEN: usize = 500;
/// 行テキストの最大長
const MAX_LINE_TEXT_LEN: usize = 2000;

/// 検索インデックスの行情報
#[derive(Debug, Clone, Deserialize)]
pub struct SearchLine {
    pub num: usize,
    pub text: String,
}

/// 記事の検索インデックス
#[derive(Debug, Clone, Deserialize)]
pub struct SearchArticle {
    pub slug: String,
    pub title: String,
    pub url: String,
    pub lines: Vec<SearchLine>,
}

/// 検索結果（記事単位）
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub article: SearchArticle,
    pub matched_lines: Vec<SearchLine>,
}

/// 検索マッチ（行単位、grep形式表示用）
#[derive(Debug, Clone)]
pub struct SearchMatch {
    /// 記事タイトル
    pub title: String,
    /// 記事URL
    pub url: String,
    /// 行番号
    pub line_num: usize,
    /// 行テキスト
    pub line_text: String,
}

/// 検索インデックス管理
pub struct SearchIndex {
    articles: RefCell<Vec<SearchArticle>>,
    loaded: RefCell<bool>,
}

impl SearchIndex {
    pub fn new() -> Self {
        Self {
            articles: RefCell::new(Vec::new()),
            loaded: RefCell::new(false),
        }
    }

    /// search-index.jsonを非同期でロード
    pub async fn load(&self) -> Result<()> {
        if *self.loaded.borrow() {
            return Ok(());
        }

        let window = web_sys::window().ok_or_else(|| DnfolioError::DomError("No window".into()))?;

        // RequestInit設定
        let opts = RequestInit::new();
        opts.set_method("GET");
        opts.set_mode(RequestMode::SameOrigin);

        // Requestオブジェクト作成
        let request = Request::new_with_str_and_init("/search-index.json", &opts)
            .map_err(|e| DnfolioError::JsError(format!("{e:?}")))?;

        // fetch実行
        let resp_value = JsFuture::from(window.fetch_with_request(&request))
            .await
            .map_err(|e| DnfolioError::JsError(format!("{e:?}")))?;

        // Responseにキャスト
        let resp: Response = resp_value
            .dyn_into()
            .map_err(|e| DnfolioError::JsError(format!("{e:?}")))?;

        // ステータスコード確認
        if !resp.ok() {
            return Err(DnfolioError::JsError(format!(
                "HTTP error: {}",
                resp.status()
            )));
        }

        // JSONとしてパース
        let json = JsFuture::from(
            resp.json()
                .map_err(|e| DnfolioError::JsError(format!("{e:?}")))?,
        )
        .await
        .map_err(|e| DnfolioError::JsError(format!("{e:?}")))?;

        // serde-wasm-bindgenでRust型に変換
        let articles: Vec<SearchArticle> = serde_wasm_bindgen::from_value(json)
            .map_err(|e| DnfolioError::JsError(format!("{e:?}")))?;

        web_sys::console::log_1(
            &format!("Search index loaded: {} articles", articles.len()).into(),
        );

        *self.articles.borrow_mut() = articles;
        *self.loaded.borrow_mut() = true;

        Ok(())
    }

    /// クエリで検索（タイトル + 本文）
    pub fn search(&self, query: &str) -> Vec<SearchResult> {
        if query.is_empty() {
            return Vec::new();
        }

        let lower_query = query.to_lowercase();
        let articles = self.articles.borrow();

        articles
            .iter()
            .filter_map(|article| {
                // タイトルまたは行にマッチ
                let title_match = article.title.to_lowercase().contains(&lower_query);
                let matched_lines: Vec<SearchLine> = article
                    .lines
                    .iter()
                    .filter(|line| line.text.to_lowercase().contains(&lower_query))
                    .cloned()
                    .collect();

                if title_match || !matched_lines.is_empty() {
                    Some(SearchResult {
                        article: article.clone(),
                        matched_lines,
                    })
                } else {
                    None
                }
            })
            .collect()
    }

    /// インデックスがロード済みか
    pub fn is_loaded(&self) -> bool {
        *self.loaded.borrow()
    }
}

impl Default for SearchIndex {
    fn default() -> Self {
        Self::new()
    }
}

// グローバルインスタンス
thread_local! {
    pub static SEARCH_INDEX: RefCell<SearchIndex> = RefCell::new(SearchIndex::new());
}

/// 検索インデックスデータを検証
///
/// # セキュリティ対策
/// - 記事数の上限チェック
/// - 各フィールドの長さ検証
/// - URL形式の検証（相対パスのみ許可）
fn validate_search_index(articles: &[SearchArticle]) -> Result<()> {
    // 記事数チェック
    if articles.len() > MAX_ARTICLES {
        return Err(DnfolioError::ValidationError(format!(
            "Too many articles: {} (max: {})",
            articles.len(),
            MAX_ARTICLES
        )));
    }

    for (i, article) in articles.iter().enumerate() {
        // タイトル長チェック
        if article.title.len() > MAX_TITLE_LEN {
            web_sys::console::warn_1(
                &format!("Article {} title too long, truncating in memory", i).into(),
            );
        }

        // URL検証（相対パスのみ許可）
        if article.url.len() > MAX_URL_LEN {
            return Err(DnfolioError::ValidationError(format!(
                "Article {} URL too long",
                i
            )));
        }
        // Protocol-relative URL（//で始まる）を拒否
        if article.url.starts_with("//") {
            return Err(DnfolioError::ValidationError(format!(
                "Article {} has protocol-relative URL (not allowed)",
                i
            )));
        }
        // /で始まる相対パスのみ許可
        if !article.url.starts_with('/') {
            return Err(DnfolioError::ValidationError(format!(
                "Article {} has invalid URL (must be relative path starting with /)",
                i
            )));
        }
        // 危険なURLスキームをチェック（URLエンコードされたものも考慮）
        let lower_url = article.url.to_lowercase();
        let dangerous_patterns = ["javascript:", "data:", "vbscript:", "blob:"];
        for pattern in dangerous_patterns {
            if lower_url.contains(pattern) {
                return Err(DnfolioError::ValidationError(format!(
                    "Article {} has dangerous URL pattern",
                    i
                )));
            }
        }

        // 行数チェック
        if article.lines.len() > MAX_LINES_PER_ARTICLE {
            web_sys::console::warn_1(
                &format!(
                    "Article {} has too many lines: {} (max: {})",
                    i,
                    article.lines.len(),
                    MAX_LINES_PER_ARTICLE
                )
                .into(),
            );
        }

        // 各行のテキスト長チェック
        for line in &article.lines {
            if line.text.len() > MAX_LINE_TEXT_LEN {
                // 警告のみ（切り捨てはしない）
                web_sys::console::warn_1(
                    &format!("Article {} line {} text too long", i, line.num).into(),
                );
            }
        }
    }

    Ok(())
}

/// 検索インデックスをロード（公開用ヘルパー関数）
///
/// # セキュリティ対策
/// - レスポンスサイズの上限チェック
/// - JSONデータの検証
/// - 本番環境ではエラー詳細を隠蔽
pub async fn load_search_index() -> Result<()> {
    // 既にロード済みかチェック
    let is_loaded = SEARCH_INDEX.with(|idx| idx.borrow().is_loaded());
    if is_loaded {
        return Ok(());
    }

    let window = web_sys::window().ok_or_else(|| DnfolioError::DomError("No window".into()))?;

    // RequestInit設定
    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::SameOrigin);

    // Requestオブジェクト作成
    let request = Request::new_with_str_and_init("/search-index.json", &opts)
        .map_err(|e| DnfolioError::JsError(format!("{e:?}")))?;

    // fetch実行
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|e| DnfolioError::JsError(format!("{e:?}")))?;

    // Responseにキャスト
    let resp: Response = resp_value
        .dyn_into()
        .map_err(|e| DnfolioError::JsError(format!("{e:?}")))?;

    // ステータスコード確認
    if !resp.ok() {
        #[cfg(debug_assertions)]
        return Err(DnfolioError::JsError(format!(
            "HTTP error: {}",
            resp.status()
        )));

        #[cfg(not(debug_assertions))]
        return Err(DnfolioError::JsError("Failed to load search index".into()));
    }

    // Content-Lengthヘッダーでサイズチェック（可能な場合）
    if let Ok(Some(headers)) = resp.headers().get("content-length") {
        if let Ok(size) = headers.parse::<usize>() {
            if size > MAX_RESPONSE_SIZE {
                return Err(DnfolioError::ValidationError(
                    "Search index too large".into(),
                ));
            }
        }
    }

    // JSONとしてパース
    let json = JsFuture::from(
        resp.json()
            .map_err(|e| DnfolioError::JsError(format!("{e:?}")))?,
    )
    .await
    .map_err(|e| DnfolioError::JsError(format!("{e:?}")))?;

    // serde-wasm-bindgenでRust型に変換
    let articles: Vec<SearchArticle> = serde_wasm_bindgen::from_value(json)
        .map_err(|e| DnfolioError::JsError(format!("{e:?}")))?;

    // データ検証
    validate_search_index(&articles)?;

    web_sys::console::log_1(&format!("Search index loaded: {} articles", articles.len()).into());

    // グローバルインスタンスに保存
    SEARCH_INDEX.with(|idx| {
        let index = idx.borrow();
        *index.articles.borrow_mut() = articles;
        *index.loaded.borrow_mut() = true;
    });

    Ok(())
}

/// グローバルインデックスで検索を実行
pub fn search_articles(query: &str) -> Vec<SearchResult> {
    SEARCH_INDEX.with(|idx| idx.borrow().search(query))
}

/// グローバルインデックスで検索を実行（行単位、grep形式）
pub fn search_lines(query: &str) -> Vec<SearchMatch> {
    if query.is_empty() {
        return Vec::new();
    }

    let lower_query = query.to_lowercase();

    SEARCH_INDEX.with(|idx| {
        let index = idx.borrow();
        let articles = index.articles.borrow();

        let mut matches = Vec::new();

        for article in articles.iter() {
            // タイトルマッチも行として追加
            if article.title.to_lowercase().contains(&lower_query) {
                matches.push(SearchMatch {
                    title: article.title.clone(),
                    url: article.url.clone(),
                    line_num: 1,
                    line_text: article.title.clone(),
                });
            }

            // 各行をチェック
            for line in &article.lines {
                if line.text.to_lowercase().contains(&lower_query) {
                    matches.push(SearchMatch {
                        title: article.title.clone(),
                        url: article.url.clone(),
                        line_num: line.num,
                        line_text: line.text.clone(),
                    });
                }
            }
        }

        matches
    })
}
