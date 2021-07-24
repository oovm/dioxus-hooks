use super::*;
impl Display for UseWindowWidth {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.get(), f)
    }
}

impl From<UseWindowSize> for UseWindowWidth {
    fn from(size: UseWindowSize) -> Self {
        Self::new(size)
    }
}
