mod display;
use super::*;

/// Window width effect handler
#[derive(Debug)]
pub struct UseWindowWidth {
    inner: UseWindow,
}

impl UseWindowWidth {
    pub(crate) fn new(size: UseWindow) -> Self {
        Self { inner: size }
    }
}

impl UseWindowWidth {
    /// get width of current window
    #[inline]
    pub fn get(&self) -> usize {
        self.inner.data_ref().inner_width() as _
    }
    /// set width of current window, return `false` if failed to run
    #[inline]
    pub fn set(&self, width: usize) -> bool {
        self.inner.data_ref().set_inner_width(width).is_some()
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
