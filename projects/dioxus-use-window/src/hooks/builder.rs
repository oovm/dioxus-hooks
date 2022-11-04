use super::*;
use crate::ResponsiveLayout;

/// Build window size hook with config
#[derive(Debug, Copy, Clone)]
pub struct UseWindowBuilder {
    ///
    pub missing_x: usize,
    ///
    pub missing_y: usize,
}

impl Default for UseWindowBuilder {
    fn default() -> Self {
        Self { missing_x: 375, missing_y: 812 }
    }
}

impl UseWindowBuilder {
    /// hooks for window's width with config
    ///
    /// # Arguments
    ///
    /// returns: [`UseWindowWidth`]
    ///
    /// # Examples
    ///
    /// ```
    /// use dioxus::prelude::*;
    /// use dioxus_use_window::{UseWindowBuilder};
    ///
    /// fn App(cx: Scope) -> Element {
    ///     let hook = UseWindowBuilder::default().use_width(&cx);
    ///
    ///     cx.render(rsx!(
    ///         h1 { "Window width: {hook}" }
    ///     ))
    /// }
    /// ```
    pub fn use_width<'a>(&self, cx: &'a ScopeState) -> &'a mut UseWindowWidth {
        let hook = UseWindowWidth::new(self.use_window_hook(cx));
        cx.use_hook(|| hook)
    }

    /// hooks for window's layout with config
    ///
    /// # Arguments
    ///
    /// returns: [`UseWindowLayout`]
    ///
    /// # Examples
    ///
    /// ```
    /// use dioxus::prelude::*;
    /// use dioxus_use_window::{ResponsiveLayout, UseWindowBuilder};
    ///
    /// fn App(cx: Scope) -> Element {
    ///     let hook = UseWindowBuilder::default().use_layout::<ResponsiveLayout>(&cx);
    ///
    ///     cx.render(rsx!(
    ///         h1 { "Window layout: {hook}" }
    ///     ))
    /// }
    /// ```
    pub fn use_layout<'a, T>(&self, cx: &'a ScopeState) -> &'a mut UseWindowLayout<T>
    where
        T: 'static,
    {
        let hook = UseWindowLayout::new(self.use_window_hook(cx));
        cx.use_hook(|| hook)
    }
    /// hooks for window's responsive layout with config
    ///
    /// # Arguments
    ///
    /// returns: [`UseWindowLayout`]
    ///
    /// # Examples
    ///
    /// ```
    /// use dioxus::prelude::*;
    /// use dioxus_use_window::{ResponsiveLayout, UseWindowBuilder};
    ///
    /// fn App(cx: Scope) -> Element {
    ///     let hook = UseWindowBuilder::default().use_responsive_layout(&cx);
    ///
    ///     cx.render(rsx!(
    ///         h1 { "Window layout: {hook}" }
    ///     ))
    /// }
    /// ```
    #[inline]
    pub fn use_responsive_layout<'a>(&self, cx: &'a ScopeState) -> &'a mut UseWindowLayout<ResponsiveLayout> {
        self.use_layout(cx)
    }
}
