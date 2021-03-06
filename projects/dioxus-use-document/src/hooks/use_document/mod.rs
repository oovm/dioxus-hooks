use super::*;
use web_sys::DocumentType;

pub struct UseDocument {
    data: Rc<RefCell<UseDocumentData>>,
}

struct UseDocumentData {
    document: Option<Document>,
}

impl UseDocument {
    pub fn new(cx: &ScopeState) -> Option<Self> {
        let document = window()?.document()?;
        let data = Rc::new(RefCell::new(UseDocumentData { document: Some(document) }));

        Some(Self { data })
    }
}

impl UseDocument {
    /// Title
    pub fn title(&self) -> String {
        let document = &self.data.borrow_mut().document;
        match document {
            None => String::new(),
            Some(e) => e.title(),
        }
    }
    ///
    pub fn set_title(&self, input: &str) -> Option<()> {
        let document = &self.data.borrow_mut().document;
        Some(document.as_ref()?.set_title(input))
    }
    /// **read-only**
    pub fn character_set(&self) -> String {
        let document = &self.data.borrow_mut().document;
        match document {
            None => String::from("utf-8"),
            Some(e) => e.character_set(),
        }
    }
    /// **read-only**
    pub fn doc_type(&self) -> Option<DocumentType> {
        let document = &self.data.borrow_mut().document;
        document.as_ref()?.doctype()
    }
}
