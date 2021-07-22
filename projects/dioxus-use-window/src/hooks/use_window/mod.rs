use super::*;
mod display;

/// Window size effect handler
pub struct WindowSize {
    data: Rc<RefCell<WindowSizeData>>,
    listen_window: Option<EventListener>,
}

struct WindowSizeData {
    x: f64,
    y: f64,
}