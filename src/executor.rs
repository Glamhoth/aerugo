//! TODO

use crate::{
    BooleanCondition, Event, InitApi, InternalApi, MessageQueue, QueueHandle, RuntimeApi, SafeCell,
    TaskHandle, Tasklet,
};

use heapless::{Entry, FnvIndexMap, Vec};

type TaskletMap = FnvIndexMap<u32, TaskHandle, 32>;
type QueueMap = FnvIndexMap<u32, QueueHandle, 32>;
type QueueTaskletMap = FnvIndexMap<u32, Vec<u32, 32>, 32>;

/// TODO
pub struct Executor {
    /// TODO
    registered_tasklets: SafeCell<TaskletMap>,
    /// TODO
    registered_queues: SafeCell<QueueMap>,
    /// TODO
    queues_to_tasklets: SafeCell<QueueTaskletMap>,
}

impl Executor {
    /// TODO
    pub const fn new() -> Self {
        let registered_tasklets = SafeCell::new(TaskletMap::new());
        let registered_queues = SafeCell::new(QueueMap::new());
        let queues_to_tasklets = SafeCell::new(QueueTaskletMap::new());

        Executor {
            registered_tasklets,
            registered_queues,
            queues_to_tasklets,
        }
    }
}

impl InitApi for Executor {
    fn register_tasklet<T>(&self, tasklet: &'static Tasklet<T>) -> u32 {
        let id = self.registered_tasklets.as_ref().len() as u32;

        self.registered_tasklets
            .as_ref_mut()
            .insert(id, TaskHandle(tasklet))
            .unwrap();

        id
    }

    fn register_queue<T>(&self, queue: &'static MessageQueue<T>) -> u32 {
        let id = self.registered_queues.as_ref().len() as u32;

        queue.set_id(id);

        self.registered_queues
            .as_ref_mut()
            .insert(id, QueueHandle(queue))
            .unwrap();

        id
    }

    fn register_event(&self, _event: impl Event) -> u32 {
        0
    }

    fn register_condition(&self, _condition: &'static BooleanCondition) -> u32 {
        0
    }

    fn subscribe_task_to_queue(&self, tasklet_id: u32, queue_id: u32) {
        let map = self.queues_to_tasklets.as_ref_mut();

        let tasklet = self.registered_tasklets.as_ref().get(&tasklet_id).unwrap();
        cortex_m_semihosting::hprintln!("{}", unsafe { (*tasklet.0).type_name() });

        match map.entry(queue_id) {
            Entry::Vacant(v) => {
                let mut tasklets = Vec::new();
                tasklets.push(tasklet_id).unwrap();

                v.insert(tasklets).unwrap();
            }
            Entry::Occupied(mut o) => {
                (*o.get_mut()).push(tasklet_id).unwrap();
            }
        }
    }

    fn subscribe_task_to_event(&self, _tasklet_id: u32, _event_id: u32) {}

    fn subscribe_task_to_condition(&self, _tasklet_id: u32, _condition_id: u32) {}

    fn subscribe_task_to_cycling(&self, _tasklet_id: u32, _period: f32) {}
}

impl InternalApi for Executor {
    fn queue_notify(&self, queue_id: u32) {
        let map = self.queues_to_tasklets.as_ref();

        match map.get(&queue_id) {
            Some(tasklets) => {
                for t in tasklets {
                    cortex_m_semihosting::hprintln!("{}", t);
                }
            }
            None => (),
        }
    }
}

impl RuntimeApi for Executor {}
