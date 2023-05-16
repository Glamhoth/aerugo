//! TODO

use core::fmt::Debug;

use crate::queue::{MessageQueueStorage, QueueHandle};
use crate::task::{TaskHandle, TaskletStorage};

/// TODO
pub trait InitApi: ErrorType {
    /// TODO
    fn create_tasklet<T: Debug, C>(
        &'static self,
        name: &'static str,
        storage: &'static TaskletStorage<T, C>,
    ) -> Result<(), Self::Error>;

    /// TODO
    fn create_message_queue<T: Debug, const N: usize>(
        &'static self,
        storage: &'static MessageQueueStorage<T, N>,
    ) -> Result<(), Self::Error>;

    /// TODO
    fn register_tasklet_to_queue<T: Debug>(
        &'static self,
        tasklet: &TaskHandle<T>,
        queue: &QueueHandle<T>,
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
