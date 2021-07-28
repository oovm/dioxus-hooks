mod display;
use super::*;

/// Window width effect handler
#[derive(Debug)]
pub struct UseWindowWidth {
    inner: UseWindowSize,
}

impl UseWindowWidth {
    pub(crate) fn new(size: UseWindowSize) -> Self {
        Self { inner: size }
    }
}

impl UseWindowWidth {
    /// get width of current window
    #[inline]
    pub fn get(&self) -> usize {
        self.inner.get_inner_width()
    }
    /// set width of current window, return `false` if failed to run
    #[inline]
    pub fn set(&self, width: usize) -> bool {
        self.inner.set_inner_width(width)
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
