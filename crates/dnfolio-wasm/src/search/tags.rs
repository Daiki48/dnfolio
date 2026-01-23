//! タグ検索機能
//!
//! tags-index.jsonを使用したタグ一覧表示とフィルタリング
//!
//! # セキュリティ対策
//!
//! - タグ数・記事数の上限チェック
//! - 各フィールドの長さ検証
//! - URL形式の検証

use std::cell::RefCell;

use serde::Deserialize;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{HtmlElement, Request, RequestInit, RequestMode, Response};

use crate::dom::{
    clear_children, create_text_div, document, query_selector_optional, validate_url,
};
use crate::error::{DnfolioError, Result};

// =============================================================================
// セキュリティ定数
// =============================================================================

/// タグの最大数
const MAX_TAGS: usize = 1000;
/// 1タグあたりの最大記事数
const MAX_ARTICLES_PER_TAG: usize = 500;
/// タグ名の最大長
const MAX_TAG_NAME_LEN: usize = 200;
/// URLの最大長
const MAX_URL_LEN: usize = 500;
/// タイトルの最大長
const MAX_TITLE_LEN: usize = 500;

/// タグに関連する記事情報
#[derive(Debug, Clone, Deserialize)]
pub struct TagArticle {
    pub date: Option<String>,
    pub title: String,
    pub url: String,
}

/// タグ情報
#[derive(Debug, Clone, Deserialize)]
pub struct TagInfo {
    pub name: String,
    pub url: String,
    pub count: usize,
    pub articles: Vec<TagArticle>,
}

/// タグインデックスデータを検証
///
/// # セキュリティ対策
/// - タグ数の上限チェック
/// - 各フィールドの長さ検証
/// - URL形式の検証（相対パスのみ許可）
fn validate_tags_index(tags: &[TagInfo]) -> Result<()> {
    // タグ数チェック
    if tags.len() > MAX_TAGS {
        return Err(DnfolioError::ValidationError(format!(
            "Too many tags: {} (max: {})",
            tags.len(),
            MAX_TAGS
        )));
    }

    for (i, tag) in tags.iter().enumerate() {
        // タグ名長チェック
        if tag.name.len() > MAX_TAG_NAME_LEN {
            web_sys::console::warn_1(&format!("Tag {} name too long", i).into());
        }

        // タグURL検証（相対パスのみ許可）
        if tag.url.len() > MAX_URL_LEN {
            return Err(DnfolioError::ValidationError(format!(
                "Tag {} URL too long",
                i
            )));
        }
        // Protocol-relative URL（//で始まる）を拒否
        if tag.url.starts_with("//") {
            return Err(DnfolioError::ValidationError(format!(
                "Tag {} has protocol-relative URL (not allowed)",
                i
            )));
        }
        if !tag.url.starts_with('/') {
            return Err(DnfolioError::ValidationError(format!(
                "Tag {} has invalid URL (must be relative path)",
                i
            )));
        }
        // 危険なURLスキームをチェック
        let lower_url = tag.url.to_lowercase();
        let dangerous_patterns = ["javascript:", "data:", "vbscript:", "blob:"];
        for pattern in dangerous_patterns {
            if lower_url.contains(pattern) {
                return Err(DnfolioError::ValidationError(format!(
                    "Tag {} has dangerous URL pattern",
                    i
                )));
            }
        }

        // 記事数チェック
        if tag.articles.len() > MAX_ARTICLES_PER_TAG {
            web_sys::console::warn_1(
                &format!(
                    "Tag {} has too many articles: {} (max: {})",
                    i,
                    tag.articles.len(),
                    MAX_ARTICLES_PER_TAG
                )
                .into(),
            );
        }

        // 各記事のURL検証
        for (j, article) in tag.articles.iter().enumerate() {
            if article.url.len() > MAX_URL_LEN {
                web_sys::console::warn_1(&format!("Tag {} article {} URL too long", i, j).into());
            }
            // Protocol-relative URL（//で始まる）を拒否
            if article.url.starts_with("//") {
                return Err(DnfolioError::ValidationError(format!(
                    "Tag {} article {} has protocol-relative URL",
                    i, j
                )));
            }
            if !article.url.starts_with('/') {
                return Err(DnfolioError::ValidationError(format!(
                    "Tag {} article {} has invalid URL",
                    i, j
                )));
            }
            if article.title.len() > MAX_TITLE_LEN {
                web_sys::console::warn_1(&format!("Tag {} article {} title too long", i, j).into());
            }
        }
    }

    Ok(())
}

/// タグインデックス
pub struct TagsIndex {
    tags: RefCell<Vec<TagInfo>>,
    loaded: RefCell<bool>,
}

impl TagsIndex {
    pub fn new() -> Self {
        Self {
            tags: RefCell::new(Vec::new()),
            loaded: RefCell::new(false),
        }
    }

    /// tags-index.jsonを非同期でロード
    ///
    /// # セキュリティ対策
    /// - HTTPステータスコードの確認
    /// - データ検証の実行
    pub async fn load(&self) -> Result<()> {
        if *self.loaded.borrow() {
            return Ok(());
        }

        let window = web_sys::window().ok_or_else(|| DnfolioError::DomError("No window".into()))?;

        let opts = RequestInit::new();
        opts.set_method("GET");
        opts.set_mode(RequestMode::SameOrigin);

        let request = Request::new_with_str_and_init("/tags-index.json", &opts)
            .map_err(|e| DnfolioError::JsError(format!("{e:?}")))?;

        let resp_value = JsFuture::from(window.fetch_with_request(&request))
            .await
            .map_err(|e| DnfolioError::JsError(format!("{e:?}")))?;

        let resp: Response = resp_value
            .dyn_into()
            .map_err(|e| DnfolioError::JsError(format!("{e:?}")))?;

        // HTTPステータスコードの確認
        if !resp.ok() {
            #[cfg(debug_assertions)]
            return Err(DnfolioError::JsError(format!(
                "HTTP error: {}",
                resp.status()
            )));

            #[cfg(not(debug_assertions))]
            return Err(DnfolioError::JsError("Failed to load tags index".into()));
        }

        let json = JsFuture::from(
            resp.json()
                .map_err(|e| DnfolioError::JsError(format!("{e:?}")))?,
        )
        .await
        .map_err(|e| DnfolioError::JsError(format!("{e:?}")))?;

        let tags: Vec<TagInfo> = serde_wasm_bindgen::from_value(json)
            .map_err(|e| DnfolioError::JsError(format!("{e:?}")))?;

        // データ検証
        validate_tags_index(&tags)?;

        web_sys::console::log_1(&format!("Tags index loaded: {} tags", tags.len()).into());

        *self.tags.borrow_mut() = tags;
        *self.loaded.borrow_mut() = true;

        Ok(())
    }

    /// クエリでタグをフィルタリング
    pub fn filter(&self, query: &str) -> Vec<TagInfo> {
        let tags = self.tags.borrow();

        if query.is_empty() {
            return tags.clone();
        }

        let lower_query = query.to_lowercase();
        tags.iter()
            .filter(|tag| tag.name.to_lowercase().contains(&lower_query))
            .cloned()
            .collect()
    }

    /// 全タグを取得
    pub fn all(&self) -> Vec<TagInfo> {
        self.tags.borrow().clone()
    }

    /// ロード済みか
    pub fn is_loaded(&self) -> bool {
        *self.loaded.borrow()
    }
}

impl Default for TagsIndex {
    fn default() -> Self {
        Self::new()
    }
}

// グローバルインスタンス
thread_local! {
    pub static TAGS_INDEX: RefCell<TagsIndex> = RefCell::new(TagsIndex::new());
}

/// タグモーダルの状態
pub struct TagsModalState {
    /// フィルタリング後のタグ一覧
    tags: RefCell<Vec<TagInfo>>,
    /// 選択中のインデックス
    selected_index: RefCell<usize>,
    /// 現在のフィルタクエリ
    current_query: RefCell<String>,
}

impl TagsModalState {
    pub fn new() -> Self {
        Self {
            tags: RefCell::new(Vec::new()),
            selected_index: RefCell::new(0),
            current_query: RefCell::new(String::new()),
        }
    }

    /// タグ一覧を表示
    pub fn render_tags(&self) -> Result<()> {
        let list = query_selector_optional::<HtmlElement>("#tags-list")?
            .ok_or_else(|| DnfolioError::ElementNotFound("#tags-list".into()))?;

        let doc = document()?;
        // XSS対策: set_inner_html("")の代わりに安全な方法で子要素を削除
        clear_children(&list)?;

        let tags = self.tags.borrow();
        let selected = *self.selected_index.borrow();
        let query = self.current_query.borrow().clone();

        for (i, tag) in tags.iter().enumerate() {
            let item = doc
                .create_element("div")
                .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;

            item.set_class_name(if i == selected {
                "tag-result-item selected"
            } else {
                "tag-result-item"
            });

            // タグ名（ハイライト付き）
            let name_span = doc
                .create_element("span")
                .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;
            name_span.set_class_name("tag-name");
            self.render_highlighted_text(&doc, &name_span, &tag.name, &query)?;

            // 記事数
            let count_span = doc
                .create_element("span")
                .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;
            count_span.set_class_name("tag-count");
            count_span.set_text_content(Some(&format!("{}", tag.count)));

            item.append_child(&name_span)
                .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;
            item.append_child(&count_span)
                .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;

            // data属性
            item.set_attribute("data-url", &tag.url)
                .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;
            item.set_attribute("data-index", &i.to_string())
                .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;

            list.append_child(&item)
                .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;
        }

        // 選択アイテムをスクロールして見える位置に
        self.scroll_selected_into_view()?;

        Ok(())
    }

    /// ハイライト付きテキストを描画
    fn render_highlighted_text(
        &self,
        doc: &web_sys::Document,
        container: &web_sys::Element,
        text: &str,
        query: &str,
    ) -> Result<()> {
        if query.is_empty() {
            container.set_text_content(Some(text));
            return Ok(());
        }

        let lower_text = text.to_lowercase();
        let lower_query = query.to_lowercase();
        let mut last_end = 0;

        for (start, _) in lower_text.match_indices(&lower_query) {
            // マッチ前のテキスト
            if start > last_end {
                let before_text = doc.create_text_node(&text[last_end..start]);
                container
                    .append_child(&before_text)
                    .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;
            }

            // マッチ部分（ハイライト）
            let mark = doc
                .create_element("mark")
                .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;
            mark.set_class_name("tags-keyword-highlight");
            mark.set_text_content(Some(&text[start..start + query.len()]));
            container
                .append_child(&mark)
                .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;

            last_end = start + query.len();
        }

        // 残りのテキスト
        if last_end < text.len() {
            let after_text = doc.create_text_node(&text[last_end..]);
            container
                .append_child(&after_text)
                .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;
        }

        Ok(())
    }

    /// プレビュー（関連記事一覧）を描画
    pub fn render_preview(&self) -> Result<()> {
        let preview = query_selector_optional::<HtmlElement>("#tags-preview")?
            .ok_or_else(|| DnfolioError::ElementNotFound("#tags-preview".into()))?;

        let doc = document()?;
        // XSS対策: set_inner_html("")の代わりに安全な方法で子要素を削除
        clear_children(&preview)?;

        let tags = self.tags.borrow();
        let selected = *self.selected_index.borrow();

        if let Some(tag) = tags.get(selected) {
            // タグ名ヘッダー
            let header = doc
                .create_element("div")
                .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;
            header.set_class_name("tags-preview-header");
            header.set_text_content(Some(&format!("{} ({})", tag.name, tag.count)));
            preview
                .append_child(&header)
                .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;

            // 記事一覧
            let articles_div = doc
                .create_element("div")
                .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;
            articles_div.set_class_name("tags-preview-articles");

            for article in &tag.articles {
                let article_div = doc
                    .create_element("div")
                    .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;
                article_div.set_class_name("tags-preview-article");

                // 日付（あれば）
                if let Some(date) = &article.date {
                    let date_span = doc
                        .create_element("span")
                        .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;
                    date_span.set_class_name("tags-preview-date");
                    date_span.set_text_content(Some(date));
                    article_div
                        .append_child(&date_span)
                        .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;
                }

                // タイトル
                let title_span = doc
                    .create_element("span")
                    .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;
                title_span.set_class_name("tags-preview-title");
                title_span.set_text_content(Some(&article.title));
                article_div
                    .append_child(&title_span)
                    .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;

                articles_div
                    .append_child(&article_div)
                    .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;
            }

            preview
                .append_child(&articles_div)
                .map_err(|e| DnfolioError::DomError(format!("{e:?}")))?;
        } else {
            // XSS対策: set_inner_htmlの代わりに安全なDOM操作を使用
            create_text_div(&preview, "tags-preview-empty", "No tags")?;
        }

        Ok(())
    }

    /// 件数表示を更新
    pub fn update_count(&self) -> Result<()> {
        if let Some(count_el) = query_selector_optional::<HtmlElement>("#tags-count")? {
            let count = self.tags.borrow().len();
            count_el.set_text_content(Some(&format!("{} tags", count)));
        }
        Ok(())
    }

    /// 選択を下に移動
    pub fn move_down(&self) -> Result<()> {
        let len = self.tags.borrow().len();
        if len == 0 {
            return Ok(());
        }
        let current = *self.selected_index.borrow();
        *self.selected_index.borrow_mut() = (current + 1) % len;
        self.render_tags()?;
        self.render_preview()?;
        Ok(())
    }

    /// 選択を上に移動
    pub fn move_up(&self) -> Result<()> {
        let len = self.tags.borrow().len();
        if len == 0 {
            return Ok(());
        }
        let current = *self.selected_index.borrow();
        *self.selected_index.borrow_mut() = if current == 0 { len - 1 } else { current - 1 };
        self.render_tags()?;
        self.render_preview()?;
        Ok(())
    }

    /// 選択中のタグページを開く
    pub fn open_selected(&self) -> Result<()> {
        let tags = self.tags.borrow();
        let selected = *self.selected_index.borrow();

        if let Some(tag) = tags.get(selected) {
            // セキュリティ: URLをバリデーション
            validate_url(&tag.url)?;

            // ローディング表示
            crate::dom::show_loading()?;

            if let Some(window) = web_sys::window() {
                let _ = window.location().set_href(&tag.url);
            }
        }
        Ok(())
    }

    /// フィルタ結果を設定
    pub fn set_tags(&self, tags: Vec<TagInfo>) {
        *self.tags.borrow_mut() = tags;
        *self.selected_index.borrow_mut() = 0;
    }

    /// クエリを設定
    pub fn set_query(&self, query: &str) {
        *self.current_query.borrow_mut() = query.to_string();
    }

    /// 状態をクリア
    pub fn clear(&self) -> Result<()> {
        *self.tags.borrow_mut() = Vec::new();
        *self.selected_index.borrow_mut() = 0;
        *self.current_query.borrow_mut() = String::new();
        Ok(())
    }

    /// 選択アイテムをスクロールして見える位置に
    fn scroll_selected_into_view(&self) -> Result<()> {
        let selected = *self.selected_index.borrow();
        let selector = format!(".tag-result-item[data-index=\"{}\"]", selected);

        if let Some(item) = query_selector_optional::<HtmlElement>(&selector)? {
            let options = web_sys::ScrollIntoViewOptions::new();
            options.set_behavior(web_sys::ScrollBehavior::Instant);
            options.set_block(web_sys::ScrollLogicalPosition::Nearest);
            item.scroll_into_view_with_scroll_into_view_options(&options);
        }
        Ok(())
    }
}

impl Default for TagsModalState {
    fn default() -> Self {
        Self::new()
    }
}

// グローバル状態
thread_local! {
    pub static TAGS_MODAL_STATE: RefCell<TagsModalState> = RefCell::new(TagsModalState::new());
}

// ヘルパー関数群

/// タグインデックスをロード
pub async fn load_tags_index() -> Result<()> {
    // TagsIndexのloadはRefCell内部で完結するので、直接呼び出す
    let already_loaded = TAGS_INDEX.with(|idx| idx.borrow().is_loaded());
    if already_loaded {
        return Ok(());
    }

    // ロード実行（thread_localから取り出してload）
    let tags_index = TagsIndex::new();
    tags_index.load().await?;

    // 結果を保存
    let tags = tags_index.tags.borrow().clone();
    TAGS_INDEX.with(|idx| {
        *idx.borrow().tags.borrow_mut() = tags;
        *idx.borrow().loaded.borrow_mut() = true;
    });

    Ok(())
}

/// タグをフィルタリング
pub fn filter_tags(query: &str) -> Vec<TagInfo> {
    TAGS_INDEX.with(|idx| idx.borrow().filter(query))
}

/// フィルタを実行して結果を更新
pub async fn perform_tags_filter(query: String) -> Result<()> {
    // インデックスをロード（初回のみ）
    load_tags_index().await?;

    // フィルタリング実行
    let filtered = filter_tags(&query);

    // モーダル状態を更新
    TAGS_MODAL_STATE.with(|state| {
        let state = state.borrow();
        state.set_query(&query);
        state.set_tags(filtered);
        state.render_tags()?;
        state.render_preview()?;
        state.update_count()
    })
}

/// 選択を下に移動
pub fn tags_modal_move_down() -> Result<()> {
    TAGS_MODAL_STATE.with(|state| state.borrow().move_down())
}

/// 選択を上に移動
pub fn tags_modal_move_up() -> Result<()> {
    TAGS_MODAL_STATE.with(|state| state.borrow().move_up())
}

/// 選択中のタグを開く
pub fn tags_modal_open_selected() -> Result<()> {
    TAGS_MODAL_STATE.with(|state| state.borrow().open_selected())
}

/// モーダル状態をクリア
pub fn tags_modal_clear() -> Result<()> {
    TAGS_MODAL_STATE.with(|state| state.borrow().clear())
}
