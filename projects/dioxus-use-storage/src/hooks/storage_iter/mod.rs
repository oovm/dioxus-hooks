mod storage_data;
use super::*;

pub struct UseStorageData {
    storage: Option<Storage>,
    // last_event: Option<StorageEvent>,
}

///
#[derive(Debug)]
pub struct StorageIter<'a> {
    inner: Option<Storage>,
    count: u32,
    index: u32,
    bound: PhantomData<&'a ()>,
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

impl<'a> StorageIter<'a> {
    /// c
    #[inline]
    pub fn new(storage: Option<Storage>) -> Self {
        let count = match &storage {
            None => 0,
            Some(s) => s.length().unwrap_or_default(),
        };
        Self { inner: storage, count, index: 0, bound: Default::default() }
    }
}

#[inline]
pub(crate) fn storage_eq(owned: &Option<Storage>, event: &Option<Storage>) -> bool {
    match (owned, event) {
        (Some(lhs), Some(rhs)) => lhs.eq(&rhs),
        _ => false,
    }
}

pub(crate) fn on_storage(cx: &ScopeState, window: &Window, data: &Rc<RefCell<UseStorageData>>) -> EventListener {
    #[cfg(debug_assertions)]
    {
        info!("Window Storage Listener Initialized at {}!", cx.scope_id().0);
    }
    let setter = data.clone();
    let regenerate = cx.schedule_update();
    EventListener::new(window, "storage", move |e| {
        let e: StorageEvent = e.clone().unchecked_into();
        let setter = setter.borrow_mut();
        if !storage_eq(&setter.storage, &e.storage_area()) {
            return;
        }
        // setter.last_event = Some(e);
        regenerate()
    })
}
