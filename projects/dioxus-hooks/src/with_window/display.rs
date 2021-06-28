use super::*;

impl<'a> Display for WindowSize<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("").field(&self.width()).field(&self.height()).finish()
    }
}

impl<'a, T> Display for WindowLayout<'a, T>
where
    T: Display,
    T: From<usize>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.get(), f)
    }
}

impl<'a> Display for WindowWidth<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.get(), f)
    }
}

impl<'a> Display for WindowHeight<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.get(), f)
    }
}
