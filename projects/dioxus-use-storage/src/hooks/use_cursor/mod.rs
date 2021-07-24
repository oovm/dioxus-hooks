mod display;

use super::*;

/// effect handler
pub struct UseLocalStorage {
    data: Rc<RefCell<UseLocalStorageData>>,
    listen_mouse_move: Option<EventListener>,
}

struct UseLocalStorageData {
    mouse: MouseData,
}

impl UseLocalStorage {
    /// builder of `UseCursor`
    pub(crate) fn new(cx: &ScopeState) -> Option<Self> {
        let storage = window()?.local_storage().ok()??;

        storage.get();

        storage.set_item()

        let data = Rc::new(RefCell::new(UseLocalStorageData::default()));
        let setter = data.clone();
        let regenerate = cx.schedule_update();
        let mouse_move = EventListener::new(&storage, "mousemove", move |e| {
            let mut setter = setter.borrow_mut();
            setter.mouse = Self::event_as_mouse_data(e);
            regenerate();
        });
        Some(Self { data, listen_mouse_move: Some(mouse_move) })
    }
    fn event_as_mouse_data(event: &Event) -> MouseData {
        let e: &MouseEvent = event.unchecked_ref();
        MouseData {
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
        }
    }
    fn on_change() {

    }
}

impl UseLocalStorage {
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
