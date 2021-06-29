#![allow(non_snake_case)]

mod layout;
use self::layout::LayoutSystem;
use dioxus::prelude::*;
use dioxus_hooks::{use_window_layout, use_window_size};

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    // dioxus::desktop::launch(App);
    dioxus::web::launch(App);
}

fn App(cx: Scope) -> Element {
    let size = use_window_size(&cx);
    let layout = use_window_layout::<LayoutSystem>(&cx);

    let a = use_state(&cx, || 0).1.needs_update()

    cx.render(rsx!(
        h1 { "Window size: {size}" }
        h2 { "Current layout: {layout}" }
    ))
}
