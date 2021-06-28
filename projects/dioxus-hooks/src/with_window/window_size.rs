use super::*;
use log::debug;

pub struct OnWindowResize {
    x: f64,
    y: f64,
    listener: Option<EventListener>,
}

impl Default for OnWindowResize {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, listener: None }
    }
}

impl OnWindowResize {
    pub fn new() -> Option<Self> {
        let window = window()?;
        let mut resize = OnWindowResize::default();
        let listener = EventListener::new(&window, "resize", move |_| {
            Self::get_size().map(|size| {
                resize.x = size.0;
                resize.y = size.1;
            });
            debug!("{:?}", Self::get_size())
        });
        resize.listener = Some(listener);
        Some(resize)
    }
    pub fn get_size() -> Option<(f64, f64)> {
        let window = window()?;
        let x = window.inner_width().ok()?.as_f64()?;
        let y = window.inner_height().ok()?.as_f64()?;
        Some((x, y))
    }
}

impl<'a> WindowSize<'a> {
    #[inline]
    pub fn width(&self) -> usize {
        self.inner.get().x as _
    }
    #[inline]
    pub fn height(&self) -> usize {
        self.inner.get().x as _
    }
    #[inline]
    pub fn layout<T>(&self) -> T
    where
        T: From<usize>,
    {
        self.width().into()
    }
    pub fn as_width(&self) -> WindowWidth {
        WindowWidth { inner: self.inner }
    }
    pub fn as_height(&self) -> WindowHeight {
        WindowHeight { inner: self.inner }
    }
}

impl<'a, T> WindowLayout<'a, T>
where
    T: From<usize>,
{
    pub fn get(&self) -> T {
        T::from(self.inner.get().x as usize)
    }
}

impl<'a> WindowWidth<'a> {
    #[inline]
    pub fn get(&self) -> usize {
        self.inner.get().x as _
    }
    #[inline]
    pub fn layout<T>(&self) -> T
    where
        T: From<usize>,
    {
        self.get().into()
    }
}

impl<'a> WindowHeight<'a> {
    #[inline]
    pub fn get(&self) -> usize {
        self.inner.get().y as _
    }
}
