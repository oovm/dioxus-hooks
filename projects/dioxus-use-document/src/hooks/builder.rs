use crate::hooks::use_document::UseDocument;
use super::*;

pub struct UseDocumentBuilder {

}

impl Default for UseDocumentBuilder {
    fn default() -> Self {
        Self {

        }
    }
}




impl UseDocumentBuilder {
    pub fn use_document<'a>(& self, cx: &'a ScopeState) -> &'a mut UseDocument {
        let hook = UseDocument::new(cx).unwrap();
        cx.use_hook(|_| hook)
    }

    pub fn use_title<'a>(&self, cx: &'a ScopeState) -> &'a mut UseTitle {
        let hook = UseTitle::new(cx).unwrap();
        cx.use_hook(|_| hook)
    }
}