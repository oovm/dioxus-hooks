use super::*;

pub struct UseTitle {
    inner: UseDocument,
}

impl UseTitle {
    pub(crate) fn new(cx: &ScopeState) -> Option<Self> {
        Some(Self { inner: UseDocument::new(cx)? })
    }
}

impl UseTitle {
    #[inline]
    pub fn get(&self) -> String {
        self.inner.title()
    }
    #[inline]
    pub fn set(&self, title: &str) -> bool {
        self.inner.set_title(title).is_some()
    }
}
