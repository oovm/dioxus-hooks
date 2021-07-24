use super::*;
use std::fmt::{Debug, Display, Formatter};

impl Default for UseLocalStorage {
    fn default() -> Self {
        Self { data: Rc::new(RefCell::new(Default::default())), listen_mouse_move: None }
    }
}

impl Default for UseLocalStorageData {
    fn default() -> Self {
        Self {
            mouse: MouseData {
                alt_key: false,
                button: 0,
                buttons: 0,
                client_x: 0,
                client_y: 0,
                ctrl_key: false,
                meta_key: false,
                page_x: 0,
                page_y: 0,
                screen_x: 0,
                screen_y: 0,
                shift_key: false,
            },
        }
    }
}

impl Debug for UseLocalStorage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UseCursor")
            .field("mouse_data", &self.data.borrow().mouse)
            .field("listen_mouse_move", &self.listen_mouse_move.is_some())
            .finish()
    }
}

impl Display for UseLocalStorage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("").field(&self.get()).field(&self.get_index()).finish()
    }
}
