mod display;

use super::*;
use dioxus::events::PointerData;
use web_sys::{Event, PointerEvent};

/// effect handler
pub struct UseCursor {
    data: Rc<RefCell<UseCursorData>>,
    listen_mouse_move: Option<EventListener>,
}

struct UseCursorData {
    pointer: PointerData,
}

impl UseCursorBuilder {
    /// hooks for window's size with config
    ///
    /// # Arguments
    ///
    /// * `cx`: [`Scope`] or [`ScopeState`]
    ///
    /// returns: [`WindowSize`]
    ///
    /// # Examples
    ///
    /// ```
    /// use dioxus::prelude::*;
    /// use dioxus_use_cursor::use_cursor;
    ///
    /// fn App(cx: Scope) -> Element {
    ///     let size = use_cursor(&cx);
    ///
    ///     cx.render(rsx!(
    ///         h1 { "Window size: {size}" }
    ///     ))
    /// }
    /// ```
    pub fn use_cursor<'a>(&self, cx: &'a ScopeState) -> &'a mut UseCursor {
        let hook = UseCursor::new(cx);
        cx.use_hook(|_| hook)
    }
}

impl UseCursor {
    fn new(cx: &ScopeState) -> Self {
        match window() {
            None => Default::default(),
            Some(w) => {
                let data = Rc::new(RefCell::new(UseCursorData::default()));
                let mouse_move = Self::on_pointer_move(cx, &w, &data);
                Self { data, listen_mouse_move: Some(mouse_move) }
            }
        }
    }
    fn on_pointer_move(cx: &ScopeState, window: &Window, data: &Rc<RefCell<UseCursorData>>) -> EventListener {
        #[cfg(debug_assertions)]
        {
            info!("Pointer Move Listener Initialized at {}!", cx.scope_id().0);
        }
        let setter = data.clone();
        let regenerate = cx.schedule_update();
        EventListener::new(&window, "pointermove", move |e| {
            let mut setter = setter.borrow_mut();
            let e: &PointerEvent = e.unchecked_ref();
            setter.pointer = PointerData {
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
                pointer_id: e.pointer_id(),
                width: e.width(),
                height: e.height(),
                pressure: e.pressure(),
                tangential_pressure: e.tangential_pressure(),
                tilt_x: e.tilt_x(),
                tilt_y: e.tilt_y(),
                twist: e.twist(),
                pointer_type: e.pointer_type(),
                is_primary: e.is_primary(),
            };
            regenerate();
        })
    }
}

impl UseCursor {
    /// Getter for the screenX field of this object.
    pub fn screen_x(&self) -> usize {
        self.data.borrow().pointer.screen_x as _
    }
    ///
    pub fn screen_y(&self) -> usize {
        self.data.borrow().pointer.screen_y as _
    }
    ///
    // pub fn element_width(&self) -> usize {
    //     self.view_mouse().map(|e| e.page_x()).unwrap_or_default() as _
    // }
    // ///
    // pub fn element_height(&self) -> usize {
    //     self.view_mouse().map(|e| e.element_height()).unwrap_or_default() as _
    // }
    ///
    pub fn is_over(&self) {}
    ///
    pub fn is_down(&self) {}

    // #[inline]
    // fn view_mouse(&self) -> Option<&MouseEvent> {
    //     self.data.borrow().mouse.as_ref()
    // }
}
