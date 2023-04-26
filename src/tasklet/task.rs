//! TODO

use crate::tasklet::TaskId;

/// TODO
pub(crate) trait Task {
    /// TODO
    fn name(&self) -> &'static str;
}

/// TODO
#[derive(Debug)]
pub(crate) struct TaskHandle(pub *const dyn Task);

/// TODO
unsafe impl Send for TaskHandle {}
/// TODO
unsafe impl Sync for TaskHandle {}
