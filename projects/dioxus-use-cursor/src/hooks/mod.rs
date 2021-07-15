mod builder;
mod display;
mod methods;

pub use self::builder::UseCursorBuilder;
use self::builder::UseCursorData;
use dioxus::{core::ScopeState, events::MouseData};
use gloo_events::EventListener;
use std::{cell::RefCell, mem::MaybeUninit, rc::Rc};
use wasm_bindgen::JsCast;
use web_sys::{window, Event, MouseEvent};

/// effect handler
pub struct UseCursor {
    data: Rc<RefCell<UseCursorData>>,
    listen_mouse_move: EventListener,
}

///
pub fn use_cursor(cx: &ScopeState) -> UseCursor {
    todo!()
}

///
#[derive(Debug, Copy, Clone)]
pub struct UseHover {
    over: bool,
}
///
pub fn use_hover(_cx: &ScopeState) -> UseCursor {
    todo!()
}
