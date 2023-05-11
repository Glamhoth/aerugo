//! TODO

use crate::aerugo::error::RuntimeError;
use crate::queue::Queue;

/// TODO
pub struct QueueHandle<T: 'static> {
    queue: &'static dyn Queue<T>,
}

impl<T> QueueHandle<T> {
    /// TODO
    pub(crate) const fn new(queue: &'static dyn Queue<T>) -> Self {
        QueueHandle { queue }
    }

    #[inline(always)]
    pub fn send_data(&self, data: T) -> Result<(), RuntimeError> {
        self.queue.send_data(data)
    }
}
