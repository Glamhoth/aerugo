//! TODO

use crate::{Event, MessageQueue, Tasklet, BooleanCondition};

/// API exposed by the RTOS used for the system initialization.
pub trait InitApi {
    /// TODO
    fn register_tasklet<T>(&self, tasklet: &'static Tasklet<T>) -> u32;
    /// TODO
    fn register_queue<T>(&self, queue: &'static MessageQueue<T>) -> u32;
    /// TODO
    fn register_event(&self, event: impl Event) -> u32;
    /// TODO
    fn register_condition(&self, event: &'static BooleanCondition) -> u32;

    /// TODO
    fn subscribe_task_to_queue(&self, tasklet_id: u32, queue_id: u32);
    /// TODO
    fn subscribe_task_to_event(&self, tasklet_id: u32, event_id: u32);
    /// TODO
    fn subscribe_task_to_condition(&self, tasklet_id: u32, condition_id: u32);
    /// TODO
    fn subscribe_task_to_cycling(&self, tasklet_id: u32, period: f32);
}
