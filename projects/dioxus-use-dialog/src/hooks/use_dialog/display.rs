use super::*;

impl Debug for UseDialog {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UseDialog").field("has_window", &self.window.is_some()).finish()
    }
}
