/*!
# aerugo

`aerugo` is a safety-critical applications oriented Real-Time Operating System.
*/
#![no_std]
#![allow(dead_code)]

mod api;
mod boolean_condition;
mod event;
mod executor;
mod message_queue;
mod safe_cell;
mod tasklet;

pub use api::{InitApi, InternalApi, RuntimeApi};
pub use boolean_condition::BooleanCondition;
pub use event::Event;
pub use executor::Executor;
pub use message_queue::MessageQueue;
pub use tasklet::Tasklet;

pub(crate) use message_queue::QueueHandle;
pub(crate) use safe_cell::SafeCell;
pub(crate) use tasklet::TaskHandle;
