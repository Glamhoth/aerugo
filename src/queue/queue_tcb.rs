//! TODO

use core::marker::PhantomData;

use crate::queue::QueueHandle;

pub struct QueueTcb<T> {
    _marker: PhantomData<T>
}

impl<T> QueueTcb<T> {
    /// TODO
    pub const fn new() -> Self {
        QueueTcb {
            _marker: PhantomData,
        }
    }

    /// TODO
    pub const fn create_handle(&self) -> QueueHandle<T> {
        QueueHandle::new()
    }
}
