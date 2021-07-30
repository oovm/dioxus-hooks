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

impl<T> From<UseWindow> for UseWindowLayout<T> {
    fn from(size: UseWindow) -> Self {
        Self::new(size)
    }
}
