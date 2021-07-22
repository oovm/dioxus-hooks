mod display;
use super::*;
impl WindowWidth {
    /// get width of current window
    #[inline]
    pub fn get(&self) -> usize {
        self.inner.width()
    }
    /// set width of current window, return `false` if failed to run
    pub fn set(&self, width: usize) -> bool {
        WindowSize::set_window_width(width).is_some()
    }
    /// get layout of current window, return `false` if failed to run
    #[inline]
    pub fn layout<T>(&self) -> T
    where
        T: From<usize>,
    {
        self.inner.layout()
    }
}
