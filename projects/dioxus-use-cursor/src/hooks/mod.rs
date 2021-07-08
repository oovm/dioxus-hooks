mod methods;
mod display;

use std::mem::MaybeUninit;
use std::rc::Rc;
use dioxus::core::ScopeState;
use dioxus::events::MouseData;
use gloo_events::EventListener;
use web_sys::{MouseEvent, window};
use wasm_bindgen::JsCast;

/// effect handler
pub struct UseCursor<'a> {
    data: Option<&'a MouseEvent>,
    update: Rc<dyn Fn()>,
    listen_mouse_move: EventListener
}

///
pub fn use_cursor(_cx: &ScopeState) -> UseCursor {
    todo!()
}

///
#[derive(Debug, Copy, Clone, )]
pub struct UseHover {
    over:bool,
}
///
pub fn use_hover(_cx: &ScopeState) -> UseCursor {
    todo!()
}
