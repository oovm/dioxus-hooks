mod builder;
mod use_charset;
mod use_document;
mod use_scroll;
mod use_title;

pub use self::{builder::UseDocumentBuilder, use_charset::UseCharacterSet, use_document::UseDocument, use_title::UseTitle};
use dioxus::core::ScopeState;
use gloo_events::EventListener;
use std::{cell::RefCell, rc::Rc};
use web_sys::{window, Document};

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

pub fn use_user_lang() {
    // window()?.navigator()?
}

pub fn use_theme() {}

pub fn use_lang() {}
