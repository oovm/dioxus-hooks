mod display;

use super::*;
use dioxus::hooks::use_ref;
use log::info;
use web_sys::EventTarget;

///
#[derive(Debug, Clone)]
pub struct UseHovering {
    data: Rc<RefCell<UseHoveringData>>,
}

#[derive(Debug, Clone)]
struct UseHoveringData {
    hover: bool,
}

/// https://www.npmjs.com/package/@react-hook/mouse-position
impl UseHovering {
    pub(crate) fn new(cx: &ScopeState, target: &EventTarget) -> Self {
        let mut data = Rc::new(RefCell::new(UseHoveringData::default()));
    }
}

impl UseHovering {
    /// check if target is hovering
    #[inline]
    pub fn get(&self) -> bool {
        self.data.borrow().hover
    }
}
