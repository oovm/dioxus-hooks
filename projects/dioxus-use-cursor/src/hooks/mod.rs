mod builder;
mod use_cursor;
mod use_hover;
// mod use_focus;
// mod use_hovering;
use log::info;
use web_sys::EventTarget;
pub use self::builder::UseCursorBuilder;
pub use self::{
    use_cursor::UseCursor,
    use_hover::UseHover
};use web_sys::Event;
use dioxus::{core::ScopeState, events::MouseData};
use gloo_events::EventListener;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::JsCast;
use web_sys::{window, MouseEvent};

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
///         h1 { "Window size: {cursor}" }
///     ))
/// }
/// ```
#[inline]
pub fn use_cursor(cx: &ScopeState) -> &UseCursor {
    UseCursorBuilder::default().use_cursor(cx)
}

///
pub fn use_hover(_cx: &ScopeState) -> UseCursor {
    todo!()
}
