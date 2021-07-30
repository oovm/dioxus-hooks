use super::*;
impl Display for UseWindowHeight {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.get(), f)
    }
}
impl From<UseWindow> for UseWindowHeight {
    fn from(size: UseWindow) -> Self {
        Self::new(size)
    }
}
