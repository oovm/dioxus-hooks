use dioxus::core::ScopeState;
use gloo_events::EventListener;
use web_sys::window;
use web_sys::Clipboard;

/// effect handler
#[derive(Debug, Clone)]
pub struct ClipboardService {
    inner: Clipboard,
    // listeners: Vec<EventListener>
}

impl ClipboardService {
    /// do
    pub fn new(cx: &ScopeState) -> Option<Self> {
        let clipboard = window()?.navigator().clipboard()?;

        Some(Self {
            inner: clipboard,
        })
    }

    // #[inline]
    // pub fn listen(&mut self, listener: EventListener) {
    //     // EventListener::new()
    //     self.listeners.push(listener)
    // }

    //
    // pub fn read_text(&self) {
    //    self.inner.read_text();
    // }

}

