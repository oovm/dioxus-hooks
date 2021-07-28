mod display;
mod iter;

use super::*;

/// effect handler
pub struct UseSessionStorage {
    data: Rc<RefCell<UseStorageData>>,
    listen_storage: Option<EventListener>,
}

impl UseStorageBuilder {
    /// hooks for window's size with config
    ///
    /// # Arguments
    ///
    /// * `cx`: [`Scope`] or [`ScopeState`]
    ///
    /// returns: [`WindowSize`]
    ///
    /// # Examples
    ///
    /// ```
    /// use dioxus::prelude::*;
    /// use dioxus_use_storage::{use_session_storage};
    ///
    /// fn App(cx: Scope) -> Element {
    ///     let hook = use_session_storage(&cx);
    ///
    ///     cx.render(rsx!(
    ///         h1 { "Session storage: {hook}" }
    ///     ))
    /// }
    /// ```
    pub fn use_session_storage<'a>(&self, cx: &'a ScopeState) -> &'a mut UseSessionStorage {
        let hook = UseSessionStorage::new(cx);
        cx.use_hook(|_| hook)
    }
}

impl UseSessionStorage {
    fn new(cx: &ScopeState) -> Self {
        match Self::try_new(cx) {
            Some(s) => s,
            None => {
                warn!("Window Storage Listener Initializing failed at {}!", cx.scope_id().0);
                Self::default()
            }
        }
    }
    fn try_new(cx: &ScopeState) -> Option<Self> {
        let window = window()?;
        let storage = window.session_storage().ok()??;
        let data = UseStorageData::new(Some(storage));
        let listen_storage = on_storage(cx, &window, &data);
        Some(Self { data, listen_storage: Some(listen_storage) })
    }
}

impl UseSessionStorage {
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
