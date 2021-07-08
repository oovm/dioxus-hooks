use std::mem::{MaybeUninit, uninitialized};
use std::ops::Deref;
use dioxus::events::MouseData;
use web_sys::Event;
use super::*;


impl UseCursor {
    ///
    pub fn new(cx: &ScopeState) -> Option<Self> {
        let window = window()?;
        let mut hook = UseCursor {
            data: None,
            update: cx.schedule_update(),
            listen_mouse_move: unsafe { MaybeUninit::uninit() },
        };
        let mouse_move = EventListener::new(&window, "mousemove", move |e| hook.on_mouse_move(e));
        hook.listen_mouse_move.write(mouse_move);
        unsafe {
            hook.listen_mouse_move.assume_init();
        }
        Some(hook)
    }
    fn on_mouse_move(&mut self, e: &Event) {
        let e: &MouseEvent = e.unchecked_ref();
        self.data = Some(e.clone());
        (self.update)()
    }
}

impl UseCursor {
    /// Getter for the screenX field of this object.
    pub fn screen_x(&self) -> usize {
        match &self.data {
            None => { Default::default() }
            Some(e) => {
                e.screen_x() as _
            }
        }
    }
    ///
    pub fn screen_y(&self) -> usize {
        match &self.data {
            None => { Default::default() }
            Some(e) => {
                e.screen_y() as _
            }
        }
    }
    ///
    pub fn element_width(&self) -> usize {
        match &self.data {
            None => { Default::default() }
            Some(e) => {
                e.screen_x() as _
            }
        }
    }
    ///
    pub fn element_height(&self) -> usize {
        match &self.data {
            None => { Default::default() }
            Some(e) => {
                e.screen_x() as _
            }
        }
    }
    ///
    pub fn is_over(&self) {}
    ///
    pub fn is_down(&self) {}
}

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