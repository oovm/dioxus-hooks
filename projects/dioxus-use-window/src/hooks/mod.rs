// #![deny(missing_doc_code_examples)]
mod builder;
mod use_height;
mod use_layout;
mod use_width;
mod use_window;

use self::use_window::WindowSizeData;
pub use self::{builder::UseWindowBuilder, use_window::UseWindowSize};
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
use web_sys::{window, Window};
use crate::hooks::use_layout::UseWindowLayout;
use crate::hooks::use_width::UseWindowWidth;

/// hooks for window's size
///
/// # Arguments
///
/// returns: [`UseWindowSize`]
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
pub fn use_window_size(cx: &ScopeState) -> &mut UseWindowSize {
    UseWindowBuilder::default().use_size(cx)
}

/// hooks for window's layout
///
/// # Arguments
///
/// * `layout`: [`ResponsiveLayout`]
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
pub fn use_window_layout<T>(cx: &ScopeState) -> &UseWindowLayout<T>
where
    T: From<usize>,
    T: 'static,
{
    UseWindowBuilder::default().use_layout(cx)
}

/// hooks for window's width
///
/// # Arguments
///
/// returns: [`UseWindowWidth`]
///
/// # Examples
///
/// ```
/// use dioxus::prelude::*;
/// use dioxus_use_window::use_window_width;
///
/// fn App(cx: Scope) -> Element {
///     let width = use_window_width(&cx);
///
///     cx.render(rsx!(
///         h1 { "Window width: {width}" }
///     ))
/// }
/// ```
pub fn use_window_width(cx: &ScopeState) -> &UseWindowWidth {
    UseWindowBuilder::default().use_width(cx)
}



/// hooks for window's height
///
/// # Arguments
///
/// returns: [`UseWindowHeight`]
///
/// # Examples
///
/// ```
/// use dioxus::prelude::*;
/// use dioxus_use_window::use_window_height;
///
/// fn App(cx: Scope) -> Element {
///     let height = use_window_height(&cx);
///
///     cx.render(rsx!(
///         h1 { "Window height: {height}" }
///     ))
/// }
/// ```
pub fn use_window_height(cx: &ScopeState) -> &UseWindowHeight {
    UseWindowBuilder::default().use_height(cx)
}
