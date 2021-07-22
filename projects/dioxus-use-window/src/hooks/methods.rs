use super::*;

impl WindowSize {
    /// builder of [`WindowSize`]
    pub(crate) fn new(cx: &ScopeState, x: f64, y: f64) -> Option<Self> {
        let window = window()?;
        let regenerate = cx.schedule_update();
        let data = Rc::new(RefCell::new(WindowSizeData { x, y }));
        let setter = data.clone();

        let listener = EventListener::new(&window, "resize", move |_| {
            let mut setter = setter.borrow_mut();
            if let Some(size) = Self::get_size() {
                setter.x = size.0;
                setter.y = size.1;
                regenerate();
            }
        });
        #[cfg(debug_assertions)]
        {
            info!("Windows Resize Listener Initialized!");
        }
        Some(Self { data, listen_window: Some(listener) })
    }
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

impl WindowSize {
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
    pub fn as_width(self) -> WindowWidth {
        WindowWidth { inner: self }
    }
    /// using as [`WindowHeight`]
    #[inline]
    pub fn as_height(self) -> WindowHeight {
        WindowHeight { inner: self }
    }
    /// using as [`WindowLayout`]
    #[inline]
    pub fn as_layout<T>(self) -> WindowLayout<T> {
        WindowLayout { inner: self, bound: Default::default() }
    }
}

impl<T> WindowLayout<T>
where
    T: From<usize>,
{
    /// get layout of current window
    pub fn get(&self) -> T {
        self.inner.layout()
        // T::from(self.inner.x as usize)
    }
}

impl WindowWidth {
    /// get width of current window
    #[inline]
    pub fn get(&self) -> usize {
        self.inner.width()
    }
    /// set width of current window, return `false` if failed to run
    pub fn set(&self, width: usize) -> bool {
        WindowSize::set_window_width(width).is_some()
    }
    /// get layout of current window, return `false` if failed to run
    #[inline]
    pub fn layout<T>(&self) -> T
    where
        T: From<usize>,
    {
        self.inner.layout()
    }
}

impl WindowHeight {
    /// get height of current window
    #[inline]
    pub fn get(&self) -> usize {
        self.inner.height()
    }
    /// set height of current window, return `false` if failed to run
    #[inline]
    pub fn set(&self, height: usize) -> bool {
        WindowSize::set_window_height(height).is_some()
    }
}
