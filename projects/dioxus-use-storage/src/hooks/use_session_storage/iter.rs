use super::*;

impl UseSessionStorage {
    ///
    pub fn iter(&self) -> StorageIter {
        StorageIter {
            inner: self.data.borrow().storage(),
            count: self.len() as _,
            index: 0,
            // value: self.get_index(0).unwrap_or_default(),
            bound: Default::default(),
        }
    }
}
