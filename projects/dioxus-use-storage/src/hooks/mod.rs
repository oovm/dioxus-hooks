#![allow(non_snake_case)]
mod builder;
mod use_local_storage;
mod use_session_storage;
// mod use_focus;
// mod use_hovering;
pub use self::{builder::UseCursorBuilder, use_cursor::UseLocalStorage, use_hover::UseHover};
use dioxus::{core::ScopeState, events::MouseData};
use gloo_events::EventListener;
use log::info;
use std::{
    cell::RefCell,
    fmt::{Debug, Display, Formatter},
    rc::Rc,
};
use wasm_bindgen::JsCast;
use web_sys::{window, Event, EventTarget, MouseEvent};

/// hooks for window's size
///
/// # Arguments
///
/// returns: [`UseCursor`]
///
/// # Examples
///
/// ```
/// use dioxus::prelude::*;
/// use dioxus_use_cursor::use_cursor;
///
/// fn App(cx: Scope) -> Element {
///     let cursor = use_cursor(&cx);
///
///     cx.render(rsx!(
///         h1 { "Cursor position: {cursor}" }
///     ))
/// }
/// ```
#[inline]
pub fn use_cursor(cx: &ScopeState) -> &UseLocalStorage {
    UseCursorBuilder::default().use_cursor(cx)
}

/// hooks for window's size
///
/// # Arguments
///
/// returns: [`UseHover`]
///
/// # Examples
///
/// ```
/// use dioxus::prelude::*;
/// use dioxus_use_cursor::{use_hover};
///
/// fn App(cx: Scope) -> Element {
///     let hover = use_hover(&cx);
///
///     cx.render(rsx!(
///         h1 { "Hover: {hover}" }
///     ))
/// }
/// ```
#[inline]
pub fn use_hover(cx: &ScopeState) -> &UseHover {
    UseCursorBuilder::default().use_hover(cx)
}
