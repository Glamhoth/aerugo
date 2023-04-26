//! TODO

mod task;
mod task_id;
mod tasklet_tcb;

pub use self::task_id::TaskId;
pub use self::tasklet_tcb::TaskletTcb;

pub(crate) use self::task::{Task, TaskHandle};

use core::marker::PhantomData;

/// TODO
pub(crate) struct Tasklet<T> {
    name: &'static str,
    _marker: PhantomData<T>,
}

/// TODO
impl<T> Tasklet<T> {
    /// TODO
    pub const fn new(name: &'static str) -> Self {
        Tasklet {
            name,
            _marker: PhantomData,
        }
    }
}

impl<T> Task for Tasklet<T> {
    fn name(&self) -> &'static str {
        self.name
    }
}
