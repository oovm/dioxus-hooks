use super::*;

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
    pub fn use_size<'a>(&self, cx: &'a ScopeState) -> &'a mut UseWindowSize {
        let hook = self.hook_window_size(cx);
        cx.use_hook(|_| hook)
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
    pub fn use_width<'a>(&self, cx: &'a ScopeState) -> &'a mut UseWindowWidth {
        let hook = UseWindowWidth { inner: self.hook_window_size(cx) };
        cx.use_hook(|_| hook)
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
    pub fn use_height<'a>(&self, cx: &'a ScopeState) -> &'a mut UseWindowHeight {
        let hook = UseWindowHeight { inner: self.hook_window_size(cx) };
        cx.use_hook(|_| hook)
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
    pub fn use_layout<'a, T>(&self, cx: &'a ScopeState) -> &'a mut UseWindowLayout<T>
    where
        T: 'static,
    {
        let hook = UseWindowLayout { inner: self.hook_window_size(cx), bound: Default::default() };
        cx.use_hook(|_| hook)
    }
}

impl UseWindowBuilder {
    fn hook_window_size(&self, cx: &ScopeState) -> UseWindowSize {
        match UseWindowSize::new(cx) {
            None => {
                let data = WindowSizeData::from(self.clone());
                UseWindowSize::new_ssr(cx, data)
            }
            Some(s) => s,
        }
    }
}
