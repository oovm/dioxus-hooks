use super::*;

pub struct UseCharacterSet {
    inner: UseDocument,
}

impl UseCharacterSet {
    pub(crate) fn new(cx: &ScopeState) -> Option<Self> {
        Some(Self { inner: UseDocument::new(cx)? })
    }
}

impl UseCharacterSet {
    #[inline]
    pub fn get(&self) -> String {
        self.inner.character_set()
    }
}
