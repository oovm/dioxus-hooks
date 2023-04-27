mod display;
mod iter;

use super::*;

/// Local storage effect handler
pub struct UseLocalStorage {
    data: Rc<RefCell<UseStorageData>>,
    listen_storage: Option<EventListener>,
}

impl UseStorageBuilder {
    /// hooks for window's size with config
    ///
    /// # Arguments
    ///
    /// returns: [`WindowSize`]
    ///
    /// # Examples
    ///
    /// ```
    /// use dioxus::prelude::*;
    /// use dioxus_use_storage::use_local_storage;
    ///
    /// fn App(cx: Scope) -> Element {
    ///     let hook = use_local_storage(&cx);
    ///
    ///     cx.render(rsx!(
    ///         h1 { "Local Storage: {hook}" }
    ///     ))
    /// }
    /// ```
    pub fn use_local_storage<'a>(&self, cx: &'a ScopeState) -> &'a mut UseLocalStorage {
        let hook = UseLocalStorage::new(cx);
        cx.use_hook(|| hook)
    }
}

impl UseLocalStorage {
    fn new(cx: &ScopeState) -> Self {
        match Self::try_new(cx) {
            Some(s) => s,
            None => {
                warn!("Local Storage Listener Initializing failed at {}!", cx.scope_id().0);
                Self::default()
            }
        }
    }
    fn try_new(cx: &ScopeState) -> Option<Self> {
        let window = window()?;
        let storage = window.local_storage().ok()??;
        let data = UseStorageData::new(Some(storage));
        let listen_storage = on_storage(cx, &window, &data);
        Some(Self { data, listen_storage: Some(listen_storage) })
    }
}

impl UseLocalStorage {
    /// get value of given key
    #[inline]
    pub fn get(&self, key: &str) -> Option<String> {
        self.data.borrow().get(key)
    }
    /// get (key, value) at index i
    #[inline]
    pub fn get_index(&self, index: usize) -> Option<(String, String)> {
        self.data.borrow().get_index(index)
    }
    /// insert new value with key, return false if failed to run
    #[inline]
    pub fn insert(&self, key: &str, value: &str) -> bool {
        self.data.borrow().insert(key, value).is_some()
    }
    /// insert value with given key, return false if failed to run
    #[inline]
    pub fn remove(&self, key: &str) -> bool {
        self.data.borrow().remove(key).is_some()
    }
    /// count items in storage
    #[inline]
    pub fn len(&self) -> usize {
        self.data.borrow().len().unwrap_or_default()
    }
    /// clear all value in storage, return false if failed to run
    #[inline]
    pub fn clear(&self) -> bool {
        self.data.borrow().clear().is_some()
    }
}
