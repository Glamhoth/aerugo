//! TODO

use super::Queue;

use crate::aerugo::error::RuntimeError;
use crate::api::InternalApi;
use crate::crit_cell::CritCell;
use crate::notifier::Notifier;
use crate::queue::QueueData;

/// TODO
pub(crate) struct MessageQueue<'a, T, const N: usize> {
    data: &'a CritCell<QueueData<T, N>>,
    api: &'static dyn InternalApi,
}

impl<'a, T, const N: usize> MessageQueue<'a, T, N> {
    /// TODO
    pub(crate) fn new(
        data: &'a CritCell<heapless::spsc::Queue<T, N>>,
        api: &'static dyn InternalApi,
    ) -> Self {
        MessageQueue {
            data,
            api,
        }
    }
}

impl<'a, T, const N: usize> Queue<T> for MessageQueue<'a, T, N> {
    fn send_data(&'static self, data: T) -> Result<(), RuntimeError> {
        match self.data.lock(|q| q.enqueue(data)) {
            Ok(_) => (),
            Err(_) => return Err(RuntimeError::QueueFull),
        };

        self.api.notify(self);

        Ok(())
    }
}

impl<'a, T, const N: usize> Notifier for MessageQueue<'a, T, N> {}
