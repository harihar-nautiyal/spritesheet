use leptos::*;

pub fn set_theme(theme: &str) {
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            if let Some(el) = document.document_element() {
                let _ = el.set_attribute("data-theme", theme);
            }
        }
    }
}
