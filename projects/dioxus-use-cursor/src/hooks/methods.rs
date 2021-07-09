use std::cell::RefCell;
use std::mem;
use std::mem::{MaybeUninit, zeroed};
use std::ops::Deref;
use dioxus::events::MouseData;
use dioxus::hooks::use_state;
use web_sys::Event;
use super::*;


impl<'a, 'b> UseCursor<'a, 'b> {
    ///
    pub fn new(cx: &'a ScopeState) -> Option<Self> {
        let window = window()?;
        let mut hook = unsafe {
            Self {
                scope: cx,
                data: None,
                listen_mouse_move: zeroed(),
            }
        };
        let regenerate = cx.schedule_update();
        let mouse_move = EventListener::new(&window, "mousemove", move |e| {
            let e: &MouseEvent = e.unchecked_ref();
            hook.data = Some(e);
            regenerate();
        });
        hook.listen_mouse_move = mouse_move;
        Some(hook)
    }
    // fn on_mouse_move(&mut self, e: &Event) {
    //     let e: &MouseEvent = e.unchecked_ref();
    //     self.data = Some(e.clone());
    //     (self.update)()
    // }
}

// impl UseCursor {
//     /// Getter for the screenX field of this object.
//     pub fn screen_x(&self) -> usize {
//         self.data.as_ref().map(|e| e.screen_x()).unwrap_or_default() as _
//     }
//     ///
//     pub fn screen_y(&self) -> usize {
//         self.data.as_ref().map(|e| e.screen_x()).unwrap_or_default() as _
//     }
//     ///
//     pub fn element_width(&self) -> usize {
//         self.data.as_ref().map(|e| e.screen_x()).unwrap_or_default() as _
//     }
//     ///
//     pub fn element_height(&self) -> usize {
//         self.data.as_ref().map(|e| e.screen_x()).unwrap_or_default() as _
//     }
//     ///
//     pub fn is_over(&self) {}
//     ///
//     pub fn is_down(&self) {}
// }

/// https://www.npmjs.com/package/@react-hook/mouse-position
impl UseHover {
    // pub fn new() {
    //     let mut hook = UseCursor {
    //         last_event: UseCursor {}
    //     };
    //
    //     let mouseover = EventListener::new(&window, "mouseover", move |_| {
    //         hook.over = true;
    //     });
    //     let mouseout = EventListener::new(&window, "mouseout", move |_| {
    //         hook.over = false;
    //     });
    // }
}

impl UseHover {
    ///
    #[inline]
    pub fn get(&self) {
        // self.over.is_over()
    }
}