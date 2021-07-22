use super::*;

impl<T> Display for WindowLayout<T>
where
    T: Display,
    T: From<usize>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.get(), f)
    }
}
