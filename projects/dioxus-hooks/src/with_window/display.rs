use std::fmt::{Display, Formatter};
use crate::WindowSize;

impl<'a> Display for WindowSize<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("").field(&self.width()).field(&self.height()).finish()
    }
}