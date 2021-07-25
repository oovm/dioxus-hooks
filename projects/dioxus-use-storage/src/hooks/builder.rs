use super::*;

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
    /// * `cx`: [`Scope`] or [`ScopeState`]
    ///
    /// returns: [`WindowSize`]
    ///
    /// # Examples
    ///
    /// ```
    /// use dioxus::prelude::*;
    /// use dioxus_use_cursor::use_cursor;
    ///
    /// fn App(cx: Scope) -> Element {
    ///     let size = use_cursor(&cx);
    ///
    ///     cx.render(rsx!(
    ///         h1 { "Window size: {size}" }
    ///     ))
    /// }
    /// ```
    pub fn use_cursor<'a>(&self, cx: &'a ScopeState) -> &'a mut UseLocalStorage {
        let hook = UseLocalStorage::new(cx).unwrap_or_default();
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
    /// use dioxus_use_cursor::use_hover;
    ///
    /// fn App(cx: Scope) -> Element {
    ///     let hook = use_hover(&cx);
    ///
    ///     cx.render(rsx!(
    ///         h1 { "Cursor is hovering: {hook}" }
    ///     ))
    /// }
    /// ```
    pub fn use_local_storage<'a>(&self, cx: &'a ScopeState) -> &'a mut UseLocalStorage {
        let hook = UseLocalStorage::new(cx).unwrap_or(UseLocalStorage::new_ssr(cx));
        cx.use_hook(|_| hook)
    }
}