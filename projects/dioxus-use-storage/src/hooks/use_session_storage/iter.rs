use super::*;

impl UseSessionStorage {
    ///
    #[inline]
    pub fn iter(&self) -> StorageIter {
        StorageIter::new(self.data.clone().borrow().storage())
    }
}
