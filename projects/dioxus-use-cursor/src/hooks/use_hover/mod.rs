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
    /// use dioxus_use_cursor::use_hover;
    ///
    /// fn App(cx: Scope) -> Element {
    ///     let hook = use_hover(&cx);
    ///
    ///     cx.render(rsx!(
    ///         h1 { "Cursor is hovering: {hook}" }
    ///     ))
    /// }
    /// ```
    pub fn use_hover<'a>(&self, cx: &'a ScopeState) -> &'a mut UseHover {
        let hook = UseHover::new(cx, todo!());
        cx.use_hook(|| hook)
    }
}

impl UseHover {
    fn new(cx: &ScopeState, target: &EventTarget) -> Self {
        let data = Rc::new(RefCell::new(UseHoverData::default()));
        let listen_mouse_over = Some(Self::on_pointer_over(cx, target, &data));
        let listen_mouse_out = Some(Self::on_pointer_out(cx, target, &data));
        Self { data, listen_mouse_over, listen_mouse_out }
    }
    fn on_pointer_over(cx: &ScopeState, target: &EventTarget, data: &Rc<RefCell<UseHoverData>>) -> EventListener {
        #[cfg(debug_assertions)]
        {
            info!("Mouse Over Listener Initialized at {}!", cx.scope_id().0);
        }
        let setter = data.clone();
        let regenerate = cx.schedule_update();
        EventListener::new(target, "pointerover", move |_| {
            let mut setter = setter.borrow_mut();
            setter.hover = true;
            regenerate()
        })
    }
    fn on_pointer_out(cx: &ScopeState, target: &EventTarget, data: &Rc<RefCell<UseHoverData>>) -> EventListener {
        #[cfg(debug_assertions)]
        {
            info!("Mouse Out Listener Initialized at {}!", cx.scope_id().0);
        }
        let setter = data.clone();
        let regenerate = cx.schedule_update();
        EventListener::new(target, "pointerout", move |_| {
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
