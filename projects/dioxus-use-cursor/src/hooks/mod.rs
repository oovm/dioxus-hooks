mod methods;
mod display;

use dioxus::core::ScopeState;
use dioxus::hooks::use_ref;
use gloo_events::EventListener;
use web_sys::{MouseEvent, window};
use wasm_bindgen::cast::JsCast;

pub struct UseCursor {
    last_event: Option<MouseEvent>
}

pub fn use_cursor(_cx: &ScopeState) -> UseCursor {
    todo!()
}

pub struct UseHover {
    over:bool,
}

pub fn use_hover(_cx: &ScopeState) -> UseCursor {
    todo!()
}
