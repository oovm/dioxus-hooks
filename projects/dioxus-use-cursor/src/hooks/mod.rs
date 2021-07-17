mod builder;
mod display;
mod methods;

pub use self::builder::UseCursorBuilder;
use self::builder::{UseCursorData, UseHoverData};
use dioxus::{core::ScopeState, events::MouseData};
use gloo_events::EventListener;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::JsCast;
use web_sys::{window, MouseEvent};

/// effect handler
pub struct UseCursor {
    data: Rc<RefCell<UseCursorData>>,
    listen_mouse_move: Option<EventListener>,
}

/// hooks for window's size
///
/// # Arguments
///
/// * `cx`: [`Scope`] or [`ScopeState`]
///
/// returns: [`WindowSize`]
///
/// # Examples
///
/// ```
/// use dioxus::prelude::*;
/// use dioxus_use_window::use_window_size;
///
/// fn App(cx: Scope) -> Element {
///     let size = use_window_size(&cx);
///
///     cx.render(rsx!(
///         h1 { "Window size: {size}" }
///     ))
/// }
/// ```
#[inline]
pub fn use_cursor(cx: &ScopeState) -> &UseCursor {
    UseCursorBuilder::default().use_cursor(cx)
}

///
#[derive(Debug, Clone)]
pub struct UseHover {
    data: UseHoverData,
}
///
pub fn use_hover(_cx: &ScopeState) -> UseCursor {
    todo!()
}
