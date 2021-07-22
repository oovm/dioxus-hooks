use super::*;

/// Build window size hook with config
#[derive(Debug, Copy, Clone)]
pub struct UseWindowBuilder {
    pub missing_x: usize,
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
    pub fn use_window_size<'a>(& self, cx: &'a ScopeState) -> &'a mut WindowSize {
        let hook = match WindowSize::new(cx) {
            None => {
                let data = WindowSizeData::from(self.clone());
                WindowSize::new_ssr(cx, data)
            }
            Some(s) => s,
        };
        cx.use_hook(|_| hook)
    }

    pub fn use_width<'a>(& self, cx: &'a ScopeState) -> &'a mut WindowSize {
        let hook = match WindowSize::new(cx) {
            None => {
                let data = WindowSizeData::from(self.clone());
                WindowSize::new_ssr(cx, data)
            }
            Some(s) => s,
        };
        cx.use_hook(|_| hook)
    }
    pub fn use_height<'a>(& self, cx: &'a ScopeState) -> &'a mut WindowSize {
        let hook = match WindowSize::new(cx) {
            None => {
                let data = WindowSizeData::from(self.clone());
                WindowSize::new_ssr(cx, data)
            }
            Some(s) => s,
        };
        cx.use_hook(|_| hook)
    }
    pub fn use_layout<'a>(& self, cx: &'a ScopeState) -> &'a mut WindowSize {
        let hook = match WindowSize::new(cx) {
            None => {
                let data = WindowSizeData::from(self.clone());
                WindowSize::new_ssr(cx, data)
            }
            Some(s) => s,
        };
        cx.use_hook(|_| hook)
    }
}
