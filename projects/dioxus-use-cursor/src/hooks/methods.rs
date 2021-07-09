use std::cell::RefCell;
use std::mem::{MaybeUninit};
use std::ops::Deref;
use dioxus::events::MouseData;
use dioxus::hooks::use_state;
use web_sys::Event;
use super::*;


impl<'a> UseCursor<'a> {
    ///
    pub fn new(cx: &ScopeState) -> Option<Self> {
        let window = window()?;

        Rc::new(RefCell::new(None));

        let mut data = None;
        let regenerate= cx.schedule_update();
        let mouse_move = EventListener::new(&window, "mousemove", move |e| {
            let e: &MouseEvent = e.unchecked_ref();
            data = Some(e.clone());
            regenerate();
        });
        Some(Self {
            data: &mut data,
            listen_mouse_move: mouse_move
        })
    }
    // fn on_mouse_move(&mut self, e: &Event) {
    //     let e: &MouseEvent = e.unchecked_ref();
    //     self.data = Some(e.clone());
    //     (self.update)()
    // }
}

impl<'a> UseCursor<'a> {
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