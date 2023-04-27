/*!
# aerugo

`aerugo` is a safety-critical applications oriented Real-Time Operating System.
*/
#![no_std]

mod api;
mod executor;
mod task;
mod queue;

pub use self::api::{InitApi, RuntimeApi};
pub use self::executor::Executor;
pub use self::task::{TaskletHandle, TaskletTcb};
pub use self::queue::{QueueHandle, QueueTcb};
