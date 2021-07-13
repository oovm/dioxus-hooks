use std::fmt::Debug;
use super::*;

impl Debug for WindowSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WindowSize")
            .field("width", &self.width())
            .field("height", &self.height())
            .field("aspect_radio", &self.aspect_radio())
            .field("listen_window", &self.listen_window.is_some())
            .finish()
    }
}

impl Display for WindowSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("").field(&self.width()).field(&self.height()).finish()
    }
}

impl<T> Display for WindowLayout<T>
    where
        T: Display,
        T: From<usize>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.get(), f)
    }
}

impl Display for WindowWidth {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.get(), f)
    }
}

impl Display for WindowHeight {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.get(), f)
    }
}
