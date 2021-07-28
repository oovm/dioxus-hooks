mod display;
use super::*;

/// Window height effect handler
#[derive(Debug)]
pub struct UseWindowHeight {
    inner: UseWindowSize,
}

impl UseWindowBuilder {
    /// hooks for window's height with config
    ///
    /// # Arguments
    ///
    /// returns: [`UseWindowHeight`]
    ///
    /// # Examples
    ///
    /// ```
    /// use dioxus::prelude::*;
    /// use dioxus_use_window::{UseWindowBuilder};
    ///
    /// fn App(cx: Scope) -> Element {
    ///     let hook = UseWindowBuilder::default().use_height(&cx);
    ///
    ///     cx.render(rsx!(
    ///         h1 { "Window height: {hook}" }
    ///     ))
    /// }
    /// ```
    pub fn use_height<'a>(&self, cx: &'a ScopeState) -> &'a mut UseWindowHeight {
        let hook = UseWindowHeight::new(self.hook_window_size(cx));
        cx.use_hook(|_| hook)
    }
}

impl UseWindowHeight {
    fn new(size: UseWindowSize) -> Self {
        Self { inner: size }
    }
}

impl UseWindowHeight {
    /// get height of current window
    #[inline]
    pub fn get(&self) -> usize {
        self.inner.get_height()
    }
    /// set height of current window, return `false` if failed to run
    #[inline]
    pub fn set(&self, height: usize) -> bool {
        self.inner.set_height(height)
    }
}
