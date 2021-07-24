mod display;

use std::collections::BTreeMap;
use wasm_bindgen::JsValue;
use web_sys::{Storage, StorageEvent};
use super::*;

/// effect handler
pub struct UseLocalStorage {
    data: Rc<RefCell<UseLocalStorageData>>,
    listen_mouse_move: Option<EventListener>,
}

struct UseLocalStorageData {
    storage: Storage,
    last_event: Option<StorageEvent>,
}

impl UseLocalStorage {
    /// builder of `UseCursor`
    pub(crate) fn new(cx: &ScopeState) -> Option<Self> {
        let storage = window()?.local_storage().ok()??;

        storage.key();

        storage.get_item()

        let data = Rc::new(RefCell::new(UseLocalStorageData::default()));
        let setter = data.clone();
        let regenerate = cx.schedule_update();
        let mouse_move = EventListener::new(&storage, "mousemove", move |e| {
            let mut setter = setter.borrow_mut();
            setter.mouse = Self::event_as_mouse_data(e);
            regenerate();
        });
        Some(Self { data, listen_mouse_move: Some(mouse_move) })
    }
    fn event_as_mouse_data(event: &Event) -> MouseData {
        let e: &MouseEvent = event.unchecked_ref();
        MouseData {
            alt_key: e.alt_key(),
            button: e.button(),
            buttons: e.buttons(),
            client_x: e.client_x(),
            client_y: e.client_y(),
            ctrl_key: e.ctrl_key(),
            meta_key: e.meta_key(),
            page_x: e.page_x(),
            page_y: e.page_y(),
            screen_x: e.screen_x(),
            screen_y: e.screen_y(),
            shift_key: e.shift_key(),
        }
    }
    fn on_storage(cx: &ScopeState, target: &EventTarget, data: &Rc<RefCell<UseHoverData>>) -> EventListener {
        #[cfg(debug_assertions)]
            {
                info!("Window Storage Listener Initialized at {}!", cx.scope_id().0);
            }
        let setter = data.clone();
        let regenerate = cx.schedule_update();
        EventListener::new(target, "storage", move |e| {
            let mut setter = setter.borrow_mut();
            e.unchecked_into()

            setter.hover = true;
            regenerate()
        })
    }
    fn as_storage_event(&e) {

    }
}


impl UseLocalStorage {
    /// Getter for the screenX field of this object.
    pub fn get(&self, key: &str) -> Option<String> {
        self.data.borrow().storage.get_item(key).ok()?
    }
    /// Getter for the screenX field of this object.
    pub fn insert(&self, key: &str, value:&str) -> bool {
        self.data.borrow().storage.set_item(key, value).is_ok()
    }
    ///
    pub fn get_index(&self, index: usize) -> Option<String> {
        self.data.borrow().storage.key(index as _).ok()?
    }
    ///
    pub fn remove(&self, key: &str) {
        let map = BTreeMap::default();
        self.data.borrow().storage.remove_item(index as _).ok()?
    }
    ///
    pub fn clear(&self) {}

    // #[inline]
    // fn view_mouse(&self) -> Option<&MouseEvent> {
    //     self.data.borrow().mouse.as_ref()
    // }
}
