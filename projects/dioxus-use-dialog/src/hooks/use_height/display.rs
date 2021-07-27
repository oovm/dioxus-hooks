use super::*;
impl Display for UseWindowHeight {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.get(), f)
    }
}
impl From<UseWindowSize> for UseWindowHeight {
    fn from(size: UseWindowSize) -> Self {
        Self::new(size)
    }
}
