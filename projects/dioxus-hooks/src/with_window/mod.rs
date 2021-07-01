mod display;
mod window_size;

pub use self::window_size::OnWindowResize;

use dioxus::{
    core::ScopeState,
    hooks::{use_state, UseState},
};
use gloo_events::EventListener;
use std::{
    fmt::{Display, Formatter},
    marker::PhantomData,
};
use std::cell::RefCell;
use std::rc::Rc;
use web_sys::window;


pub struct WindowSize<'a> {
    inner: OnWindowResize<'a>,
}

pub fn use_window_size(cx: &ScopeState) -> WindowSize {
    WindowSize {
        inner: OnWindowResize::new(&cx).unwrap()
    }
}

pub struct WindowLayout<'a, T> {
    inner: OnWindowResize<'a>,
    bound: PhantomData<T>,
}

pub fn use_window_layout<T>(cx: &ScopeState) -> WindowLayout<T>
    where
        T: From<usize>,
{
    WindowLayout {
        inner: OnWindowResize::new(cx).unwrap(),
        bound: Default::default(),
    }
}

pub struct WindowWidth<'a> {
    inner: OnWindowResize<'a>,
}

pub fn use_width(cx: &ScopeState) -> WindowWidth {
    WindowWidth { inner: OnWindowResize::new(cx).unwrap() }
}

pub struct WindowHeight<'a> {
    inner: OnWindowResize<'a>,
}

pub fn use_height(cx: &ScopeState) -> WindowHeight {
    WindowHeight { inner: OnWindowResize::new(cx).unwrap() }
}
