mod display;
mod window_size;

use dioxus::{
    core::ScopeState,
    hooks::{use_state, UseState},
};
use gloo_events::EventListener;
use std::{
    fmt::{Display, Formatter},
    marker::PhantomData,
};
use web_sys::window;

pub use self::window_size::OnWindowResize;

pub struct WindowSize<'a> {
    inner: &'a UseState<OnWindowResize>,
}

pub fn use_window_size(cx: &ScopeState) -> WindowSize {
    WindowSize { inner: use_state(cx, || OnWindowResize::new().unwrap()).1 }
}

pub struct WindowLayout<'a, T> {
    inner: &'a UseState<OnWindowResize>,
    bound: PhantomData<T>,
}

pub fn use_window_layout<T>(cx: &ScopeState) -> WindowLayout<T>
where
    T: From<usize>,
{
    WindowLayout { inner: use_state(cx, || OnWindowResize::new().unwrap()).1, bound: Default::default() }
}

pub struct WindowWidth<'a> {
    inner: &'a UseState<OnWindowResize>,
}

pub fn use_width(cx: &ScopeState) -> WindowWidth {
    WindowWidth { inner: use_state(cx, || OnWindowResize::new().unwrap()).1 }
}

pub struct WindowHeight<'a> {
    inner: &'a UseState<OnWindowResize>,
}

pub fn use_height(cx: &ScopeState) -> WindowHeight {
    WindowHeight { inner: use_state(cx, || OnWindowResize::new().unwrap()).1 }
}
