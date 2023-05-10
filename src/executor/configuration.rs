//! TODO
use super::Executor;

use crate::api::init_api;

#[allow(dead_code)]
pub struct TaskletConfiguration {
    pub name: &'static str,
    pub priority: u32,
}

impl init_api::TaskConfiguration for TaskletConfiguration {}

impl Default for TaskletConfiguration {
    fn default() -> Self {
        TaskletConfiguration {
            name: "NO_NAME",
            priority: 0,
        }
    }
}

impl init_api::TaskConfigType for Executor {
    type TaskConfig = TaskletConfiguration;
}
