#![allow(non_snake_case)]
mod use_dialog;

use crate::hooks::use_dialog::UseDialog;
use dioxus::core::ScopeState;
use log::info;
use std::fmt::{Debug, Formatter};
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
/// use dioxus_use_dialog::use_dialog;
///
/// fn App(cx: Scope) -> Element {
///     let size = use_dialog(&cx);
///
///     cx.render(rsx!(
///         h1 { "Window size: {size}" }
///     ))
/// }
/// ```
#[inline]
pub fn use_dialog(cx: &ScopeState) -> UseDialog {
    UseDialog::new(cx)
    // cx.use_hook(|_| UseDialog::new(cx))
}
