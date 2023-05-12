//! TODO

use core::marker::PhantomData;

use crate::task::Task;

/// TODO
#[allow(dead_code)]
pub struct TaskHandle<T> {
    task: &'static dyn Task,
    _data_type_marker: PhantomData<T>,
}

impl<T> TaskHandle<T> {
    /// TODO
    pub(crate) const fn new(task: &'static dyn Task) -> Self {
        TaskHandle {
            task,
            _data_type_marker: PhantomData,
        }
    }
}
