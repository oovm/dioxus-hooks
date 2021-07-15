use super::*;
use std::{
    cell::RefCell,
    mem,
    mem::{zeroed, MaybeUninit},
    ops::Deref,
};
use web_sys::Event;

impl UseCursor {
    ///
    pub(crate) fn new(cx: &ScopeState) -> Option<Self> {
        let window = window()?;
        let data = Rc::new(RefCell::new(UseCursorData { mouse: None }));
        let setter = data.clone();
        let regenerate = cx.schedule_update();
        let mouse_move = EventListener::new(&window, "mousemove", move |e| {
            let mut setter = setter.borrow_mut();
            let e: MouseEvent = e.clone().unchecked_into();
            setter.mouse = Some(e);
            regenerate();
        });
        Some(Self { data, listen_mouse_move: mouse_move })
    }
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
