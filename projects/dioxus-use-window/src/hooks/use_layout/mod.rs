mod display;

use super::*;

/// Window layout effect handler
#[derive(Debug)]
pub struct UseWindowLayout<T> {
    inner: UseWindow,
    bound: PhantomData<T>,
}

impl<T> UseWindowLayout<T> {
    #[inline]
    pub(crate) fn new(size: UseWindow) -> Self {
        Self { inner: size, bound: Default::default() }
    }
}

impl<T> UseWindowLayout<T>
where
    T: From<usize>,
{
    /// get layout of current window
    #[inline]
    pub fn get(&self) -> T {
        self.inner.layout()
    }
}
