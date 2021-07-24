mod display;

use super::*;

///
pub struct UseHover {
    data: Rc<RefCell<UseHoverData>>,
    listen_mouse_over: Option<EventListener>,
    listen_mouse_out: Option<EventListener>,
}

#[derive(Debug, Clone)]
struct UseHoverData {
    hover: bool,
}

impl UseHover {
    pub(crate) fn new(cx: &ScopeState, target: &EventTarget) -> Self {
        let data = Rc::new(RefCell::new(UseHoverData::default()));
        let listen_mouse_over = Some(Self::on_mouse_over(cx, target, &data));
        let listen_mouse_out = Some(Self::on_mouse_out(cx, target, &data));
        Self { data, listen_mouse_over, listen_mouse_out }
    }
    fn on_mouse_over(cx: &ScopeState, target: &EventTarget, data: &Rc<RefCell<UseHoverData>>) -> EventListener {
        #[cfg(debug_assertions)]
        {
            info!("Mouse Over Listener Initialized at {}!", cx.scope_id().0);
        }
        let setter = data.clone();
        let regenerate = cx.schedule_update();
        EventListener::new(target, "mouseover", move |_| {
            let mut setter = setter.borrow_mut();
            setter.hover = true;
            regenerate()
        })
    }
    fn on_mouse_out(cx: &ScopeState, target: &EventTarget, data: &Rc<RefCell<UseHoverData>>) -> EventListener {
        #[cfg(debug_assertions)]
        {
            info!("Mouse Out Listener Initialized at {}!", cx.scope_id().0);
        }
        let setter = data.clone();
        let regenerate = cx.schedule_update();
        EventListener::new(target, "mouseout", move |_| {
            let mut setter = setter.borrow_mut();
            setter.hover = false;
            regenerate()
        })
    }
}

impl UseHover {
    /// check if the target is hovering
    #[inline]
    pub fn get(&self) -> bool {
        self.data.borrow().hover
    }
}
