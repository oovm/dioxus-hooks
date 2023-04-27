#![allow(non_snake_case)]

pub mod builder;
pub mod storage_iter;
pub mod local_storage;
pub mod session_storage;

use crate::{StorageIter, UseLocalStorage, UseSessionStorage};
use self::storage_iter::{on_storage, UseStorageData};
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
use crate::UseStorageBuilder;

/// A hook for handle local storage
///
/// # Arguments
///
/// returns: [`UseLocalStorage`]
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

/// A hook for handle session storage
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
