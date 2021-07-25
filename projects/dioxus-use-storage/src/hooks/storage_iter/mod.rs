use std::mem::swap;
use wasm_bindgen::JsValue;
use web_sys::Storage;

pub struct StorageIter<'a> {
    pub(crate) inner: &'a Option<Storage>,
    pub(crate) index: u32,
    pub(crate) value: String,
}

impl<'a> Iterator for StorageIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let new = self.inner?.key(self.index + 1).ok()?;
        let mut out = new.unwrap_or_default();
        self.index += 1;
        swap(&mut out, &mut self.value);
        Some(out)
    }
}