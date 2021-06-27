mod window_size;
mod display;

use std::fmt::{Display, Formatter};
use dioxus::core::{ScopeState};
use dioxus::hooks::{use_state, UseState};
use gloo_events::EventListener;
use web_sys::{UiEvent, window, Window};
use web_sys::EventTarget;
pub use self::window_size::OnWindowResize;

pub struct WindowSize<'a> {
    inner: &'a UseState<OnWindowResize>,
}

pub fn use_window_size(cx: &ScopeState) -> WindowSize {
    WindowSize {
        inner: use_state(cx, || OnWindowResize::new().unwrap()).1
    }
}

pub struct WindowWidth<'a> {
    inner: &'a UseState<OnWindowResize>,
}

pub fn use_width(cx: &ScopeState) -> WindowWidth {
    WindowWidth {
        inner: use_state(cx, || OnWindowResize::new().unwrap()).1
    }
}

pub struct WindowHeight<'a> {
    inner: &'a UseState<OnWindowResize>,
}

pub fn use_height(cx: &ScopeState) -> WindowHeight {
    WindowHeight {
        inner: use_state(cx, || OnWindowResize::new().unwrap()).1
    }
}






