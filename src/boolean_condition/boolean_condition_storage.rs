//! TODO

use crate::boolean_condition::BooleanConditionHandle;

pub struct BooleanConditionStorage {}

impl BooleanConditionStorage {
    /// TODO
    pub const fn new() -> Self {
        BooleanConditionStorage {}
    }

    /// TODO
    pub const fn create_handle(&self) -> BooleanConditionHandle {
        BooleanConditionHandle::new()
    }
}
