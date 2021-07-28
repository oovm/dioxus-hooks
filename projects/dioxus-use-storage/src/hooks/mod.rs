#![allow(non_snake_case)]

mod builder;
mod storage_iter;
mod use_local_storage;
mod use_session_storage;

use self::storage_iter::{on_storage, UseStorageData};
pub use self::{builder::UseStorageBuilder, use_local_storage::UseLocalStorage};
use crate::hooks::use_session_storage::UseSessionStorage;
use dioxus::core::ScopeState;
use gloo_events::EventListener;
use log::{info, warn};
use std::{
    cell::RefCell,
    fmt::{Debug, Display, Formatter},
    marker::PhantomData,
    rc::Rc,
};
use wasm_bindgen::JsCast;
use web_sys::{window, Storage, StorageEvent, Window};

/// hooks for window's size with config
///
/// # Arguments
///
/// returns: [`WindowSize`]
///
/// # Examples
///
/// ```
/// use dioxus::prelude::*;
/// use dioxus_use_storage::{use_local_storage};
///
/// fn App(cx: Scope) -> Element {
///     let hook = use_local_storage(&cx);
///
///     cx.render(rsx!(
///         h1 { "Local Storage: {hook:#?}" }
///     ))
/// }
/// ```
#[inline]
pub fn use_local_storage(cx: &ScopeState) -> &UseLocalStorage {
    UseStorageBuilder::default().use_local_storage(cx)
}

/// hooks for window's size with config
///
/// # Arguments
///
/// returns: [`WindowSize`]
///
/// # Examples
///
/// ```
/// use dioxus::prelude::*;
/// use dioxus_use_storage::{use_session_storage};
///
/// fn App(cx: Scope) -> Element {
///     let hook = use_session_storage(&cx);
///
///     cx.render(rsx!(
///         h1 { "Local Storage: {hook}" }
///     ))
/// }
/// ```
#[inline]
pub fn use_session_storage(cx: &ScopeState) -> &UseSessionStorage {
    UseStorageBuilder::default().use_session_storage(cx)
}

///
#[derive(Debug)]
pub struct StorageIter<'a> {
    inner: Option<Storage>,
    count: u32,
    index: u32,
    bound: PhantomData<&'a ()>,
    // pub(crate) value: String,
}
