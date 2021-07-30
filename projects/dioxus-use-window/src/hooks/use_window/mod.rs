use super::*;

pub mod data;
mod display;

/// Window size effect handler
pub struct UseWindow {
    data: Rc<RefCell<WindowSizeData>>,
    listen_window: Option<EventListener>,
}

impl UseWindowBuilder {
    /// hooks for window's size with config
    ///
    /// # Arguments
    ///
    /// returns: [`UseWindowSize`]
    ///
    /// # Examples
    ///
    /// ```
    /// use dioxus::prelude::*;
    /// use dioxus_use_window::{UseWindowBuilder};
    ///
    /// fn App(cx: Scope) -> Element {
    ///     let hook = UseWindowBuilder::default().use_window(&cx);
    ///
    ///     cx.render(rsx!(
    ///         h1 { "Window size: {hook}" }
    ///     ))
    /// }
    /// ```
    pub fn use_window<'a>(&self, cx: &'a ScopeState) -> &'a mut UseWindow {
        let hook = self.use_window_hook(cx);
        cx.use_hook(|_| hook)
    }
    #[inline]
    pub(crate) fn use_window_hook(&self, cx: &ScopeState) -> UseWindow {
        UseWindow::new(cx, self).unwrap_or_else(|| UseWindow::new_ssr(cx, self))
    }
}

impl UseWindow {
    fn new(cx: &ScopeState, config: &UseWindowBuilder) -> Option<Self> {
        let data = WindowSizeData::new(window(), (config.missing_x as _, config.missing_y as _));
        let window = window()?;
        let listener = Self::on_window_resize(cx, &window, &data);
        Some(Self { data, listen_window: Some(listener) })
    }
    fn new_ssr(cx: &ScopeState, config: &UseWindowBuilder) -> Self {
        warn!("Window Resize Listener Initializing failed at {}!", cx.scope_id().0);
        let data = WindowSizeData::new(None, (config.missing_x as _, config.missing_y as _));
        Self { data, listen_window: None }
    }
    #[allow(unused_variables)]
    fn on_window_resize(cx: &ScopeState, window: &Window, data: &Rc<RefCell<WindowSizeData>>) -> EventListener {
        #[cfg(debug_assertions)]
        {
            info!("Windows Resize Listener Initialized at {}!", cx.scope_id().0);
        }
        let regenerate = cx.schedule_update();
        // let setter = data.clone();
        EventListener::new(window, "resize", move |_| {
            // let mut setter = setter.borrow_mut();
            // if let Some(size) = get_size() {
            // setter.x = size.0;
            // setter.y = size.1;
            // regenerate();
            // }
            regenerate();
        })
    }
}

impl UseWindow {
    /// get width of current window
    #[inline]
    pub(crate) fn data(&self) -> Ref<WindowSizeData> {
        self.data.borrow()
    }
    /// get height of current window
    ///
    /// **read-only**
    #[inline]
    pub fn size(&self) -> (usize, usize) {
        (self.width(), self.height())
    }
    /// get height of current window
    ///
    /// **read-only**
    #[inline]
    pub fn width(&self) -> usize {
        self.data.borrow().inner_width() as usize
    }
    /// get height of current window
    ///
    /// **read-only**
    #[inline]
    pub fn height(&self) -> usize {
        self.data.borrow().inner_height() as usize
    }
    /// get aspect radio of current window
    #[inline]
    pub fn aspect_radio(&self) -> f64 {
        self.data.borrow().inner_aspect_radio()
    }
    /// get layout of current window
    #[inline]
    pub fn layout<T>(&self) -> T
    where
        T: From<usize>,
    {
        let w = self.data.borrow().inner_width() as usize;
        T::from(w)
    }
}

impl UseWindow {
    /// get size of current browser
    #[inline]
    pub fn browser_size(&self) -> (usize, usize) {
        (self.width(), self.height())
    }
    /// get width of current browser
    ///
    /// **read-only**
    #[inline]
    pub fn browser_width(&self) -> usize {
        self.data.borrow().outer_width() as usize
    }
    /// get height of current window
    ///
    /// **read-only**
    #[inline]
    pub fn browser_height(&self) -> usize {
        self.data.borrow().outer_height() as usize
    }
    /// get aspect radio of current window
    #[inline]
    pub fn browser_aspect_radio(&self) -> f64 {
        self.data.borrow().outer_aspect_radio()
    }
    /// get height of current window
    #[inline]
    pub fn browser_size_set(&self, size: (usize, usize)) -> bool {
        self.data.borrow().resize_outer_size(size.0 as _, size.1 as _).is_some()
    }
    /// get height of current window
    #[inline]
    pub fn browser_size_delta(&self, size: (i32, i32)) -> bool {
        self.data.borrow().resize_outer_delta(size.0, size.1).is_some()
    }
}

// noinspection RsSelfConvention
impl UseWindow {
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
