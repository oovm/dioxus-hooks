mod display;
mod iter;

use super::*;

/// effect handler
#[allow(dead_code)]
pub struct UseSessionStorage {
    data: Rc<RefCell<UseStorageData>>,
    listen_storage: Option<EventListener>,
}

impl UseSessionStorage {
    pub(crate) fn new(cx: &ScopeState) -> Self {
        match Self::try_new(cx) {
            Some(s) => s,
            None => {
                warn!("Window Storage Listener Initializing failed at {}!", cx.scope_id().0);
                Self::default()
            }
        }
    }
    pub fn try_new(cx: &ScopeState) -> Option<Self> {
        let window = window()?;
        let storage = window.session_storage().ok()??;
        let data = Rc::new(RefCell::new(UseStorageData::new(Some(storage))));
        let listen_storage = on_storage(cx, &window, &data);
        Some(Self { data, listen_storage: Some(listen_storage) })
    }
}

impl UseSessionStorage {
    /// Getter for the screenX field of this object.
    #[inline]
    pub fn get(&self, key: &str) -> Option<String> {
        self.data.borrow().get(key)
    }
    ///
    #[inline]
    pub fn get_index(&self, index: usize) -> Option<String> {
        self.data.borrow().get_index(index)
    }
    /// Getter for the screenX field of this object.
    #[inline]
    pub fn insert(&self, key: &str, value: &str) -> bool {
        self.data.borrow().insert(key, value).is_some()
    }
    ///
    #[inline]
    pub fn remove(&self, key: &str) -> bool {
        self.data.borrow().remove(key).is_some()
    }
    ///
    #[inline]
    pub fn len(&self) -> usize {
        self.data.borrow().len().unwrap_or_default()
    }
    ///
    #[inline]
    pub fn clear(&self) -> bool {
        self.data.borrow().clear().is_some()
    }
}
