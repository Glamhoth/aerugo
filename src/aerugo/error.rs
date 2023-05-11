///! TODO
use super::Aerugo;

use crate::api::{init_api, runtime_api};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum InitError {
    AlreadyInitialized,
}

impl init_api::Error for InitError {}

impl init_api::ErrorType for Aerugo {
    type Error = InitError;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum RuntimeError {
    QueueFull,
}

impl runtime_api::Error for RuntimeError {}

impl runtime_api::ErrorType for Aerugo {
    type Error = RuntimeError;
}
