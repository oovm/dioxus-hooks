use super::*;

impl Default for UseWindow {
    fn default() -> Self {
        Self { data: Rc::new(RefCell::new(Default::default())), listen_window: None }
    }
}

impl Debug for UseWindow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let size = self.size();
        f.debug_struct("WindowSize")
            .field("width", &size.0)
            .field("height", &size.1)
            .field("aspect_radio", &self.aspect_radio())
            .field("listen_window", &self.listen_window.is_some())
            .finish()
    }
}

impl Display for UseWindow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.size(), f)
    }
}
