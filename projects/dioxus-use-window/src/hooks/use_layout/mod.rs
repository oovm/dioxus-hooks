mod display;
use super::*;

impl<T> WindowLayout<T>
where
    T: From<usize>,
{
    /// get layout of current window
    pub fn get(&self) -> T {
        self.inner.layout()
        // T::from(self.inner.x as usize)
    }
}
