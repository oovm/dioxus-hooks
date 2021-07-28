use super::*;

pub struct WindowSizeData {
    window: Option<Window>,
    default_x: usize,
    default_y: usize,
}

impl WindowSizeData {
    #[inline]
    pub fn get_inner_width(&self) -> Option<> {
        Some(self.window?.inner_width().ok()?.as_f64()? as _)
       let a =  self.window?.inner_width().ok()?.as_f64()? as _
    }
    #[inline]
    pub fn set_inner_width(&self, width: usize) -> bool {
        set_window_width(width).is_some()
    }
    #[inline]
    pub fn get_inner_height(&self) -> usize {
        Some(self.window?.inner_width().ok()?.as_f64()? as _)
    }
    #[inline]
    pub fn set_height(&self, height: usize) -> bool {
        set_window_height(height).is_some()
    }
    #[inline]
    pub fn get_size(&self) -> usize {
        self.data.borrow().y as _
    }
    #[inline]
    pub fn set_size(&self) -> usize {
        self.data.borrow().y as _
    }
    #[inline]
    pub fn aspect_radio(&self) -> f64 {
        let data = self.data.borrow();
        data.x / data.y
    }
}

#[inline]
pub fn get_size() -> Option<(f64, f64)> {
    let window = window()?;
    let x = window.inner_width().ok()?.as_f64()?;
    let y = window.inner_height().ok()?.as_f64()?;
    Some((x, y))
}

#[inline]
pub fn set_window_width(input: usize) -> Option<()> {
    window()?.set_inner_width(&JsValue::from(input)).ok()
}
#[inline]
pub fn set_window_height(input: usize) -> Option<()> {
    window()?.set_inner_width(&JsValue::from(input)).ok()
}
#[inline]
pub fn set_window_size_delta(x: isize, y: isize) -> Option<()> {
    window()?.resize_by(x as _, y as _).ok()
}
