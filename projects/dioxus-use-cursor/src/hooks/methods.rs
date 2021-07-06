use super::*;


impl UseCursor {
    pub fn new(cx: &ScopeState) -> Option<Self> {
        let window = window()?;
        let mut hook = UseCursor::default();
        let regenerate = cx.schedule_update();

        let mouseout = EventListener::new(&window, "mousemove", move |e| {
            let e: MouseEvent = e.unchecked_into();
            hook.last_event = Some(e);
            regenerate()
        });
    }
}


impl UseCursor {

    /// Getter for the screenX field of this object.
    pub fn screen_x(&self) -> usize {
        match &self.last_event {
            None => { Default::default() }
            Some(e) => {
                e.screen_x() as _
            }
        }
    }
    ///
    pub fn screen_y(&self) -> usize {
        self.last_event.unwrap_or_default().screen_y() as _
    }
    ///
    pub fn element_width(&self) -> usize {
        match &self.last_event {
            None => { Default::default() }
            Some(e) => {
                e.screen_x() as _
            }
        }
    }
    ///
    pub fn element_height(&self) -> usize {
        match &self.last_event {
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
    #[inline]
    pub fn get(&self) {
        self.over.is_over()
    }
}