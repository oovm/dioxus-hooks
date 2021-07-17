use super::*;



pub struct UseDocument {
    data: Rc<RefCell<UseDocumentData>>
}

struct UseDocumentData {
    document: Document
}

impl UseDocument {
    pub fn new(cx: &ScopeState) -> Option<Self>{
        let document = window()?.document()?;
        let data = Rc::new(RefCell::new(UseDocumentData {
            document
        }));


        Some(Self {
            data
        })
    }
}

impl UseDocument {
    /// Title
    pub fn title(&self) -> String {
        self.data.borrow().document.title()
    }
}
