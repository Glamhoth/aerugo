//! TODO

use core::marker::PhantomData;

use crate::task::TaskletHandle;

pub struct TaskletTcb<T, C> {
    _data_marker: PhantomData<T>,
    _context_marker: PhantomData<C>,
}

impl<T, C> TaskletTcb<T, C> {
    /// TODO
    pub const fn new() -> Self {
        TaskletTcb {
            _data_marker: PhantomData,
            _context_marker: PhantomData,
        }
    }

    /// TODO
    pub fn create_handle(&self) -> TaskletHandle<T> {
        TaskletHandle::new()
    }
}
