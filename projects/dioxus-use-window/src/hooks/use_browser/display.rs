use super::*;
impl Display for UseBrowser {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.width(), f)
    }
}
impl From<UseWindow> for UseBrowser {
    fn from(size: UseWindow) -> Self {
        Self::new(size)
    }
}
