///! TODO
use super::Executor;

use crate::api::{init_api, runtime_api};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum InitError {}

impl init_api::Error for InitError {}

impl init_api::ErrorType for Executor {
    type Error = InitError;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum RuntimeError {}

impl runtime_api::Error for RuntimeError {}

impl runtime_api::ErrorType for Executor {
    type Error = RuntimeError;
}
