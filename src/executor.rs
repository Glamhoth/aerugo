//! TODO

mod error;

use crate::api::{InitApi, RuntimeApi};
use crate::queue::{QueueHandle, QueueTcb};
use crate::task::{TaskletHandle, TaskletTcb};

/// TODO
pub struct Executor {}

impl Executor {
    /// TODO
    pub const fn new() -> Self {
        Executor {}
    }
}

impl InitApi for Executor {
    fn create_tasklet<T, C: 'static>(
        &'static self,
        _name: &'static str,
        _tcb: &'static TaskletTcb<T, C>,
    ) -> Result<TaskletHandle<T>, Self::Error> {
        todo!();
    }

    fn create_queue<T>(&'static self, _tcb: &'static QueueTcb<T>) {
        todo!();
    }

    fn subscribe_tasklet_to_queue<T>(
        &'static self,
        _tasklet: &TaskletHandle<T>,
        _queue: &QueueHandle<T>,
    ) {
        todo!();
    }
}

impl RuntimeApi for Executor {
    fn send_data_to_queue<T>(
        &'static self,
        _queue: &QueueHandle<T>,
        _data: T,
    ) -> Result<(), Self::Error> {
        todo!();
    }
}
