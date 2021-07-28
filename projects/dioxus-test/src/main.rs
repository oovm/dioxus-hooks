#![allow(non_snake_case)]

mod test_storage;
pub use self::test_storage::TestStorage;
use dioxus::prelude::*;
use dioxus_hooks_plus::{use_local_storage, use_responsive_layout, use_session_storage, use_window_size};

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

fn TestWindow(cx: Scope) -> Element {
    let size = use_window_size(&cx);
    let layout = use_responsive_layout(&cx);
    let ap = size.aspect_radio();

    cx.render(rsx!(
        div { "Window size: {size}, aspect radio: {ap}." }
        div { "Current layout: {layout}." }
    ))
}
