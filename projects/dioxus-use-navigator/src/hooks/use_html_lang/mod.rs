use super::*;

struct UseHtmlLanguage {
    data: Rc<RefCell<UseHtmlLanguageData>>
}

pub struct UseHtmlLanguageData {
    lang: String,
    html: Element,
}

impl UseHtmlLanguageData {
    fn new() -> Option<Self> {
        let html = window()?.document()?.get_elements_by_tag_name("html").get_with_index(0)?;
        let lang = Self::get_lang(&html)?;
        Some(Self {
            lang,
            html
        })
    }
}

impl UseHtmlLanguage {
    pub(crate) fn new(cx:&ScopeState) -> Option<Self> {
        let data = Rc::new(RefCell::new(UseHtmlLanguageData::new()?));


        Some(Self {
            data
        })
    }
    fn get_lang(html: &Element) -> Option<String> {
        html.get_attribute("lang").or(html.get_attribute("xml:lang"))
    }
    fn set_lang(html: &Element) -> Option<String> {
        html.remove_attribute("xml:lang");
        html.set_attribute("lang", l)
    }
}