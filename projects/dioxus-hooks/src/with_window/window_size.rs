use dioxus::core::exports::futures_channel::mpsc::{unbounded, UnboundedReceiver};
use dioxus::core::SchedulerMsg;
use super::*;
use log::debug;

pub struct OnWindowResize<'a> {
    x: f64,
    y: f64,
    listener: Option<EventListener>,
    cx: &'a ScopeState
    // receiver: UnboundedReceiver<SchedulerMsg>,
}

impl<'a> OnWindowResize<'a> {
    pub fn new(cx: &'a ScopeState) -> Option<Self> {
        // let (sender, receiver) = unbounded();
        // let id = cx.scope_id();
        let window = window()?;
        let regenerate = cx.schedule_update();
        let mut resize = OnWindowResize {
            x: 0.0,
            y: 0.0,
            listener: None,

            cx
        };
        let listener = EventListener::new(&window, "resize", move |_| {
            Self::get_size().map(|size| {
                resize.x = size.0;
                resize.y = size.1;
                regenerate()
                // sender.unbounded_send(SchedulerMsg::Immediate(id)).ok()
            });
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
        self.inner.x as _
    }
    #[inline]
    pub fn height(&self) -> usize {
        self.inner.x as _
    }
    #[inline]
    pub fn layout<T>(&self) -> T
        where
            T: From<usize>,
    {
        self.width().into()
    }
    pub fn as_width(self) -> WindowWidth<'a> {
        WindowWidth { inner: self.inner }
    }
    pub fn as_height(self) -> WindowHeight<'a> {
        WindowHeight { inner: self.inner }
    }
    pub fn as_layout<T>(self) -> WindowLayout<'a ,T> {
        WindowLayout { inner: self.inner, bound: Default::default() }
    }
}

impl<'a, T> WindowLayout<'a, T>
    where
        T: From<usize>,
{
    pub fn get(&self) -> T {
        T::from(self.inner.x as usize)
    }
}

impl<'a> WindowWidth<'a> {
    #[inline]
    pub fn get(&self) -> usize {
        self.inner.x as _
    }
    pub fn set(&self, width: usize) -> bool {
        false
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
        self.inner.y as _
    }
    pub fn set(&self, height: usize) -> bool {
        false
    }
}
