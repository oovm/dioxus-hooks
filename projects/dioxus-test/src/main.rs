#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_hooks::use_window_size;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    // dioxus::desktop::launch(App);
    dioxus::web::launch(App);

}

fn App(cx: Scope) -> Element {
    let size = use_window_size(&cx);

    cx.render(rsx!(
        h1 { "Window size: {size}" }
    ))
}