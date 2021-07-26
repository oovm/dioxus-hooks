mod display;

use super::*;
use web_sys::Window;

/// effect handler
pub struct UseCursor {
    data: Rc<RefCell<UseCursorData>>,
    listen_mouse_move: Option<EventListener>,
}

struct UseCursorData {
    mouse: MouseData,
}

impl UseCursor {
    /// builder of `UseCursor`
    pub(crate) fn new(cx: &ScopeState) -> Option<Self> {
        let window = window()?;
        let data = Rc::new(RefCell::new(UseCursorData::default()));
        let mouse_move = Self::on_mouse_move(cx, &window, &data);
        Some(Self { data, listen_mouse_move: Some(mouse_move) })
    }
    fn on_mouse_move(cx: &ScopeState, window: &Window, data: &Rc<RefCell<UseCursorData>>) -> EventListener {
        #[cfg(debug_assertions)]
        {
            info!("Window Mouse move Listener Initialized at {}!", cx.scope_id().0);
        }
        let setter = data.clone();
        let regenerate = cx.schedule_update();
        EventListener::new(&window, "mousemove", move |e| {
            let mut setter = setter.borrow_mut();
            let e: &MouseEvent = e.unchecked_ref();
            setter.mouse = MouseData {
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
            };
            regenerate();
        })
    }
}

impl UseCursor {
    /// Getter for the screenX field of this object.
    pub fn screen_x(&self) -> usize {
        self.data.borrow().mouse.screen_x as _
    }
    ///
    pub fn screen_y(&self) -> usize {
        self.data.borrow().mouse.screen_y as _
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
