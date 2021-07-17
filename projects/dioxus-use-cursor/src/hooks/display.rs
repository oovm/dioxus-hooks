use super::*;
use std::fmt::{Debug, Display, Formatter};

// impl Default for UseCursor {
//     fn default() -> Self {
//         Self {
//             data: None,
//             listen_mouse_move: None
//         }
//     }
// }

impl Debug for UseCursor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UseCursor")
            .field("mouse_data", &self.data.borrow().mouse)
            .field("listen_mouse_move", &self.listen_mouse_move.is_some())
            .finish()
    }
}


impl Display for UseCursor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("")
            .field(&self.screen_x())
            .field(&self.screen_y())
            .finish()
    }
}