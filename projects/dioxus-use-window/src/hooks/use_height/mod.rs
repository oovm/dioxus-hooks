mod display;
use super::*;

/// Window height effect handler
#[derive(Debug)]
pub struct UseWindowHeight {
    inner: UseWindow,
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
    #[inline]
    pub fn use_height<'a>(&self, cx: &'a ScopeState) -> &'a mut UseWindowHeight {
        let hook = UseWindowHeight::new(self.use_window_hook(cx));
        cx.use_hook(|_| hook)
    }
}

impl UseWindowHeight {
    #[inline]
    pub(crate) fn new(size: UseWindow) -> Self {
        Self { inner: size }
    }
}

impl UseWindowHeight {
    /// get height of current window
    #[inline]
    pub fn get(&self) -> usize {
        self.inner.data().inner_height() as _
    }
    /// set height of current window, return `false` if failed to run
    #[inline]
    pub fn set(&self, height: usize) -> bool {
        self.inner.data().set_inner_height(height).is_some()
    }
}
