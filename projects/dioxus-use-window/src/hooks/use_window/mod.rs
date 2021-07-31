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
    fn new_ssr(_: &ScopeState, config: &UseWindowBuilder) -> Self {
        info!("Window Resize Listener Initializing failed, using ssr mode now.");
        let data = WindowSizeData::new(None, (config.missing_x as _, config.missing_y as _));
        Self { data, listen_window: None }
    }
    fn on_window_resize(cx: &ScopeState, window: &Window, _: &Rc<RefCell<WindowSizeData>>) -> EventListener {
        #[cfg(debug_assertions)]
        {
            info!("Windows Resize Listener Initialized at {}!", cx.scope_id().0);
        }
        let regenerate = cx.schedule_update();
        EventListener::new(window, "resize", move |_| {
            regenerate();
        })
    }
    #[inline]
    pub(crate) fn data_ref(&self) -> Ref<WindowSizeData> {
        self.data.borrow()
    }
}

impl UseWindow {
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
