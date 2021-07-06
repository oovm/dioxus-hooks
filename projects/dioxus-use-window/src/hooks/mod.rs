#[deny(missing_doc_code_examples)]
mod display;
mod methods;

use dioxus::core::ScopeState;
use gloo_events::EventListener;
use std::{
    fmt::{Display, Formatter},
    marker::PhantomData,
};
use web_sys::window;

const MISSING_W: f64 = 375.0;
const MISSING_H: f64 = 812.0;

/// Window size effect handler
#[derive(Debug)]
pub struct WindowSize {
    x: f64,
    y: f64,
    listener: Option<EventListener>,
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
pub fn use_window_size(cx: &ScopeState) -> WindowSize {
    WindowSize::new(&cx, MISSING_W, MISSING_H).unwrap_or_default()
}

/// Window layout effect handler
#[derive(Debug)]
pub struct WindowLayout<T> {
    inner: WindowSize,
    bound: PhantomData<T>,
}

/// hooks for window's layout
///
/// # Arguments
///
/// * `cx`: [`Scope`] or [`ScopeState`]
///
/// returns: [`WindowLayout`]
///
/// # Examples
///
/// ```
/// use dioxus::prelude::*;
/// use dioxus_use_window::use_window_layout;
///
/// fn App(cx: Scope) -> Element {
///     let layout = use_window_layout(&cx);
///
///     cx.render(rsx!(
///         h1 { "Window layout: {layout}" }
///     ))
/// }
/// ```
pub fn use_window_layout<T>(cx: &ScopeState) -> WindowLayout<T>
where
    T: From<usize>,
{
    WindowLayout { inner: WindowSize::new(cx, MISSING_W, MISSING_H).unwrap_or_default(), bound: Default::default() }
}

/// Window width effect handler
#[derive(Debug)]
pub struct WindowWidth {
    inner: WindowSize,
}

/// hooks for window's width
///
/// # Arguments
///
/// * `cx`: [`Scope`] or [`ScopeState`]
///
/// returns: [`WindowWidth`]
///
/// # Examples
///
/// ```
/// use dioxus::prelude::*;
/// use dioxus_use_window::use_width;
///
/// fn App(cx: Scope) -> Element {
///     let width = use_width(&cx);
///
///     cx.render(rsx!(
///         h1 { "Window width: {width}" }
///     ))
/// }
/// ```
pub fn use_width(cx: &ScopeState) -> WindowWidth {
    WindowWidth { inner: WindowSize::new(cx, MISSING_W, MISSING_H).unwrap_or_default() }
}

/// Window height effect handler
#[derive(Debug)]
pub struct WindowHeight {
    inner: WindowSize,
}

/// hooks for window's height
///
/// # Arguments
///
/// * `cx`: [`Scope`] or [`ScopeState`]
///
/// returns: [`WindowHeight`]
///
/// # Examples
///
/// ```
/// use dioxus::prelude::*;
/// use dioxus_use_window::use_height;
///
/// fn App(cx: Scope) -> Element {
///     let height = use_height(&cx);
///
///     cx.render(rsx!(
///         h1 { "Window height: {height}" }
///     ))
/// }
/// ```
pub fn use_height(cx: &ScopeState) -> WindowHeight {
    WindowHeight { inner: WindowSize::new(cx, MISSING_W, MISSING_H).unwrap_or_default() }
}
