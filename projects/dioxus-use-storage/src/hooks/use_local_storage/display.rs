use super::*;
use std::fmt::{Debug, Display, Formatter};

impl Default for UseLocalStorage {
    fn default() -> Self {
        Self { data: Rc::new(RefCell::new(Default::default())), listen_storage: None }
    }
}

impl Default for UseLocalStorageData {
    fn default() -> Self {
        Self {
            storage: None,
            last_event: None
        }
    }
}

impl Debug for UseLocalStorage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UseCursor")
            .field("mouse_data", &self.data.borrow().mouse)
            .field("listen_mouse_move", &self.listen_storage.is_some())
            .finish()
    }
}

impl Display for UseLocalStorage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("").field(&self.get()).field(&self.get_index()).finish()
    }
}
