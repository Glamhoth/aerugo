//! TODO

use crate::tasklet::{Tasklet, TaskId};

use core::cell::SyncUnsafeCell;
use core::marker::PhantomData;
use heapless::Vec;

pub struct TaskletTcb<T> {
    id: SyncUnsafeCell<Option<TaskId>>,
    tcb: SyncUnsafeCell<Vec<u8, { core::mem::size_of::<Tasklet<()>>() }>>,
    _marker: PhantomData<T>
}

impl<T> TaskletTcb<T> {
    pub const fn new() -> Self {
        TaskletTcb {
            id: SyncUnsafeCell::new(None),
            tcb: SyncUnsafeCell::new(Vec::new()),
            _marker: PhantomData,
        }
    }

    pub(crate) unsafe fn init(&self, id: TaskId, tasklet: Tasklet<T>) {
        *self.id.get() = Some(id);

        let buf = (*self.tcb.get()).as_mut_ptr();
        unsafe { core::ptr::write(tcb.ptr() as *mut Tasklet<T>, tasklet) };
    }
}
