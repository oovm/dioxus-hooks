use super::*;
use std::fmt::{Display, Formatter};

impl Default for UseHoveringData {
    fn default() -> Self {
        Self { hover: false }
    }
}

impl Display for UseHovering {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.get(), f)
    }
}
