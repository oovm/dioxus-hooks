mod window_size;

use dioxus::core::{ScopeState};
use dioxus::hooks::{use_state, UseState};
use gloo_events::EventListener;
use web_sys::{UiEvent, window, Window};
use web_sys::EventTarget;

pub fn use_window_size(cx: &ScopeState) -> WindowSize {
    WindowSize {
        inner: use_state(&cx, || OnWindowResize::default()).1
    }
}

pub struct WindowSize<'a> {
    inner: &'a UseState<OnWindowResize>,
}

pub struct OnWindowResize {
    x: f64,
    y: f64,
    listener: Option<EventListener>,
}