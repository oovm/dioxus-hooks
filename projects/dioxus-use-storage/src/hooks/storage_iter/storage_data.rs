use super::*;
use std::borrow::Borrow;

impl Default for UseStorageData {
    fn default() -> Self {
        Self {
            storage: None,
            // last_event: None
        }
    }
}

impl UseStorageData {
    #[inline]
    pub fn new(storage: Option<Storage>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(UseStorageData { storage }))
    }
    #[inline]
    pub fn storage(&self) -> Option<Storage> {
        self.storage.borrow().clone()
    }
    #[inline]
    pub fn get(&self, key: &str) -> Option<String> {
        self.storage.as_ref()?.get_item(key).ok()?
    }
    #[inline]
    pub fn get_index(&self, index: usize) -> Option<(String, String)> {
        let key = self.storage.as_ref()?.key(index as _).ok()??;
        let value = self.storage.as_ref()?.get_item(&key).ok()??;
        Some((key, value))
    }
    #[inline]
    pub fn insert(&self, key: &str, value: &str) -> Option<()> {
        self.storage.as_ref()?.set_item(key, value).ok()
    }
    #[inline]
    pub fn remove(&self, key: &str) -> Option<()> {
        self.storage.as_ref()?.remove_item(key).ok()
    }
    #[inline]
    pub fn len(&self) -> Option<usize> {
        Some(self.storage.as_ref()?.length().ok()? as _)
    }
    #[inline]
    pub fn clear(&self) -> Option<()> {
        self.storage.as_ref()?.clear().ok()
    }
}
