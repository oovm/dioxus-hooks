use super::*;

impl Default for UseSessionStorage {
    fn default() -> Self {
        Self { data: Rc::new(RefCell::new(Default::default())), listen_storage: None }
    }
}

impl Default for UseLocalSessionData {
    fn default() -> Self {
        Self { storage: None, last_event: None }
    }
}

impl Debug for UseSessionStorage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("UseSessionStorage ")?;
        f.debug_map().entries(self.iter()).finish()
    }
}

impl Display for UseSessionStorage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UseSessionStorage").field("items", &self.len()).finish()
    }
}
