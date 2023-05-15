//! TODO

use crate::aerugo::error::RuntimeError;
use crate::data_provider::DataProvider;
use crate::notifier::Notifier;
use crate::queue::{MessageQueue, Queue};

/// TODO
#[derive(Copy, Clone)]
pub struct QueueHandle<T: 'static> {
    queue: &'static dyn Queue<T>,
    notifier: &'static dyn Notifier,
    data_provider: &'static dyn DataProvider<T>,
}

impl<T> QueueHandle<T> {
    /// TODO
    pub(crate) const fn new<const N: usize>(message_queue: &'static MessageQueue<T, N>) -> Self {
        QueueHandle {
            queue: message_queue,
            notifier: message_queue,
            data_provider: message_queue,
        }
    }

    /// TODO
    pub(crate) fn as_notifier(&self) -> &'static dyn Notifier {
        self.notifier
    }

    /// TODO
    pub(crate) fn as_data_provider(&self) -> &'static dyn DataProvider<T> {
        self.data_provider
    }

    /// TODO
    #[inline(always)]
    pub fn send_data(&self, data: T) -> Result<(), RuntimeError> {
        self.queue.send_data(data)
    }
}
