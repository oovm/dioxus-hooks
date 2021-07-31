use super::*;
impl Display for UseFullScreen {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.toggle(), f)
    }
}
impl From<UseWindow> for UseFullScreen {
    fn from(size: UseWindow) -> Self {
        Self::new(size)
    }
}
