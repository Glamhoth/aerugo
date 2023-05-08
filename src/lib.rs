/*!
# aerugo

`aerugo` is a safety-critical applications oriented Real-Time Operating System.
*/
#![no_std]

mod api;
mod event;
mod executor;
mod peripherals;
mod queue;
mod task;

pub use self::api::{InitApi, RuntimeApi};
pub use self::event::{EventHandle, EventTcb, EventType};
pub use self::executor::Executor;
pub use self::peripherals::Peripherals;
pub use self::queue::{QueueHandle, QueueTcb};
pub use self::task::{TaskletHandle, TaskletTcb};
