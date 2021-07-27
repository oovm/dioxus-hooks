use super::*;
use crate::hooks::use_session_storage::UseSessionStorage;

///
#[derive(Debug, Copy, Clone)]
pub struct UseStorageBuilder {}

impl Default for UseStorageBuilder {
    fn default() -> Self {
        Self {}
    }
}

impl UseStorageBuilder {
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
    /// use dioxus_use_storage::use_local_storage;
    ///
    /// fn App(cx: Scope) -> Element {
    ///     let hook = use_local_storage(&cx);
    ///
    ///     cx.render(rsx!(
    ///         h1 { "Local Storage: {hook}" }
    ///     ))
    /// }
    /// ```
    pub fn use_local_storage<'a>(&self, cx: &'a ScopeState) -> &'a mut UseLocalStorage {
        let hook = UseLocalStorage::new(cx).unwrap_or_else(|| UseLocalStorage::new_ssr(cx));
        cx.use_hook(|_| hook)
    }
    /// hooks for window's size with config
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
    /// use dioxus_use_storage::{use_session_storage};
    ///
    /// fn App(cx: Scope) -> Element {
    ///     let hook = use_session_storage(&cx);
    ///
    ///     cx.render(rsx!(
    ///         h1 { "Session storage: {hook}" }
    ///     ))
    /// }
    /// ```
    pub fn use_session_storage<'a>(&self, cx: &'a ScopeState) -> &'a mut UseSessionStorage {
        let hook = UseSessionStorage::new(cx).unwrap_or_else(|| UseSessionStorage::new_ssr(cx));
        cx.use_hook(|_| hook)
    }
}
