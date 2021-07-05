use super::*;


impl UseCursor {
    // pub fn new(cx: &ScopeState) {
    //     let mut hook = UseHover {
    //         inner: false
    //     };
    //     let mouseover = EventListener::new(&window, "mouseover", move |_| {
    //         hook.inner = true;
    //     });
    //     let mouseout = EventListener::new(&window, "mouseout", move |_| {
    //         hook.inner = false;
    //     });
    // }
}

/// https://www.npmjs.com/package/@react-hook/mouse-position
impl UseCursor {
    pub fn screen_x() {

    }
    pub fn screen_y() {

    }
    pub fn element_width() {

    }
    pub fn element_height() {

    }
    pub fn is_over(&self) {

    }
    pub fn is_down(&self) {

    }
}

impl UseHover {
    #[inline]
    pub fn get(&self) {
        self.inner.is_over()
    }
}