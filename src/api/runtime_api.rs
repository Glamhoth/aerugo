//! TODO

use crate::queue::QueueHandle;

/// TODO
pub trait RuntimeApi: ErrorType {
    /// TODO
    fn send_data_to_queue<T>(&'static self, queue: &QueueHandle<T>, data: T) -> Result<(), Self::Error>;
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
