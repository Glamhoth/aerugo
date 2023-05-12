//! TODO

use crate::queue::MessageQueueStorage;
use crate::task::TaskletStorage;

/// TODO
pub trait InitApi: ErrorType {
    /// TODO
    fn create_tasklet<T, C>(
        &'static self,
        storage: &'static TaskletStorage<T, C>,
    ) -> Result<(), Self::Error>;

    /// TODO
    fn create_message_queue<T, const N: usize>(
        &'static self,
        storage: &'static MessageQueueStorage<T, N>,
    ) -> Result<(), Self::Error>;
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
