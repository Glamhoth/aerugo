//! TODO

use crate::boolean_condition::BooleanConditionHandle;
use crate::event::{EventHandle, EventType};
use crate::queue::QueueHandle;

/// TODO
pub trait RuntimeApi: ErrorType {
    /// TODO
    fn send_data_to_queue<T>(
        &'static self,
        queue: &QueueHandle<T>,
        data: T,
    ) -> Result<(), Self::Error>;

    /// TODO
    fn emit_event<T: EventType>(&'static self, event: &EventHandle<T>) -> Result<(), Self::Error>;

    /// TODO
    fn emit_event_delayed<T: EventType>(
        &'static self,
        event: &EventHandle<T>,
        delay: f64,
    ) -> Result<(), Self::Error>;

    /// TODO
    fn cancel_event<T: EventType>(&'static self, event: &EventHandle<T>)
        -> Result<(), Self::Error>;

    /// TODO
    fn get_condition_value(
        &'static self,
        condition: &BooleanConditionHandle,
    ) -> Result<bool, Self::Error>;

    /// TODO
    fn set_condition_value(
        &'static self,
        condition: &BooleanConditionHandle,
        value: bool,
    ) -> Result<(), Self::Error>;
}

/// Runtime error
pub trait Error: core::fmt::Debug {}

/// Runtime error type trait
pub trait ErrorType {
    /// Error type
    type Error: Error;
}

impl<T: ErrorType> ErrorType for &mut T {
    type Error = T::Error;
}
