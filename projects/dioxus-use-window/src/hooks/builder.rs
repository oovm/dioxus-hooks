use super::*;

/// Build window size hook with config
#[derive(Debug, Copy, Clone)]
pub struct WindowSizeBuilder {}

pub(crate) struct WindowSizeData {
    pub(crate) x: f64,
    pub(crate) y: f64,
}

impl Default for WindowSizeData {
    fn default() -> Self {
        // used for ssr
        Self { x: MISSING_W, y: MISSING_H }
    }
}

impl Default for WindowSize {
    fn default() -> Self {
        Self {
            data: Rc::new(RefCell::new(Default::default())),
            listen_window: None
        }
    }
}