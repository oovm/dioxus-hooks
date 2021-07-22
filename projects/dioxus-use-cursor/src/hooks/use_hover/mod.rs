mod display;

use super::*;


///
#[derive(Debug)]
pub struct UseHover {
    data: Rc<RefCell<UseHoverData>>,
    listen_mouse_over: EventListener,
    listen_mouse_out: EventListener,
}

#[derive(Debug, Clone)]
struct UseHoverData {
    hover: bool,
}

impl UseHover {
    pub(crate) fn new(cx: &ScopeState, target: &EventTarget) -> Self {
        let mut data = Rc::new(RefCell::new(UseHoverData::default()));

        let setter = data.clone();
        let regenerate = cx.schedule_update();
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

        Self { data, listen_mouse_over, listen_mouse_out }
    }
    fn on_mouse_over(cx: &ScopeState, target: &EventTarget, data: Rc<RefCell<UseHoverData>>) -> EventListener {
        #[cfg(debug_assertions)]
            {
                info!("Mouse Over Listener Initialized at {}!", cx.scope_id().0);
                info!("Mouse Out Listener Initialized at {}!", cx.scope_id().0);
            }
        let setter = data.clone();
        let regenerate = cx.schedule_update();
        EventListener::new(target, "mouseover", move |_| {
            let mut setter = setter.borrow_mut();
            setter.hover = true;
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
