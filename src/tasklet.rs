//! TODO

use crate::{RuntimeApi, QueueHandle, MessageQueue};
use core::marker::PhantomData;

/// TODO
pub(crate) trait Task {
    fn execute(&self, queue: QueueHandle);
    fn type_name(&self) -> &'static str;
}

/// TODO
#[derive(Debug)]
pub(crate) struct TaskHandle(pub *const dyn Task);

/// TODO
unsafe impl Send for TaskHandle {}
/// TODO
unsafe impl Sync for TaskHandle {}

/// TODO
pub struct Tasklet<T> {
    /// TODO
    runtime_api: &'static (dyn RuntimeApi + Sync),
    _marker: PhantomData<T>,
}

/// TODO
impl<T> Tasklet<T> {
    /// TODO
    pub const fn new(runtime_api: &'static (dyn RuntimeApi + Sync)) -> Self {
        Tasklet { runtime_api, _marker: PhantomData }
    }
}

impl<T: 'static> Task for Tasklet<T> {
    fn execute(&self, queue: QueueHandle) {
        let _concrete_queue = unsafe { (*queue.0).as_any().downcast_ref::<MessageQueue<T>>() };
    }

    fn type_name(&self) -> &'static str {
        core::any::type_name::<T>()
    }
}
