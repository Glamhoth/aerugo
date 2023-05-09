//! TODO

mod error;

use crate::api::{InitApi, RuntimeApi};
use crate::boolean_condition::{
    BooleanConditionHandle, BooleanConditionSet, BooleanConditionStorage,
};
use crate::event::{EventHandle, EventStorage, EventType};
use crate::peripherals::Peripherals;
use crate::queue::{QueueHandle, QueueStorage};
use crate::task::{TaskletHandle, TaskletStorage};

/// TODO
pub struct Executor {}

impl Executor {
    /// TODO
    pub const fn new() -> Self {
        Executor {}
    }

    /// TODO
    pub fn start_scheduler(&self) -> ! {
        self.init_system();

        #[allow(clippy::empty_loop)]
        loop {}
    }

    /// TODO
    fn init_system(&self) {
        todo!();
    }
}

impl InitApi for Executor {
    fn create_tasklet<T, C: 'static>(
        &'static self,
        _name: &'static str,
        _storage: &'static TaskletStorage<T, C>,
    ) -> Result<TaskletHandle<T>, Self::Error> {
        todo!();
    }

    fn create_queue<T>(
        &'static self,
        _storage: &'static QueueStorage<T>,
    ) -> Result<QueueHandle<T>, Self::Error> {
        todo!();
    }

    fn create_event<T: EventType>(
        &'static self,
        _event_type: T,
        _storage: &'static EventStorage<T>,
    ) -> Result<EventHandle<T>, Self::Error> {
        todo!();
    }

    fn create_boolean_condition(
        &'static self,
        _storage: &'static BooleanConditionStorage,
    ) -> Result<BooleanConditionHandle, Self::Error> {
        todo!();
    }

    fn subscribe_tasklet_to_queue<T>(
        &'static self,
        _tasklet: &TaskletHandle<T>,
        _queue: &QueueHandle<T>,
    ) -> Result<(), Self::Error> {
        todo!();
    }

    fn subscribe_tasklet_to_event<T: EventType>(
        &'static self,
        _tasklet: &TaskletHandle<T>,
        _event: &EventHandle<T>,
    ) -> Result<(), Self::Error> {
        todo!();
    }

    fn subscribe_tasklet_to_condition_set<T>(
        &'static self,
        _tasklet: &TaskletHandle<T>,
        _condition_set: BooleanConditionSet,
    ) -> Result<(), Self::Error> {
        todo!();
    }

    fn subscribe_tasklet_to_cycling_execution(
        &'static self,
        _tasklet: &TaskletHandle<()>,
        _period: f64,
    ) -> Result<(), Self::Error> {
        todo!();
    }

    fn init_hardware(&self, _init_fn: fn(&Peripherals)) {
        todo!();
    }
}

impl RuntimeApi for Executor {
    fn send_data_to_queue<T>(
        &'static self,
        _queue: &QueueHandle<T>,
        _data: T,
    ) -> Result<(), Self::Error> {
        todo!();
    }

    fn emit_event<T: EventType>(&'static self, _event: &EventHandle<T>) -> Result<(), Self::Error> {
        todo!();
    }
}
