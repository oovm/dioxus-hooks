mod display;

use super::*;

/// Window layout effect handler
#[derive(Debug)]
pub struct UseWindowLayout<T> {
    pub(crate) inner: UseWindowSize,
    pub(crate) bound: PhantomData<T>,
}

impl<T> UseWindowLayout<T>
    where
        T: From<usize>,
{
    /// get layout of current window
    pub fn get(&self) -> T {
        self.inner.layout()
        // T::from(self.inner.x as usize)
    }
}
