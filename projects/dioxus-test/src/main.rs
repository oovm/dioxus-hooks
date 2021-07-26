#![allow(non_snake_case)]

// mod layout;

// use self::layout::LayoutSystem;
use dioxus::prelude::*;
use dioxus_hooks_plus::{use_local_storage, use_responsive_layout, use_window_size};

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    // dioxus::desktop::launch(App);
    dioxus::web::launch(App);
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! (
        div {
            TestWindow {}
            TestStorage {}
        }
    ))
}

fn TestStorage(cx: Scope) -> Element {
    let local = use_local_storage(&cx);

    cx.render(rsx!(
        h1 { "Local storage: {local:#?}" }
    ))
}

fn TestWindow(cx: Scope) -> Element {
    let size = use_window_size(&cx);
    let layout = use_responsive_layout(&cx);
    let ap = size.aspect_radio();

    cx.render(rsx!(
        h1 { "Window size: {size}, aspect radio: {ap}." }
        h2 { "Current layout: {layout}." }
    ))
}
