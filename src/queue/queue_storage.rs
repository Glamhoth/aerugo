//! TODO

use core::marker::PhantomData;

use crate::queue::QueueHandle;

/// TODO
pub struct QueueStorage<T> {
    _marker: PhantomData<T>,
}

impl<T> QueueStorage<T> {
    /// TODO
    pub const fn new() -> Self {
        QueueStorage {
            _marker: PhantomData,
        }
    }

    /// TODO
    pub const fn create_handle(&self) -> QueueHandle<T> {
        QueueHandle::new()
    }
}
