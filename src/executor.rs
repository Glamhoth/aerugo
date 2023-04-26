//! TODO

use crate::api::InitApi;
use crate::tasklet::{Task, TaskHandle, TaskId, Tasklet, TaskletTcb};

use core::cell::SyncUnsafeCell;
use heapless::FnvIndexMap;

type TaskletMap = FnvIndexMap<TaskId, TaskHandle, 32>;

/// TODO
pub struct Executor {
    /// TODO
    registered_tasklets: SyncUnsafeCell<TaskletMap>,
}

impl Executor {
    /// TODO
    pub const fn new() -> Self {
        let registered_tasklets = SyncUnsafeCell::new(TaskletMap::new());

        Executor {
            registered_tasklets,
        }
    }

    /// TODO
    pub fn create_tasklet<T: 'static>(&self, name: &'static str, tcb: &'static TaskletTcb<T>) {
        static mut next_id: u32 = 0;

        let id = TaskId(next_id);

        let tasklet = Tasklet::<T>::new(name);
        tcb.init(id, tasklet);

        unsafe {
            (*self.registered_tasklets.get())
                .insert(id, TaskHandle(tcb.ptr() as *const Tasklet<T>))
                .unwrap();
        }

        next_id += 1;
    }
}

impl InitApi for Executor {}
