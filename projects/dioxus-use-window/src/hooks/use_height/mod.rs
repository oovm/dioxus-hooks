mod display;
use super::*;

impl WindowHeight {
    /// get height of current window
    #[inline]
    pub fn get(&self) -> usize {
        self.inner.height()
    }
    /// set height of current window, return `false` if failed to run
    #[inline]
    pub fn set(&self, height: usize) -> bool {
        WindowSize::set_window_height(height).is_some()
    }
}
