use super::*;

impl Default for UseLocalStorage {
    fn default() -> Self {
        Self { data: Rc::new(RefCell::new(Default::default())), listen_storage: None }
    }
}

impl Default for UseLocalStorageData {
    fn default() -> Self {
        Self { storage: None, last_event: None }
    }
}

impl Debug for UseLocalStorage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("UseLocalStorage ")?;
        f.debug_map().entries(self.iter()).finish()
    }
}

impl Display for UseLocalStorage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UseLocalStorage").field("items", &self.len()).finish()
    }
}
