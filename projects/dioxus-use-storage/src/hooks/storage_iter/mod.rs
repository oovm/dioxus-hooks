use super::*;

///
#[derive(Debug)]
pub struct StorageIter<'a> {
    pub(crate) inner: Option<Storage>,
    pub(crate) count: u32,
    pub(crate) index: u32,
    pub(crate) bound: PhantomData<&'a ()>,
    // pub(crate) value: String,
}

impl<'a> Iterator for StorageIter<'a> {
    type Item = (String, String);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index + 1 > self.count {
            return None;
        }
        self.index += 1;
        let storage = self.inner.as_ref()?;
        let key = storage.key(self.index).ok()??;
        let value = storage.get_item(&key).ok()??;
        Some((key, value))
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let c = self.count as usize;
        (c, Some(c))
    }
    #[inline]
    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.count as usize
    }
}

pub(crate) fn storage_eq(owned: &Option<Storage>, event: &Option<Storage>) -> bool {
    match (owned, event) {
        (Some(lhs), Some(rhs)) => {
            lhs.eq(&rhs)
        }
        _ => false
    }
}