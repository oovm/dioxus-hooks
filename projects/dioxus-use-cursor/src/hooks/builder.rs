use super::*;

///
#[derive(Debug, Copy, Clone)]
pub struct UseCursorBuilder {}

pub(crate) struct UseCursorData {
    pub(crate) mouse: MouseData,
}
#[derive(Debug, Clone)]
pub(crate) struct UseHoverData {
    pub(crate) hover: bool,
}

impl Default for UseCursorData {
    fn default() -> Self {
        Self {
            mouse: MouseData {
                alt_key: false,
                button: 0,
                buttons: 0,
                client_x: 0,
                client_y: 0,
                ctrl_key: false,
                meta_key: false,
                page_x: 0,
                page_y: 0,
                screen_x: 0,
                screen_y: 0,
                shift_key: false,
            },
        }
    }
}

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
