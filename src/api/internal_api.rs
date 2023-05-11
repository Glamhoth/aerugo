//! TODO

use crate::notifier::Notifier;

/// TODO
pub(crate) trait InternalApi {
    /// TODO
    fn notify(&'static self, notifier: &'static dyn Notifier);
}
