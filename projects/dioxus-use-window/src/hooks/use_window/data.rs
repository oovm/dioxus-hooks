#![allow(dead_code)]
use super::*;

pub struct WindowSizeData {
    window: Option<Window>,
    default_x: f64,
    default_y: f64,
}

impl Default for WindowSizeData {
    fn default() -> Self {
        Self { window: None, default_x: 375.0, default_y: 812.0 }
    }
}

/// FFI for [`WindowSizeData`]
impl WindowSizeData {
    #[inline]
    pub fn get_inner_width(&self) -> Option<f64> {
        self.window.as_ref()?.inner_width().ok()?.as_f64()
    }
    #[inline]
    pub fn set_inner_width(&self, input: usize) -> Option<()> {
        self.window.as_ref()?.set_inner_height(&JsValue::from(input)).ok()
    }
    #[inline]
    pub fn get_inner_height(&self) -> Option<f64> {
        self.window.as_ref()?.inner_height().ok()?.as_f64()
    }
    #[inline]
    pub fn set_inner_height(&self, input: usize) -> Option<()> {
        self.window.as_ref()?.set_inner_height(&JsValue::from(input)).ok()
    }
    #[inline]
    pub fn get_outer_width(&self) -> Option<f64> {
        self.window.as_ref()?.outer_width().ok()?.as_f64()
    }
    #[inline]
    pub fn set_outer_width(&self, input: usize) -> Option<()> {
        self.window.as_ref()?.set_outer_width(&JsValue::from(input)).ok()
    }
    #[inline]
    pub fn get_outer_height(&self) -> Option<f64> {
        self.window.as_ref()?.outer_height().ok()?.as_f64()
    }
    #[inline]
    pub fn set_outer_height(&self, input: usize) -> Option<()> {
        self.window.as_ref()?.set_outer_height(&JsValue::from(input)).ok()
    }
    #[inline]
    pub fn resize_outer_size(&self, x: i32, y: i32) -> Option<()> {
        self.window.as_ref()?.resize_to(x, y).ok()
    }
    #[inline]
    pub fn resize_outer_delta(&self, x: i32, y: i32) -> Option<()> {
        self.window.as_ref()?.resize_by(x, y).ok()
    }
}

impl WindowSizeData {
    #[inline]
    pub fn new(window: Option<Window>, default_size: (f64, f64)) -> Rc<RefCell<Self>> {
        let default = match window.as_ref() {
            None => default_size,
            Some(s) => get_size(s).unwrap_or(default_size),
        };
        Rc::new(RefCell::new(Self { window, default_x: default.0, default_y: default.1 }))
    }
    #[inline]
    pub fn inner_width(&self) -> f64 {
        self.get_inner_width().unwrap_or(self.default_x)
    }
    #[inline]
    pub fn inner_height(&self) -> f64 {
        self.get_inner_height().unwrap_or(self.default_y)
    }
    #[inline]
    pub fn outer_width(&self) -> f64 {
        self.get_inner_width().unwrap_or(self.default_x)
    }
    #[inline]
    pub fn outer_height(&self) -> f64 {
        self.get_inner_height().unwrap_or(self.default_y)
    }
    #[inline]
    pub fn inner_aspect_radio(&self) -> f64 {
        self.inner_width() / self.inner_height()
    }
    #[inline]
    pub fn outer_aspect_radio(&self) -> f64 {
        self.outer_width() / self.outer_height()
    }
}

#[inline]
pub fn get_size(window: &Window) -> Option<(f64, f64)> {
    let x = window.inner_width().ok()?.as_f64()?;
    let y = window.inner_height().ok()?.as_f64()?;
    Some((x, y))
}
