//! OUTLINE（目次）追従機能
//!
//! スクロールに応じて現在表示中の見出しをハイライト

use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

use crate::dom::{document, query_selector_optional};
use crate::error::Result;

/// OUTLINEの現在位置を更新
pub fn update_active_heading() -> Result<()> {
    let doc = document()?;

    // main-content内の見出し要素を取得
    let headings = doc
        .query_selector_all(".main-content h1[id], .main-content h2[id], .main-content h3[id], .main-content h4[id]")
        .ok();

    let Some(headings) = headings else {
        return Ok(());
    };

    if headings.length() == 0 {
        return Ok(());
    }

    let Some(window) = web_sys::window() else {
        return Ok(());
    };

    let scroll_y = window.scroll_y().unwrap_or(0.0);
    // ヘッダー分のオフセット（60px）を考慮
    let threshold = scroll_y + 80.0;

    // 現在スクロール位置より上にある最後の見出しを探す
    let mut active_id: Option<String> = None;

    for i in 0..headings.length() {
        if let Some(heading) = headings.get(i) {
            if let Some(el) = heading.dyn_ref::<HtmlElement>() {
                let rect = el.get_bounding_client_rect();
                let heading_top = scroll_y + rect.top();

                if heading_top <= threshold {
                    active_id = el.id().into();
                } else {
                    // この見出しより下はスクロール位置より下にあるので終了
                    break;
                }
            }
        }
    }

    // 目次のアクティブ状態を更新
    update_toc_active_state(&active_id)?;

    Ok(())
}

/// 目次のアクティブ状態を更新
fn update_toc_active_state(active_id: &Option<String>) -> Result<()> {
    let doc = document()?;

    // 全ての目次アイテムからactiveを削除
    let toc_items = doc.query_selector_all(".toc-section .toc-item").ok();

    if let Some(items) = toc_items {
        for i in 0..items.length() {
            if let Some(item) = items.get(i) {
                if let Some(el) = item.dyn_ref::<HtmlElement>() {
                    el.class_list().remove_1("active").ok();
                }
            }
        }
    }

    // アクティブな見出しに対応する目次アイテムにactiveを追加
    if let Some(id) = active_id {
        // href="#id" を持つリンクの親要素を探す
        let selector = format!(".toc-section .toc-item a[href=\"#{}\"]", id);
        if let Some(link) = query_selector_optional::<HtmlElement>(&selector)? {
            if let Some(parent) = link.parent_element() {
                if let Some(parent_el) = parent.dyn_ref::<HtmlElement>() {
                    parent_el.class_list().add_1("active").ok();

                    // アクティブな項目が見えるようにスクロール（目次内で）
                    scroll_toc_item_into_view(parent_el)?;
                }
            }
        }
    }

    Ok(())
}

/// 目次アイテムを目次エリア内で見えるようにスクロール
fn scroll_toc_item_into_view(item: &HtmlElement) -> Result<()> {
    // .toc-section を探す
    if let Some(toc_section) = item.closest(".toc-section").ok().flatten() {
        if let Some(toc_el) = toc_section.dyn_ref::<HtmlElement>() {
            let item_rect = item.get_bounding_client_rect();
            let toc_rect = toc_el.get_bounding_client_rect();

            // アイテムがtocセクションの表示範囲外にある場合
            if item_rect.top() < toc_rect.top() || item_rect.bottom() > toc_rect.bottom() {
                // block: "nearest" でスムーズにスクロール
                let options = web_sys::ScrollIntoViewOptions::new();
                options.set_behavior(web_sys::ScrollBehavior::Smooth);
                options.set_block(web_sys::ScrollLogicalPosition::Nearest);
                item.scroll_into_view_with_scroll_into_view_options(&options);
            }
        }
    }

    Ok(())
}
