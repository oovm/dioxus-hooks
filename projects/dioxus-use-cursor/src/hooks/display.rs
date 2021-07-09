use std::fmt::{Debug, Formatter};
use super::*;

//
// impl Default for UseCursor {
//     fn default() -> Self {
//         Self {
//             data: None,
//             listen_mouse_move: None
//         }
//     }
// }


impl<'a,'b> Debug for UseCursor<'a,'b> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}