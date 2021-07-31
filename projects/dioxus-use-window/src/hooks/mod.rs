#![allow(non_snake_case)]
mod builder;
mod use_browser;
// mod use_full_screen;
mod use_height;
mod use_layout;
mod use_width;
mod use_window;

use self::use_window::data::*;
pub use self::{
    builder::UseWindowBuilder, use_browser::UseBrowser, use_height::UseWindowHeight, use_layout::UseWindowLayout,
    use_width::UseWindowWidth, use_window::UseWindow,
};
use crate::ResponsiveLayout;
use dioxus::core::ScopeState;
use gloo_events::EventListener;
use log::info;
use std::{
    cell::{Ref, RefCell},
    fmt::{Debug, Display, Formatter},
    marker::PhantomData,
    rc::Rc,
};
use wasm_bindgen::JsValue;
use web_sys::{window, Window};

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
pub fn use_window_size(cx: &ScopeState) -> &UseWindow {
    UseWindowBuilder::default().use_window(cx)
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
#[inline]
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
#[inline]
pub fn use_window_height(cx: &ScopeState) -> &UseWindowHeight {
    UseWindowBuilder::default().use_height(cx)
}

/// hooks for window's layout
///
/// # Arguments
///
/// returns: [`WindowLayout`]
///
/// # Examples
///
/// ```
/// use dioxus::prelude::*;
/// use dioxus_use_window::{ResponsiveLayout, use_window_layout};
///
/// fn App(cx: Scope) -> Element {
///     let layout = use_window_layout::<ResponsiveLayout>(&cx);
///
///     cx.render(rsx!(
///         h1 { "Window layout: {layout}" }
///     ))
/// }
/// ```
#[inline]
pub fn use_window_layout<T>(cx: &ScopeState) -> &UseWindowLayout<T>
where
    T: From<usize>,
    T: 'static,
{
    UseWindowBuilder::default().use_layout(cx)
}

/// hooks for window's layout
///
/// # Arguments
///
/// returns: [`WindowLayout`]
///
/// # Examples
///
/// ```
/// use dioxus::prelude::*;
/// use dioxus_use_window::{use_responsive_layout};
///
/// fn App(cx: Scope) -> Element {
///     let layout = use_responsive_layout(&cx);
///
///     cx.render(rsx!(
///         h1 { "Window layout: {layout}" }
///     ))
/// }
/// ```
#[inline]
pub fn use_responsive_layout(cx: &ScopeState) -> &UseWindowLayout<ResponsiveLayout> {
    UseWindowBuilder::default().use_responsive_layout(cx)
}

/// hooks for window's size
///
/// # Arguments
///
/// returns: [`UseBrowser`]
///
/// # Examples
///
/// ```
/// use dioxus::prelude::*;
/// use dioxus_use_window::use_browser;
///
/// fn App(cx: Scope) -> Element {
///     let size = use_browser(&cx);
///
///     cx.render(rsx!(
///         h1 { "Window size: {size}" }
///     ))
/// }
/// ```
#[inline]
pub fn use_browser(cx: &ScopeState) -> &UseBrowser {
    UseWindowBuilder::default().use_browser(cx)
}
