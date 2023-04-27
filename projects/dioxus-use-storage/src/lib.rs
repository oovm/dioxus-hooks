#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![deny(missing_copy_implementations)]
#![doc = include_str!("../Readme.md")]
#![doc(html_logo_url = "https://avatars.githubusercontent.com/u/79236386")]
#![doc(html_favicon_url = "https://avatars.githubusercontent.com/u/79236386")]

mod hooks;

pub use crate::hooks::{builder::UseStorageBuilder, storage_iter::StorageIter, local_storage::UseLocalStorage, session_storage::UseSessionStorage, use_local_storage, use_session_storage};