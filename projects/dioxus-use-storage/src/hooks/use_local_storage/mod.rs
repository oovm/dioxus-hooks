mod display;

use std::collections::BTreeMap;
use wasm_bindgen::JsValue;
use web_sys::{Storage, StorageEvent, Window};
use super::*;

/// effect handler
pub struct UseLocalStorage {
    data: Rc<RefCell<UseLocalStorageData>>,
    listen_storage: Option<EventListener>,
}

struct UseLocalStorageData {
    storage: Option<Storage>,
    last_event: Option<StorageEvent>,
}

impl UseLocalStorage {
    /// builder of `UseCursor`
    pub(crate) fn new(cx: &ScopeState) -> Option<Self> {
        let window = window()?;
        let storage = window.local_storage().ok()??;
        let data = Rc::new(RefCell::new(UseLocalStorageData {
            storage: Some(storage),
            last_event: None,
        }));
        let listen_storage = Self::on_storage(cx, &window, &data);
        Some(Self { data, listen_storage: Some(listen_storage) })
    }
    #[inline]
    pub(crate) fn new_ssr(_: &ScopeState) -> Self {
        Self::default()
    }
    fn on_storage(cx: &ScopeState, window: &Window, data: &Rc<RefCell<UseLocalStorageData>>) -> EventListener {
        #[cfg(debug_assertions)]
            {
                info!("Window Storage Listener Initialized at {}!", cx.scope_id().0);
            }
        let setter = data.clone();
        let regenerate = cx.schedule_update();
        EventListener::new(window, "storage", move |e| {
            let e: StorageEvent = e.clone().unchecked_into();
            if e.storage_area() {
                let mut setter = setter.borrow_mut();
                setter.last_event = Some(e);
                regenerate()
            }
        })
    }
}


impl UseLocalStorage {
    /// Getter for the screenX field of this object.
    pub fn get(&self, key: &str) -> Option<String> {
        self.data.borrow().storage.get_item(key).ok()?
    }
    ///
    pub fn get_index(&self, index: usize) -> Option<String> {
        self.data.borrow().storage.key(index as _).ok()?
    }
    /// Getter for the screenX field of this object.
    pub fn insert(&self, key: &str, value: &str) -> bool {
        self.data.borrow().storage.set_item(key, value).is_ok()
    }
    ///
    pub fn remove(&self, key: &str) -> bool {
        self.data.borrow().storage.remove_item(key).is_ok()
    }
    ///
    pub fn clear(&self) -> bool {
        self.data.borrow().storage.clear().is_ok()
    }
    pub fn iter(&self) -> StorageIter {
        StorageIter {
            inner: &self.data.borrow().storage,
            index: 0,
            value: self.get_index(0).unwrap_or_default(),
        }
    }
}


