mod use_title;
mod use_document;
mod use_charset;
mod builder;
mod use_html_lang;

use dioxus::core::ScopeState;
use gloo_events::EventListener;
use web_sys::{Element, window};
use std::cell::RefCell;
use std::rc::Rc;
use web_sys::Document;
pub use self::builder::UseDocumentBuilder;
pub use self::use_document::UseDocument;
pub use self::use_title::{UseTitle};
pub use self::use_charset::{UseCharacterSet};

#[inline]
pub fn use_document(cx: &ScopeState) -> &UseDocument {
    UseDocumentBuilder::default().use_document(cx)
}

#[inline]
pub fn use_title(cx: &ScopeState) -> &UseTitle {
    UseDocumentBuilder::default().use_title(cx)
}

#[inline]
pub fn use_charset(cx: &ScopeState) -> &UseCharacterSet {
    UseDocumentBuilder::default().use_charset(cx)
}


pub fn use_html_lang() {
    // window()?.navigator()?
}




impl UseHtmlLanguage {
    pub(crate) fn new() {
        let html = window()?.document()?.get_elements_by_tag_name("html").get_with_index(0)?;
        // TODO: listen attribute
    }
}


pub fn use_user_lang() -> Option<String> {


}

pub fn use_theme() {

}



pub fn use_lang() {

}


