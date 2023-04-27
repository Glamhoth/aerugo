//! TODO

use core::marker::PhantomData;

pub struct QueueHandle<T> {
    _marker: PhantomData<T>,
}

impl<T> QueueHandle<T> {
    pub const fn new() -> Self {
        QueueHandle {
            _marker: PhantomData
        }
    }
}
