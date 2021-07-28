mod display;

use super::*;

pub struct UseDialog {
    window: Option<Window>,
}

impl UseDialog {
    pub(crate) fn new(_: &ScopeState) -> Self {
        let out = Self { window: window() };
        if out.window.is_none() {
            info!("`window` not found, `UseDialog` works as ssr mode.");
        }
        out
    }

    /// Calls the alert function.
    ///
    /// # Arguments
    ///
    /// * `message`:
    ///
    /// returns:
    /// - `None` if failed to run
    /// - `Some` if succeed
    #[inline]
    pub fn alert(&self, message: &str) -> Option<()> {
        self.window.as_ref()?.alert_with_message(message).ok()
    }

    /// Calls the confirm function.
    ///
    /// # Arguments
    ///
    /// * `message`:
    ///
    /// returns:
    /// - `None` if failed to run
    /// - `Some(bool)` if user have confirm
    #[inline]
    pub fn confirm(&self, message: &str) -> Option<bool> {
        self.window.as_ref()?.confirm_with_message(message).ok()
    }

    /// Calls the `prompt` function.
    ///
    /// A default value can be supplied which will be returned if the user doesn't input anything.
    /// This function will return `None` if the value of `default` is `None` and the user cancels
    /// the operation.
    ///
    /// # Arguments
    ///
    /// * `message`:
    ///
    /// returns:
    /// - `None` if failed to run
    /// - `Some(String)` if user have enter prompt
    #[inline]
    pub fn prompt(&self, message: &str, default: &str) -> Option<String> {
        let w = &self.window.as_ref()?;
        match default.is_empty() {
            true => w.prompt_with_message(message).ok()?,
            false => w.prompt_with_message_and_default(message, default).ok()?,
        }
    }
}
