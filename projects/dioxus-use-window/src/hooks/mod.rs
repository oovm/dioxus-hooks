#[deny(missing_doc_code_examples)]

mod display;
mod window_size;

use dioxus::core::ScopeState;
use gloo_events::EventListener;
use std::{
    fmt::{Display, Formatter},
    marker::PhantomData,
};
use web_sys::window;

/// Window size effect handler
#[derive(Debug)]
pub struct WindowSize {
    x: f64,
    y: f64,
    listener: Option<EventListener>,
}


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
pub fn use_window_size(cx: &ScopeState) -> WindowSize {
    WindowSize::new(&cx).unwrap()
}

/// Window size handler
#[derive(Debug)]
pub struct WindowLayout<T> {
    inner: WindowSize,
    bound: PhantomData<T>,
}

/// hook of
pub fn use_window_layout<T>(cx: &ScopeState) -> WindowLayout<T>
where
    T: From<usize>,
{
    WindowLayout { inner: WindowSize::new(cx).unwrap(), bound: Default::default() }
}

/// w
#[derive(Debug)]pub struct WindowWidth {
    inner: WindowSize,
}

/// hook
pub fn use_width(cx: &ScopeState) -> WindowWidth {
    WindowWidth { inner: WindowSize::new(cx).unwrap() }
}

/// hook
#[derive(Debug)]pub struct WindowHeight {
    inner: WindowSize,
}


/// hook of
pub fn use_height(cx: &ScopeState) -> WindowHeight {
    WindowHeight { inner: WindowSize::new(cx).unwrap() }
}
