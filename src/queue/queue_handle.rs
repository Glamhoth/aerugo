//! TODO

use core::marker::PhantomData;

/// TODO
pub struct QueueHandle<T> {
    _marker: PhantomData<T>,
}

impl<T> QueueHandle<T> {
    /// TODO
    pub(crate) const fn new() -> Self {
        QueueHandle {
            _marker: PhantomData,
        }
    }
}
