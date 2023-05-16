//! TODO

use super::Queue;

use heapless::Vec;

use crate::aerugo::error::{InitError, RuntimeError};
use crate::api::InternalApi;
use crate::crit_cell::CritCell;
use crate::data_provider::DataProvider;
use crate::internal_cell::InternalCell;
use crate::notifier::Notifier;
use crate::queue::QueueData;
use crate::task::Task;

/// TODO
pub(crate) struct MessageQueue<'a, T, const N: usize> {
    data: &'a CritCell<QueueData<T, N>>,
    registered_tasklets: InternalCell<Vec<&'static dyn Task, 8>>,
    api: &'static dyn InternalApi,
}

impl<'a, T, const N: usize> MessageQueue<'a, T, N> {
    /// TODO
    pub(crate) fn new(
        data: &'a CritCell<heapless::spsc::Queue<T, N>>,
        api: &'static dyn InternalApi,
    ) -> Self {
        let registered_tasklets = InternalCell::new(Vec::new());

        MessageQueue {
            data,
            registered_tasklets,
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

impl<'a, T, const N: usize> Notifier for MessageQueue<'a, T, N> {
    fn register_tasklet(&'static self, task: &'static dyn Task) -> Result<(), InitError> {
        match unsafe { self.registered_tasklets.as_mut_ref().push(task) } {
            Ok(_) => Ok(()),
            Err(_) => return Err(InitError::TooManyTasklets),
        }
    }

    fn get_registered_tasklets(&'static self) -> &Vec<&'static dyn Task, 8> {
        unsafe { self.registered_tasklets.as_ref() }
    }
}

impl<'a, T, const N: usize> DataProvider<T> for MessageQueue<'a, T, N> {
    fn data_ready(&self) -> bool {
        self.data.lock(|q| !q.is_empty())
    }

    fn get_data(&self) -> T {
        self.data.lock(|q| q.dequeue().unwrap())
    }
}
