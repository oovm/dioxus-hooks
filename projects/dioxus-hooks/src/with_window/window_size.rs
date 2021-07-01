use super::*;
use log::debug;

impl Default for WindowSize {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, listener: None }
    }
}

impl WindowSize {
    pub(crate) fn new(cx: &ScopeState) -> Option<Self> {
        debug!("Windows Resize Listener");
        let window = window()?;
        let regenerate = cx.schedule_update();
        let mut hook = WindowSize { x: 0.0, y: 0.0, listener: None };
        let listener = EventListener::new(&window, "resize", move |_| {
            if let Some(size) = Self::get_size() {
                hook.x = size.0;
                hook.y = size.1;
                // hook.updater();
                regenerate();
                debug!("Windows Resize Event: {:?}", size)
            }
        });
        hook.listener = Some(listener);
        Some(hook)
    }
    pub fn get_size() -> Option<(f64, f64)> {
        let window = window()?;
        let x = window.inner_width().ok()?.as_f64()?;
        let y = window.inner_height().ok()?.as_f64()?;
        Some((x, y))
    }
    pub fn set_window_width() -> Option<()> {
        todo!()
        // let window = window()?.set_inner_width();
    }
    pub fn set_window_height() -> Option<()> {
        todo!()
        // let window = window()?.set_inner_width();
    }
}

impl WindowSize {
    #[inline]
    pub fn width(&self) -> usize {
        self.x as _
    }
    #[inline]
    pub fn height(&self) -> usize {
        self.x as _
    }
    #[inline]
    pub fn layout<T>(&self) -> T
    where
        T: From<usize>,
    {
        self.width().into()
    }
    pub fn as_width(self) -> WindowWidth {
        WindowWidth { inner: self }
    }
    pub fn as_height(self) -> WindowHeight {
        WindowHeight { inner: self }
    }
    pub fn as_layout<T>(self) -> WindowLayout<T> {
        WindowLayout { inner: self, bound: Default::default() }
    }
}

impl<T> WindowLayout<T>
where
    T: From<usize>,
{
    pub fn get(&self) -> T {
        self.inner.layout()
        // T::from(self.inner.x as usize)
    }
}

impl WindowWidth {
    #[inline]
    pub fn get(&self) -> usize {
        self.inner.width()
    }
    pub fn set(&self, _width: usize) -> bool {
        WindowSize::set_window_width().is_some()
    }
    #[inline]
    pub fn layout<T>(&self) -> T
    where
        T: From<usize>,
    {
        self.inner.layout()
    }
}

impl WindowHeight {
    #[inline]
    pub fn get(&self) -> usize {
        self.inner.height()
    }
    pub fn set(&self, _height: usize) -> bool {
        WindowSize::set_window_height().is_some()
    }
}
