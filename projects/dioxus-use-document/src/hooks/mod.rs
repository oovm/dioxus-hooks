mod use_title;
mod use_document;

use dioxus::core::ScopeState;
use gloo_events::EventListener;
use web_sys::{window, Clipboard};
use std::cell::RefCell;
use std::rc::Rc;
use web_sys::Document;

/// effect handler
#[derive(Debug, Clone)]
pub struct ClipboardService {
    inner: Clipboard,
    // listeners: Vec<EventListener>
}

pub struct UseTitle {

}

impl UseTitle {
    pub fn new() {

    }
}



pub fn use_theme() {

}

pub fn use_title() {

}

pub fn use_lang() {

}

pub fn use_document() {

}

impl ClipboardService {
    /// do
    pub fn new(cx: &ScopeState) -> Option<Self> {
        let clipboard = window()?.navigator().clipboard()?;

        Some(Self { inner: clipboard })
    }

    // #[inline]
    // pub fn listen(&mut self, listener: EventListener) {
    //     // EventListener::new()
    //     self.listeners.push(listener)
    // }

    // pub fn read_text(&self) {
    //    self.inner.read_text();
    // }
}
