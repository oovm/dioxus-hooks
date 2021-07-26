use super::*;
use std::{
    borrow::{Borrow, BorrowMut},
    fmt::{Display, Formatter},
};
use web_sys::MutationObserver;

pub struct UseHtmlLanguage {
    data: Rc<RefCell<UseHtmlLanguageData>>,
    listen_html: MutationObserver,
}

struct UseHtmlLanguageData {
    html: Element,
}

impl Display for UseHtmlLanguage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.get(), f)
    }
}

impl UseHtmlLanguageData {
    fn get_lang(&self) -> Option<String> {
        self.html.get_attribute("lang").or_else(|| self.html.get_attribute("xml:lang"))
    }
    fn set_lang(&self, lang: &str) -> Option<()> {
        self.html.remove_attribute("xml:lang").ok();
        self.html.set_attribute("lang", lang).ok()
    }
}

impl UseHtmlLanguage {
    pub(crate) fn new(cx: &ScopeState) -> Option<Self> {
        let html = window()?.document()?.get_elements_by_tag_name("html").get_with_index(0)?;
        let mut data = UseHtmlLanguageData { html };
        // Make sure the attribute exists
        data.get_lang()?;
        let regenerate = cx.schedule_update();
        let listen_html = Self::listen(&data.html);
        let mut data = Rc::new(RefCell::new(data));
        // listen_html.observe()
        Some(Self { data, listen_html })
    }
    // https://stackoverflow.com/questions/59614459/using-mutationobserver-to-observe-attribute-change-for-a-specific-attribute-only
    fn listen(html: &Element) -> MutationObserver {
        let _ = html;
        // MutationObserver::new()
        todo!()
    }
}

impl UseHtmlLanguage {
    #[inline]
    pub fn get(&self) -> String {
        self.data.as_ref().borrow().get_lang().unwrap_or_default()
    }
    #[inline]
    pub fn set(&self, lang: &str) -> bool {
        self.data.as_ref().borrow().set_lang(lang).is_some()
    }
}
