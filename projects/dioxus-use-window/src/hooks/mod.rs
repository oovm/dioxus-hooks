// #![deny(missing_doc_code_examples)]
mod builder;
mod use_height;
mod use_layout;
mod use_width;
mod use_window;

use self::use_window::WindowSizeData;
pub use self::{builder::UseWindowBuilder, use_window::WindowSize};
use dioxus::core::ScopeState;
use gloo_events::EventListener;
use log::info;
use std::{
    cell::RefCell,
    fmt::{Debug, Display, Formatter},
    marker::PhantomData,
    rc::Rc,
};
use wasm_bindgen::JsValue;
use web_sys::{window, EventTarget, Window};

const MISSING_W: f64 = 375.0;
const MISSING_H: f64 = 812.0;

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
pub fn use_window_size(cx: &ScopeState) -> &mut WindowSize {
    UseWindowBuilder::default().use_window_size(cx)
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
/// use dioxus_use_window::{use_window_layout, ResponsiveLayout};
///
/// fn App(cx: Scope) -> Element {
///     let layout = use_window_layout::<ResponsiveLayout>(&cx);
///
///     cx.render(rsx!(
///         h1 { "Window layout: {layout}" }
///     ))
/// }
/// ```
pub fn use_window_layout<T>(cx: &ScopeState) -> &WindowLayout<T>
where
    T: From<usize>,
    T: 'static,
{
    cx.use_hook(|_| WindowLayout {
        inner: WindowSize::new(cx, MISSING_W, MISSING_H).unwrap_or_default(),
        bound: Default::default(),
    })
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
pub fn use_width(cx: &ScopeState) -> &WindowWidth {
    cx.use_hook(|_| WindowWidth { inner: WindowSize::new(cx, MISSING_W, MISSING_H).unwrap_or_default() })
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
pub fn use_height(cx: &ScopeState) -> &WindowHeight {
    cx.use_hook(|_| WindowHeight { inner: WindowSize::new(cx, MISSING_W, MISSING_H).unwrap_or_default() })
}
