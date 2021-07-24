use super::*;

mod display;

/// Window size effect handler
pub struct UseWindowSize {
    data: Rc<RefCell<WindowSizeData>>,
    listen_window: Option<EventListener>,
}

pub(crate) struct WindowSizeData {
    x: f64,
    y: f64,
}

impl UseWindowSize {
    pub(crate) fn new(cx: &ScopeState) -> Option<Self> {
        let window = window()?;
        let size = Self::get_size()?;
        let data = Rc::new(RefCell::new(WindowSizeData { x: size.0, y: size.1 }));
        let listener = Self::on_window_resize(cx, &window, &data);
        Some(Self { data, listen_window: Some(listener) })
    }
    pub(crate) fn new_ssr(_: &ScopeState, data: WindowSizeData) -> Self {
        let data = Rc::new(RefCell::new(data));
        Self { data, listen_window: None }
    }
    fn on_window_resize(cx: &ScopeState, window: &Window, data: &Rc<RefCell<WindowSizeData>>) -> EventListener {
        #[cfg(debug_assertions)]
        {
            info!("Windows Resize Listener Initialized at {}!", cx.scope_id().0);
        }
        let regenerate = cx.schedule_update();
        let setter = data.clone();
        EventListener::new(window, "resize", move |_| {
            let mut setter = setter.borrow_mut();
            if let Some(size) = Self::get_size() {
                setter.x = size.0;
                setter.y = size.1;
                regenerate();
            }
        })
    }
}

impl UseWindowSize {
    /// get size of the current window, return `None` if window not found
    pub fn get_size() -> Option<(f64, f64)> {
        let window = window()?;
        let x = window.inner_width().ok()?.as_f64()?;
        let y = window.inner_height().ok()?.as_f64()?;
        Some((x, y))
    }
    /// set width of the current window, return `None` if failed to run
    pub fn set_window_width(input: usize) -> Option<()> {
        window()?.set_inner_width(&JsValue::from(input)).ok()
    }
    /// set height of the current window, return `None` if failed to run
    pub fn set_window_height(input: usize) -> Option<()> {
        window()?.set_inner_width(&JsValue::from(input)).ok()
    }
}

impl UseWindowSize {
    /// get width of current window
    #[inline]
    pub fn width(&self) -> usize {
        self.data.borrow().x as _
    }
    /// get height of current window
    #[inline]
    pub fn height(&self) -> usize {
        self.data.borrow().y as _
    }
    /// get layout of current window
    #[inline]
    pub fn layout<T>(&self) -> T
    where
        T: From<usize>,
    {
        self.width().into()
    }
    /// get aspect radio of current window
    #[inline]
    pub fn aspect_radio(&self) -> f64 {
        let data = self.data.borrow();
        data.x / data.y
    }
    /// using as [`WindowWidth`]
    #[inline]
    pub fn as_width(self) -> UseWindowWidth {
        UseWindowWidth::new(self)
    }
    /// using as [`WindowHeight`]
    #[inline]
    pub fn as_height(self) -> UseWindowHeight {
        UseWindowHeight::new(self)
    }
    /// using as [`WindowLayout`]
    #[inline]
    pub fn as_layout<T>(self) -> UseWindowLayout<T> {
        UseWindowLayout::new(self)
    }
}
