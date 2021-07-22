use super::*;

///
#[derive(Debug, Copy, Clone)]
pub struct UseCursorBuilder {}

impl Default for UseCursorBuilder {
    fn default() -> Self {
        Self {}
    }
}

impl Default for UseCursor {
    fn default() -> Self {
        Self { data: Rc::new(RefCell::new(Default::default())), listen_mouse_move: None }
    }
}

impl UseCursorBuilder {
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
    pub fn use_cursor<'a, 'b>(&'a self, cx: &'b ScopeState) -> &'b mut UseCursor {
        let hook = UseCursor::new(cx).unwrap_or_default();
        cx.use_hook(|_| hook)
    }
}
