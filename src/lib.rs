/*!
# aerugo

`aerugo` is a safety-critical applications oriented Real-Time Operating System.
*/
#![no_std]

mod api;
mod boolean_condition;
mod event;
mod executor;
mod peripherals;
mod queue;
mod task;

pub use self::api::{InitApi, RuntimeApi};
pub use self::boolean_condition::{
    BooleanConditionHandle, BooleanConditionSet, BooleanConditionSetType, BooleanConditionStorage,
};
pub use self::event::{EventHandle, EventStorage, EventType};
pub use self::executor::{Executor, TaskletConfiguration};
pub use self::peripherals::Peripherals;
pub use self::queue::{QueueHandle, QueueStorage};
pub use self::task::{TaskletHandle, TaskletStorage};
