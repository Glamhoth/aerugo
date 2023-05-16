//! TODO

use core::fmt::Debug;

use crate::data_receiver::DataReceiver;
use crate::task::{Task, Tasklet};

/// TODO
#[allow(dead_code)]
pub struct TaskHandle<T: 'static + Debug> {
    task: &'static dyn Task,
    data_receiver: &'static dyn DataReceiver<T>,
}

impl<T: Debug> TaskHandle<T> {
    /// TODO
    pub(crate) const fn new<C>(tasklet: &'static Tasklet<T, C>) -> Self {
        TaskHandle {
            task: tasklet,
            data_receiver: tasklet,
        }
    }

    /// TODO
    pub(crate) fn as_task(&self) -> &'static dyn Task {
        self.task
    }

    /// TODO
    pub(crate) fn as_data_receiver(&self) -> &'static dyn DataReceiver<T> {
        self.data_receiver
    }
}
