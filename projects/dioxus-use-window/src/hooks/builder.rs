use super::*;

/// Build window size hook with config
#[derive(Debug, Copy, Clone)]
pub struct UseWindowBuilder {
    missing_x: usize,
    missing_y: usize,
}

pub(crate) struct WindowSizeData {
    pub(crate) x: f64,
    pub(crate) y: f64,
}

impl Default for UseWindowBuilder {
    fn default() -> Self {
        Self { missing_x: 375, missing_y: 812 }
    }
}

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

impl UseWindowBuilder {
    /// build
    pub fn use_window_size(&self, cx: &ScopeState) -> WindowSize {
        let x = self.missing_x as f64;
        let y = self.missing_y as f64;
        match WindowSize::new(cx, x, y) {
            None => {
                let data = WindowSizeData { x, y };
                WindowSize { data: Rc::new(RefCell::new(data)), listen_window: None }
            }
            Some(s) => s,
        }
    }
}
