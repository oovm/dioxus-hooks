use super::*;

impl Default for WindowSizeData {
    fn default() -> Self {
        // used for ssr
        const MISSING_W: f64 = 375.0;
        const MISSING_H: f64 = 812.0;
        Self { x: MISSING_W, y: MISSING_H }
    }
}

impl Default for UseWindowSize {
    fn default() -> Self {
        Self { data: Rc::new(RefCell::new(Default::default())), listen_window: None }
    }
}

impl Debug for UseWindowSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WindowSize")
            .field("width", &self.get_width())
            .field("height", &self.get_height())
            .field("aspect_radio", &self.aspect_radio())
            .field("listen_window", &self.listen_window.is_some())
            .finish()
    }
}

impl Display for UseWindowSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("").field(&self.get_width()).field(&self.get_height()).finish()
    }
}

impl From<UseWindowBuilder> for WindowSizeData {
    fn from(builder: UseWindowBuilder) -> Self {
        let x = builder.missing_x as f64;
        let y = builder.missing_y as f64;

        Self { x, y }
    }
}
