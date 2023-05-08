//! TODO

use crate::boolean_condition::BooleanConditionHandle;

pub struct BooleanConditionTcb {}

impl BooleanConditionTcb {
    /// TODO
    pub const fn new() -> Self {
        BooleanConditionTcb {}
    }

    /// TODO
    pub const fn create_handle(&self) -> BooleanConditionHandle {
        BooleanConditionHandle::new()
    }
}
