mod use_title;
mod use_document;
mod builder;

use dioxus::core::ScopeState;
use gloo_events::EventListener;
use web_sys::{window};
use std::cell::RefCell;
use std::rc::Rc;
use web_sys::Document;
pub use self::builder::UseDocumentBuilder;
pub use self::use_document::UseDocument;
pub use self::use_title::{UseTitle};


#[inline]
pub fn use_document(cx: &ScopeState) -> &UseDocument {
    UseDocumentBuilder::default().use_document(cx)
}

#[inline]
pub fn use_title(cx: &ScopeState) -> &UseTitle {
    UseDocumentBuilder::default().use_title(cx)
}

pub fn use_theme() {

}

pub fn use_lang() {

}


