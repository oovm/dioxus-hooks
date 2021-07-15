use super::*;

///
#[derive(Debug, Copy, Clone)]
pub struct UseCursorBuilder {}

pub(crate) struct UseCursorData {
    pub(crate) mouse: Option<MouseEvent>,
}
