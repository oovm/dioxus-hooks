use super::*;

impl Default for UseCursor {
    fn default() -> Self {
        Self { data: Rc::new(RefCell::new(Default::default())), listen_mouse_move: None }
    }
}

impl Default for UseCursorData {
    fn default() -> Self {
        Self {
            pointer: PointerData {
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
                pointer_id: 0,
                width: 0,
                height: 0,
                pressure: 0.0,
                tangential_pressure: 0.0,
                tilt_x: 0,
                tilt_y: 0,
                twist: 0,
                pointer_type: "".to_string(),
                is_primary: false,
            },
        }
    }
}

impl Debug for UseCursor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UseCursor")
            .field("mouse_data", &self.data.borrow().pointer)
            .field("listen_mouse_move", &self.listen_mouse_move.is_some())
            .finish()
    }
}

impl Display for UseCursor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("").field(&self.screen_x()).field(&self.screen_y()).finish()
    }
}
