/*!
# aerugo

`aerugo` is a safety-critical applications oriented Real-Time Operating System.
*/
#![no_std]
#![allow(dead_code)]
#![allow(unused_imports)]
#![feature(sync_unsafe_cell)]

mod api;
mod executor;
// mod safe_cell;
mod tasklet;

pub use self::api::InitApi;
pub use self::executor::Executor;
pub use self::tasklet::{TaskId, TaskletTcb};
