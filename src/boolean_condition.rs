//! TODO

mod boolean_condition_handle;
mod boolean_condition_tcb;

pub use self::boolean_condition_handle::BooleanConditionHandle;
pub use self::boolean_condition_tcb::BooleanConditionTcb;

/// TODO
pub enum BooleanConditionSetType {
    AND,
    OR,
}

/// TODO
pub struct BooleanConditionSet {
    /// TODO
    #[allow(dead_code)]
    set_type: BooleanConditionSetType,
}

impl BooleanConditionSet {
    /// TODO
    pub const fn new(set_type: BooleanConditionSetType) -> Self {
        BooleanConditionSet { set_type }
    }

    pub fn add(&mut self, _handle: BooleanConditionHandle) {
        todo!();
    }
}
