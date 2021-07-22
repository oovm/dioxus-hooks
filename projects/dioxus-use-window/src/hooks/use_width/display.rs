use super::*;
impl Display for UseWindowWidth {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.get(), f)
    }
}
