//! TODO

use core::marker::PhantomData;

pub struct TaskletHandle<T> {
    _marker: PhantomData<T>,
}

impl<T> TaskletHandle<T> {
    pub const fn new() -> Self {
        TaskletHandle {
            _marker: PhantomData,
        }
    }
}
