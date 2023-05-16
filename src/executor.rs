//! TODO

use crate::crit_cell::CritCell;
use crate::task::{Task, TaskPtr};

/// TODO
pub(crate) struct Executor {
    queued_tasklets: CritCell<heapless::spsc::Queue<TaskPtr, 16>>,
}

impl Executor {
    pub(crate) const fn new() -> Self {
        let queued_tasklets = CritCell::new(heapless::spsc::Queue::new());

        Executor { queued_tasklets }
    }

    pub(crate) fn enqueue_task(&self, task: &'static dyn Task) {
        self.queued_tasklets
            .lock(|q| match q.enqueue(TaskPtr(task)) {
                Ok(_) => (),
                Err(_) => panic!("Executor queue full"),
            })
    }

    pub(crate) fn run(&self) -> ! {
        loop {
            if self.queued_tasklets.lock(|q| !q.is_empty()) {
                let task = self.queued_tasklets.lock(|q| q.dequeue()).unwrap();
                unsafe {
                    if (*task.0).is_ready() {
                        (*task.0).execute();
                    }
                }
            }
        }
    }
}
