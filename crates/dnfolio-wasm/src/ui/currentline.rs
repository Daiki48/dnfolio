//! 現在行（current-line）ハイライト機能
//!
//! 行番号クリック時のシアン系ハイライト表示

use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

use crate::dom::document;
use crate::error::Result;

/// 指定した要素にcurrent-lineクラスを設定（他からは削除）
pub fn set_current_line(target: &HtmlElement) -> Result<()> {
    // 全てのcurrent-lineクラスを削除
    clear_current_line()?;

    // ターゲット要素にcurrent-lineクラスを追加
    target.class_list().add_1("current-line").ok();

    Ok(())
}

/// 全てのcurrent-lineクラスを削除
pub fn clear_current_line() -> Result<()> {
    let doc = document()?;

    let current_lines = doc.query_selector_all(".main-content > .current-line").ok();

    if let Some(elements) = current_lines {
        for i in 0..elements.length() {
            if let Some(el) = elements.get(i) {
                if let Some(html_el) = el.dyn_ref::<HtmlElement>() {
                    html_el.class_list().remove_1("current-line").ok();
                }
            }
        }
    }

    Ok(())
}

/// 行番号（::before擬似要素）のクリック位置から対象要素を特定
/// main-content直下のブロック要素かどうかを判定
pub fn is_line_number_click(target: &HtmlElement, click_x: f64) -> bool {
    // 行番号は左端にあるので、クリック位置がgutter領域内かチェック
    // gutter幅は約40-50px（CSSで定義）
    let rect = target.get_bounding_client_rect();
    let relative_x = click_x - rect.left();

    // 行番号領域（左端約50px）内のクリックかどうか
    relative_x < 50.0
}

/// main-content直下のブロック要素かどうかを判定
pub fn is_block_element(target: &HtmlElement) -> bool {
    if let Some(parent) = target.parent_element() {
        parent.class_list().contains("main-content")
    } else {
        false
    }
}

/// クリックされた要素からmain-content直下のブロック要素を取得
pub fn get_block_element(target: &HtmlElement) -> Option<HtmlElement> {
    // まず、targetがmain-content直下かチェック
    if is_block_element(target) {
        return Some(target.clone());
    }

    // そうでなければ、親を辿ってmain-content直下の要素を探す
    let mut current = target.parent_element();
    while let Some(el) = current {
        if let Some(parent) = el.parent_element() {
            if parent.class_list().contains("main-content") {
                return el.dyn_into::<HtmlElement>().ok();
            }
        }
        current = el.parent_element();
    }

    None
}
