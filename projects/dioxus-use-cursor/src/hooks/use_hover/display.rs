use super::*;

impl Default for UseHover {
    fn default() -> Self {
        Self {
            data: Rc::new(RefCell::new(Default::default())),
            listen_mouse_over: None,
            listen_mouse_out: None
        }
    }
}

impl Default for UseHoverData {
    fn default() -> Self {
        Self { hover: false }
    }
}

impl Display for UseHover {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.get(), f)
    }
}

impl Debug for UseHover {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WindowSize")
            .field("is_hover", &self.get())
            .field("listen_mouse_over", &self.listen_mouse_over.is_some())
            .field("listen_mouse_out", &self.listen_mouse_out.is_some())
            .finish()
    }
}
