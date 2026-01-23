//! DOM要素取得ヘルパー

use wasm_bindgen::JsCast;
use web_sys::{Document, Element, Window};

use crate::error::{DnfolioError, Result};

/// windowオブジェクトを取得
pub fn window() -> Result<Window> {
    web_sys::window().ok_or_else(|| DnfolioError::DomError("windowが見つかりません".to_string()))
}

/// documentオブジェクトを取得
pub fn document() -> Result<Document> {
    window()?
        .document()
        .ok_or_else(|| DnfolioError::DomError("documentが見つかりません".to_string()))
}

/// IDで要素を取得し、指定した型にキャスト
///
/// # Example
/// ```ignore
/// let input = get_element_by_id::<HtmlInputElement>("my-input")?;
/// ```
pub fn get_element_by_id<T>(id: &str) -> Result<T>
where
    T: JsCast,
{
    let doc = document()?;
    let element = doc
        .get_element_by_id(id)
        .ok_or_else(|| DnfolioError::ElementNotFound(format!("#{id}")))?;

    element
        .dyn_into::<T>()
        .map_err(|_| DnfolioError::DomError(format!("#{id} の型キャストに失敗")))
}

/// CSSセレクタで要素を取得し、指定した型にキャスト
///
/// # Example
/// ```ignore
/// let main = query_selector::<HtmlElement>(".main-content")?;
/// ```
pub fn query_selector<T>(selector: &str) -> Result<T>
where
    T: JsCast,
{
    let doc = document()?;
    let element = doc
        .query_selector(selector)
        .map_err(|e| DnfolioError::DomError(format!("セレクタエラー: {e:?}")))?
        .ok_or_else(|| DnfolioError::ElementNotFound(selector.to_string()))?;

    element
        .dyn_into::<T>()
        .map_err(|_| DnfolioError::DomError(format!("{selector} の型キャストに失敗")))
}

/// CSSセレクタで要素を取得（Elementのまま）
pub fn query_selector_element(selector: &str) -> Result<Element> {
    query_selector::<Element>(selector)
}

/// CSSセレクタで要素を取得（存在しない場合はNone）
pub fn query_selector_optional<T>(selector: &str) -> Result<Option<T>>
where
    T: JsCast,
{
    let doc = document()?;
    let element = doc
        .query_selector(selector)
        .map_err(|e| DnfolioError::DomError(format!("セレクタエラー: {e:?}")))?;

    match element {
        Some(el) => el
            .dyn_into::<T>()
            .map(Some)
            .map_err(|_| DnfolioError::DomError(format!("{selector} の型キャストに失敗"))),
        None => Ok(None),
    }
}

/// ローディングオーバーレイを表示
pub fn show_loading() -> Result<()> {
    use web_sys::HtmlElement;

    if let Some(overlay) = query_selector_optional::<HtmlElement>("#loading-overlay")? {
        overlay.class_list().add_1("visible").ok();
    }
    Ok(())
}

/// ローディングオーバーレイを非表示
pub fn hide_loading() -> Result<()> {
    use web_sys::HtmlElement;

    if let Some(overlay) = query_selector_optional::<HtmlElement>("#loading-overlay")? {
        overlay.class_list().remove_1("visible").ok();
    }
    Ok(())
}

// =============================================================================
// セキュリティヘルパー関数
// XSS対策として、set_inner_html()の代わりにこれらの関数を使用する
// =============================================================================

/// 要素の全ての子要素を安全に削除
///
/// set_inner_html("")の代わりに使用する
/// XSS脆弱性を防ぐため、DOM APIで個別に削除
pub fn clear_children(element: &Element) -> Result<()> {
    while let Some(child) = element.first_child() {
        element
            .remove_child(&child)
            .map_err(|e| DnfolioError::DomError(format!("子要素の削除に失敗: {e:?}")))?;
    }
    Ok(())
}

/// テキストコンテンツを持つdiv要素を安全に作成して追加
///
/// set_inner_html("<div>...</div>")の代わりに使用
/// XSS脆弱性を防ぐため、textContentを使用
pub fn create_text_div(parent: &Element, class_name: &str, text: &str) -> Result<Element> {
    let doc = document()?;
    let div = doc
        .create_element("div")
        .map_err(|e| DnfolioError::DomError(format!("div要素の作成に失敗: {e:?}")))?;
    div.set_class_name(class_name);
    div.set_text_content(Some(text));
    parent
        .append_child(&div)
        .map_err(|e| DnfolioError::DomError(format!("要素の追加に失敗: {e:?}")))?;
    Ok(div)
}

/// 指定した要素を安全に作成
pub fn create_element(tag: &str) -> Result<Element> {
    let doc = document()?;
    doc.create_element(tag)
        .map_err(|e| DnfolioError::DomError(format!("{tag}要素の作成に失敗: {e:?}")))
}

/// テキストノードを作成
pub fn create_text_node(text: &str) -> Result<web_sys::Text> {
    let doc = document()?;
    Ok(doc.create_text_node(text))
}

// =============================================================================
// 入力バリデーション関数
// =============================================================================

/// 入力値の最大長
pub const MAX_SEARCH_QUERY_LEN: usize = 500;
pub const MAX_COMMAND_LEN: usize = 1000;
pub const MAX_URL_LEN: usize = 2000;

/// 検索クエリをバリデーション（長さ制限）
///
/// 長すぎるクエリはReDoS攻撃やメモリ枯渇の原因になる
pub fn validate_search_query(query: &str) -> Result<&str> {
    if query.len() > MAX_SEARCH_QUERY_LEN {
        return Err(DnfolioError::DomError(format!(
            "検索クエリが長すぎます（最大{}文字）",
            MAX_SEARCH_QUERY_LEN
        )));
    }
    Ok(query)
}

/// コマンドをバリデーション（長さ制限）
pub fn validate_command(cmd: &str) -> Result<&str> {
    if cmd.len() > MAX_COMMAND_LEN {
        return Err(DnfolioError::DomError(format!(
            "コマンドが長すぎます（最大{}文字）",
            MAX_COMMAND_LEN
        )));
    }
    Ok(cmd)
}

/// URLをバリデーション
///
/// - 相対パス（/で始まり、//で始まらない）のみ許可
/// - javascript:, data: などの危険なスキームを拒否
/// - protocol-relative URL（//example.com）を拒否
/// - 長さ制限
pub fn validate_url(url: &str) -> Result<&str> {
    // 長さチェック
    if url.len() > MAX_URL_LEN {
        return Err(DnfolioError::DomError("URLが長すぎます".into()));
    }

    // 空文字チェック
    if url.is_empty() {
        return Err(DnfolioError::DomError("URLが空です".into()));
    }

    // Protocol-relative URL（//で始まる）を先に拒否
    // これは外部サイトへのリダイレクトに悪用される
    if url.starts_with("//") {
        return Err(DnfolioError::DomError(
            "Protocol-relative URLは許可されていません".into(),
        ));
    }

    // #で始まるフラグメントは許可
    if url.starts_with('#') {
        return Ok(url);
    }

    // /で始まる相対パスは許可（//は上で除外済み）
    if url.starts_with('/') {
        return Ok(url);
    }

    // 危険なスキームをチェック（大文字小文字を無視）
    let lower_url = url.to_lowercase();
    let dangerous_schemes = [
        "javascript:",
        "data:",
        "vbscript:",
        "file:",
        "blob:", // Blob URLも危険な可能性
    ];
    for scheme in dangerous_schemes {
        if lower_url.starts_with(scheme) {
            return Err(DnfolioError::DomError(format!(
                "危険なURLスキームです: {}",
                scheme
            )));
        }
    }

    // http/httpsは外部URLなので拒否
    if lower_url.starts_with("http://") || lower_url.starts_with("https://") {
        return Err(DnfolioError::DomError("外部URLは許可されていません".into()));
    }

    // その他のスキーム（mailto:, tel:, etc.）は拒否
    if url.contains(':') {
        return Err(DnfolioError::DomError("無効なURLフォーマットです".into()));
    }

    // 上記に該当しないパス（例: "path/to/page"）は許可
    Ok(url)
}

/// フラグメント（アンカー）をバリデーション
///
/// #で始まるフラグメントが安全な文字のみを含むかチェック
pub fn validate_fragment(fragment: &str) -> Result<&str> {
    let id = fragment.trim_start_matches('#');

    // 空のフラグメントは許可
    if id.is_empty() {
        return Ok(fragment);
    }

    // 英数字、ハイフン、アンダースコア、日本語のみ許可
    // （slugifyされたIDを想定）
    for c in id.chars() {
        if !c.is_alphanumeric() && c != '-' && c != '_' {
            return Err(DnfolioError::DomError(format!(
                "無効なフラグメント文字: {}",
                c
            )));
        }
    }

    Ok(fragment)
}
