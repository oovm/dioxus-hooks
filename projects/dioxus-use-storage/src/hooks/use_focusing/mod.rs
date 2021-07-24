mod display;

use super::*;
use dioxus::hooks::use_ref;
use log::info;
use web_sys::EventTarget;

///
#[derive(Debug, Clone)]
pub struct UseFocused {
    data: Rc<RefCell<UseHoverData>>,
    listen_mouse_over: EventListener,
    listen_mouse_out: EventListener,
}

#[derive(Debug, Clone)]
struct UseHoverData {
    hover: bool,
}

/// https://www.npmjs.com/package/@react-hook/mouse-position
impl UseFocused {
    pub(crate) fn new(cx: &ScopeState, target: &EventTarget) -> Self {
        let mut data = Rc::new(RefCell::new(UseHoverData::default()));
        let regenerate = cx.schedule_update();
        let setter = data.clone();
        let listen_mouse_over = EventListener::new(target, "mouseover", move |_| {
            let mut setter = setter.borrow_mut();
            setter.hover = true;
            regenerate()
        });
        let setter = data.clone();
        let listen_mouse_out = EventListener::new(target, "mouseout", move |_| {
            let mut setter = setter.borrow_mut();
            setter.hover = false;
            regenerate()
        });
        #[cfg(debug_assertions)]
        {
            info!("Mouse Over Listener Initialized at {}!", cx.scope_id().0);
            info!("Mouse Out Listener Initialized at {}!", cx.scope_id().0);
        }
        Self { data, listen_mouse_over, listen_mouse_out }
    }
}

impl UseFocused {
    /// check if target is hovering
    #[inline]
    pub fn get(&self) -> bool {
        self.data.borrow().hover
    }
}
