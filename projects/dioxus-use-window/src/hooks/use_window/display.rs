use super::*;

impl Default for WindowSizeData {
    fn default() -> Self {
        // used for ssr
        Self { x: MISSING_W, y: MISSING_H }
    }
}

impl Default for WindowSize {
    fn default() -> Self {
        Self { data: Rc::new(RefCell::new(Default::default())), listen_window: None }
    }
}

impl Debug for WindowSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WindowSize")
            .field("width", &self.width())
            .field("height", &self.height())
            .field("aspect_radio", &self.aspect_radio())
            .field("listen_window", &self.listen_window.is_some())
            .finish()
    }
}

impl Display for WindowSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("").field(&self.width()).field(&self.height()).finish()
    }
}
