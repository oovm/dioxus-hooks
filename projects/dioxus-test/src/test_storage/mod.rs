use super::*;

pub fn TestStorage(cx: Scope) -> Element {
    cx.render(rsx!(
        TestLocalStorage {}
        TestSessionStorage {}
    ))
}
pub fn TestLocalStorage(cx: Scope) -> Element {
    let hook = use_local_storage(&cx);
    cx.render(rsx!(
        h2 {
            "Test for `use_local_storage`"
        }
        div {
            style : "white-space:pre",
            "Local storage: {hook:#?}"
        }
    ))
}
pub fn TestSessionStorage(cx: Scope) -> Element {
    let hook = use_session_storage(&cx);

    cx.render(rsx!(
        h2 {
            "Test for `use_session_storage`"
        }
        div {
            style : "white-space:pre",
            "Session storage: {hook:#?}"
        }
    ))
}
