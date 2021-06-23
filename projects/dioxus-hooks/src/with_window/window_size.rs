use log::debug;
use super::*;


pub struct OnWindowResize {
    x: f64,
    y: f64,
    listener: Option<EventListener>,
}

impl Default for OnWindowResize {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            listener: None,
        }
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
    pub fn width(&self) -> usize {
        self.inner.get().x as _
    }
    pub fn height(&self) -> usize {
        self.inner.get().x as _
    }
    pub fn force_update(&self)  {

    }
}

