//! TODO

use heapless::Vec;

use crate::aerugo::error::InitError;
use crate::api::InternalApi;
use crate::crit_cell::CritCell;
use crate::internal_cell::InternalCell;
use crate::queue::{MessageQueue, QueueHandle};

pub(crate) type QueueData<T, const N: usize> = heapless::spsc::Queue<T, N>;
pub(crate) type QueueBuffer = Vec<u8, { core::mem::size_of::<MessageQueue<(), 0>>() }>;

/// TODO
pub struct MessageQueueStorage<T, const N: usize> {
    initialized: InternalCell<bool>,
    queue_buffer: InternalCell<QueueBuffer>,
    queue_data: CritCell<QueueData<T, N>>,
}

impl<T, const N: usize> MessageQueueStorage<T, N> {
    /// TODO
    pub const fn new() -> Self {
        let queue_buffer = InternalCell::new(QueueBuffer::new());
        let queue_data = CritCell::new(QueueData::new());

        MessageQueueStorage {
            initialized: InternalCell::new(false),
            queue_buffer,
            queue_data,
        }
    }

    /// TODO
    pub(crate) fn init(&self, api: &'static dyn InternalApi) -> Result<(), InitError> {
        if unsafe { *self.initialized.as_ref() } {
            return Err(InitError::AlreadyInitialized);
        }

        let message_queue = MessageQueue::<T, N>::new(&self.queue_data, api);

        unsafe {
            let queue_buffer =
                self.queue_buffer.as_mut_ref().as_mut_ptr() as *mut MessageQueue<T, N>;
            core::ptr::write(queue_buffer, message_queue);
        }

        unsafe {
            *self.initialized.as_mut_ref() = true;
        }

        Ok(())
    }

    /// TODO
    pub fn create_queue_handle(&'static self) -> Option<QueueHandle<T>> {
        match self.queue() {
            Some(q) => Some(QueueHandle::new(q)),
            None => None,
        }
    }

    /// TODO
    fn queue(&self) -> Option<&'static MessageQueue<T, N>> {
        match unsafe { *self.initialized.as_ref() } {
            true => Some(unsafe {
                &*(self.queue_buffer.as_ref().as_ptr() as *const MessageQueue<T, N>)
            }),
            false => None,
        }
    }
}
