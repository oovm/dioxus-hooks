use super::*;

impl<T> Display for UseWindowLayout<T>
where
    T: Display,
    T: From<usize>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.get(), f)
    }
}

impl<T> From<UseWindowSize> for UseWindowLayout<T> {
    fn from(size: UseWindowSize) -> Self {
        Self::new(size)
    }
}
