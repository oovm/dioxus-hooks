mod display;
mod iter;

use super::*;

/// effect handler
#[allow(dead_code)]
pub struct UseLocalStorage {
    data: Rc<RefCell<UseStorageData>>,
    listen_storage: Option<EventListener>,
}

impl UseLocalStorage {
    /// builder of `UseCursor`
    pub(crate) fn new(cx: &ScopeState) -> Option<Self> {
        let window = window()?;
        let storage = window.local_storage().ok()??;
        let data = Rc::new(RefCell::new(UseStorageData { storage: Some(storage), last_event: None }));
        let listen_storage = Self::on_storage(cx, &window, &data);
        Some(Self { data, listen_storage: Some(listen_storage) })
    }
    #[inline]
    pub(crate) fn new_ssr(cx: &ScopeState) -> Self {
        #[cfg(debug_assertions)]
        {
            warn!("Window Storage Listener Initializing failed at {}!", cx.scope_id().0);
        }
        Self::default()
    }
    fn on_storage(cx: &ScopeState, window: &Window, data: &Rc<RefCell<UseStorageData>>) -> EventListener {
        #[cfg(debug_assertions)]
        {
            info!("Window Storage Listener Initialized at {}!", cx.scope_id().0);
        }
        let setter = data.clone();
        let regenerate = cx.schedule_update();
        EventListener::new(window, "storage", move |e| {
            let e: StorageEvent = e.clone().unchecked_into();
            let mut setter = setter.borrow_mut();
            if !storage_eq(&setter.storage, &e.storage_area()) {
                return;
            }
            setter.last_event = Some(e);
            regenerate()
        })
    }
}

impl UseLocalStorage {
    /// Getter for the screenX field of this object.
    #[inline]
    pub fn get(&self, key: &str) -> Option<String> {
        self.data.borrow().storage.as_ref()?.get_item(key).ok()?
    }
    ///
    #[inline]
    pub fn get_index(&self, index: usize) -> Option<String> {
        self.data.borrow().storage.as_ref()?.key(index as _).ok()?
    }
    /// Getter for the screenX field of this object.
    #[inline]
    pub fn insert(&self, key: &str, value: &str) -> bool {
        match &self.data.borrow().storage {
            None => false,
            Some(s) => s.set_item(key, value).is_ok(),
        }
    }
    ///
    #[inline]
    pub fn remove(&self, key: &str) -> bool {
        match &self.data.borrow().storage {
            None => false,
            Some(s) => s.remove_item(key).is_ok(),
        }
    }
    ///
    #[inline]
    pub fn len(&self) -> usize {
        match &self.data.borrow().storage {
            None => 0,
            Some(s) => s.length().unwrap_or(0) as _,
        }
    }
    ///
    #[inline]
    pub fn clear(&self) -> bool {
        match &self.data.borrow().storage {
            None => false,
            Some(s) => s.clear().is_ok(),
        }
    }
}
