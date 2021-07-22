use super::*;
use std::fmt::{Display, Formatter};

impl Default for UseHoverData {
    fn default() -> Self {
        Self { hover: false }
    }
}

impl Display for UseHover {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.get(), f)
    }
}
