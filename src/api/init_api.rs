//! TODO

use crate::boolean_condition::{
    BooleanConditionHandle, BooleanConditionSet, BooleanConditionStorage,
};
use crate::event::{EventHandle, EventStorage, EventType};
use crate::peripherals::Peripherals;
use crate::queue::{QueueHandle, QueueStorage};
use crate::task::{TaskletHandle, TaskletStorage};

/// TODO
pub trait InitApi: ErrorType {
    /// TODO
    fn create_tasklet<T, C: 'static>(
        &'static self,
        name: &'static str,
        storage: &'static TaskletStorage<T, C>,
    ) -> Result<TaskletHandle<T>, Self::Error>;

    /// TODO
    fn create_queue<T>(
        &'static self,
        storage: &'static QueueStorage<T>,
    ) -> Result<QueueHandle<T>, Self::Error>;

    /// TODO
    fn create_event<T: EventType>(
        &'static self,
        event_type: T,
        storage: &'static EventStorage<T>,
    ) -> Result<EventHandle<T>, Self::Error>;

    /// TODO
    fn create_boolean_condition(
        &'static self,
        storage: &'static BooleanConditionStorage,
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
