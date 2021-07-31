mod display;
use super::*;

/// Window height effect handler
#[derive(Debug)]
pub struct UseFullScreen {
    data: Rc<RefCell<WindowSizeData>>,
    listen_fullscreen: Option<EventListener>,
}

impl UseWindowBuilder {
    /// hooks for window's height with config
    ///
    /// # Arguments
    ///
    /// returns: [`UseWindowHeight`]
    ///
    /// # Examples
    ///
    /// ```
    /// use dioxus::prelude::*;
    /// use dioxus_use_window::{UseWindowBuilder};
    ///
    /// fn App(cx: Scope) -> Element {
    ///     let hook = UseWindowBuilder::default().use_browser(&cx);
    ///
    ///     cx.render(rsx!(
    ///         h1 { "Window height: {hook}" }
    ///     ))
    /// }
    /// ```
    #[inline]
    pub fn use_full_screen<'a>(&self, cx: &'a ScopeState) -> &'a mut UseFullScreen {
        let hook = UseFullScreen::new(self.use_window_hook(cx));
        cx.use_hook(|_| hook)
    }
}

impl UseFullScreen {
    fn new(cx: &ScopeState, _: &UseWindowBuilder) -> Option<Self> {
        let data = WindowSizeData::new(window(), (0.0, 0.0));
        let window = window()?;
        let listener = Self::on_fullscreen_change(cx, &window, &data);
        Some(Self { data, listen_fullscreen: Some(listener) })
    }
    fn new_ssr(_: &ScopeState, _: &UseWindowBuilder) -> Self {
        info!("Window Resize Listener Initializing failed, using ssr mode now.");
        let data = WindowSizeData::new(None, (0.0, 0.0));
        Self { data, listen_fullscreen: None }
    }
    fn on_fullscreen_change(cx: &ScopeState, window: &Window, _: &Rc<RefCell<WindowSizeData>>) -> EventListener {
        #[cfg(debug_assertions)]
        {
            info!("Windows Resize Listener Initialized at {}!", cx.scope_id().0);
        }
        let regenerate = cx.schedule_update();
        EventListener::new(window, "fullscreenchange", move |_| {
            regenerate();
        })
    }
    #[inline]
    pub(crate) fn data_ref(&self) -> Ref<WindowSizeData> {
        self.data.borrow()
    }
}

impl UseFullScreen {
    #[inline]
    pub(crate) fn new(size: UseWindow) -> Self {
        Self { inner: size }
    }
    /// get size of current browser
    #[inline]
    pub fn is_full(&self) -> (usize, usize) {
        (self.toggle(), self.height())
    }
    /// get width of current browser
    #[inline]
    pub fn toggle(&self) -> usize {
        self.data.borrow()
    }
    #[inline]
    pub fn on(&self) -> usize {
        self.inner.data_ref().outer_width() as usize
    }
    #[inline]
    pub fn off(&self) -> usize {
        self.inner.data_ref().outer_width() as usize
    }
}
