mod display;
mod window_size;

use dioxus::core::ScopeState;
use gloo_events::EventListener;
use std::{
    fmt::{Display, Formatter},
    marker::PhantomData,
};
use web_sys::window;

pub struct WindowSize {
    x: f64,
    y: f64,
    listener: Option<EventListener>,
}

pub fn use_window_size(cx: &ScopeState) -> WindowSize {
    WindowSize::new(&cx).unwrap()
}

pub struct WindowLayout<T> {
    inner: WindowSize,
    bound: PhantomData<T>,
}

pub fn use_window_layout<T>(cx: &ScopeState) -> WindowLayout<T>
where
    T: From<usize>,
{
    WindowLayout { inner: WindowSize::new(cx).unwrap(), bound: Default::default() }
}

pub struct WindowWidth {
    inner: WindowSize,
}

pub fn use_width(cx: &ScopeState) -> WindowWidth {
    WindowWidth { inner: WindowSize::new(cx).unwrap() }
}

pub struct WindowHeight {
    inner: WindowSize,
}

pub fn use_height(cx: &ScopeState) -> WindowHeight {
    WindowHeight { inner: WindowSize::new(cx).unwrap() }
}
