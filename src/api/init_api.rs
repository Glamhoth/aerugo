//! TODO

use crate::boolean_condition::{BooleanConditionHandle, BooleanConditionSet, BooleanConditionTcb};
use crate::event::{EventHandle, EventTcb, EventType};
use crate::peripherals::Peripherals;
use crate::queue::{QueueHandle, QueueTcb};
use crate::task::{TaskletHandle, TaskletTcb};

/// TODO
pub trait InitApi: ErrorType {
    /// TODO
    fn create_tasklet<T, C: 'static>(
        &'static self,
        name: &'static str,
        tcb: &'static TaskletTcb<T, C>,
    ) -> Result<TaskletHandle<T>, Self::Error>;

    /// TODO
    fn create_queue<T>(
        &'static self,
        tcb: &'static QueueTcb<T>,
    ) -> Result<QueueHandle<T>, Self::Error>;

    /// TODO
    fn create_event<T: EventType>(
        &'static self,
        event_type: T,
        tcb: &'static EventTcb<T>,
    ) -> Result<EventHandle<T>, Self::Error>;

    /// TODO
    fn create_boolean_condition(
        &'static self,
        tcb: &'static BooleanConditionTcb,
    ) -> Result<BooleanConditionHandle, Self::Error>;

    /// TODO
    fn subscribe_tasklet_to_queue<T>(
        &'static self,
        tasklet: &TaskletHandle<T>,
        queue: &QueueHandle<T>,
    ) -> Result<(), Self::Error>;

    /// TODO
    fn subscribe_tasklet_to_event<T: EventType>(
        &'static self,
        tasklet: &TaskletHandle<T>,
        event: &EventHandle<T>,
    ) -> Result<(), Self::Error>;

    /// TODO
    fn subscribe_tasklet_to_condition_set<T>(
        &'static self,
        tasklet: &TaskletHandle<T>,
        condition_set: BooleanConditionSet,
    ) -> Result<(), Self::Error>;

    /// TODO
    fn subscribe_tasklet_to_cycling_execution(
        &'static self,
        tasklet: &TaskletHandle<()>,
        period: f64,
    ) -> Result<(), Self::Error>;

    /// TODO
    fn init_hardware(&self, init_fn: fn(&Peripherals));
}

/// Initialization error
pub trait Error: core::fmt::Debug {}

/// Initialization error type trait
pub trait ErrorType {
    /// Error type
    type Error: Error;
}

impl<T: ErrorType> ErrorType for &mut T {
    type Error = T::Error;
}
