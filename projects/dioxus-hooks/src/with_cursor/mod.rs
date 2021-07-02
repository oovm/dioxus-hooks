mod methods;
use dioxus::core::ScopeState;
use dioxus::hooks::use_ref;
use gloo_events::EventListener;

pub struct UseCursor {}

pub fn use_cursor(_cx: &ScopeState) -> UseCursor {
    todo!()
}

pub struct UseHover {
    inner: UseCursor
}

pub fn use_hover(_cx: &ScopeState) -> UseCursor {
    todo!()
}
