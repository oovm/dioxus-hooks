use super::*;

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