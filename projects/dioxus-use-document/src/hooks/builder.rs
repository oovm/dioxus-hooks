use super::*;
use crate::hooks::use_document::UseDocument;

pub struct UseDocumentBuilder {}

impl Default for UseDocumentBuilder {
    fn default() -> Self {
        Self {}
    }
}

impl UseDocumentBuilder {
    pub fn use_document<'a>(&self, cx: &'a ScopeState) -> &'a mut UseDocument {
        let hook = UseDocument::new(cx).unwrap();
        cx.use_hook(|| hook)
    }
    pub fn use_title<'a>(&self, cx: &'a ScopeState) -> &'a mut UseTitle {
        let hook = UseTitle::new(cx).unwrap();
        cx.use_hook(|| hook)
    }
    pub fn use_charset<'a>(&self, cx: &'a ScopeState) -> &'a mut UseCharacterSet {
        let hook = UseCharacterSet::new(cx).unwrap();
        cx.use_hook(|| hook)
    }
}
