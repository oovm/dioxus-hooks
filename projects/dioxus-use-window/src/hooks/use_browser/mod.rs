mod display;
use super::*;

/// Window height effect handler
#[derive(Debug)]
pub struct UseBrowser {
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
    ///     let hook = UseWindowBuilder::default().use_browser(&cx);
    ///
    ///     cx.render(rsx!(
    ///         h1 { "Window height: {hook}" }
    ///     ))
    /// }
    /// ```
    #[inline]
    pub fn use_browser<'a>(&self, cx: &'a ScopeState) -> &'a mut UseBrowser {
        let hook = UseBrowser::new(self.use_window_hook(cx));
        cx.use_hook(|_| hook)
    }
}

impl UseBrowser {
    #[inline]
    pub(crate) fn new(size: UseWindow) -> Self {
        Self { inner: size }
    }
    /// get size of current browser
    #[inline]
    pub fn size(&self) -> (usize, usize) {
        (self.width(), self.height())
    }
    /// get width of current browser
    ///
    /// **read-only**
    #[inline]
    pub fn width(&self) -> usize {
        self.inner.data_ref().outer_width() as usize
    }
    /// get height of current window
    ///
    /// **read-only**
    #[inline]
    pub fn height(&self) -> usize {
        self.inner.data_ref().outer_height() as usize
    }
    /// get aspect radio of current window
    #[inline]
    pub fn aspect_radio(&self) -> f64 {
        self.inner.data_ref().outer_aspect_radio()
    }
    /// get height of current window
    #[inline]
    pub fn resize_to(&self, size: (usize, usize)) -> bool {
        self.inner.data_ref().resize_outer_size(size.0 as _, size.1 as _).is_some()
    }
    /// get height of current window
    #[inline]
    pub fn resize_delta(&self, size: (i32, i32)) -> bool {
        self.inner.data_ref().resize_outer_delta(size.0, size.1).is_some()
    }
}

impl UseBrowser {
    #[inline]
    pub fn is_fullscreen(&self) -> (usize, usize) {
        (self.toggle(), self.height())
    }
    /// get width of current browser
    #[inline]
    pub fn fullscreen_toggle(&self) -> usize {
        self.inner.data_ref().fullscreen_toggle()
    }
    #[inline]
    pub fn fullscreen_on(&self) -> usize {
        self.inner.data_ref().outer_width() as usize
    }
    #[inline]
    pub fn fullscreen_off(&self) -> usize {
        self.inner.data_ref().outer_width() as usize
    }
}