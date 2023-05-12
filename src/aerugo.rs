//! TODO

pub mod error;

use crate::api::{InitApi, InternalApi, RuntimeApi};
use crate::notifier::Notifier;
use crate::queue::MessageQueueStorage;
use crate::task::TaskletStorage;

pub struct Aerugo {}

impl Aerugo {
    pub const fn new() -> Self {
        Aerugo {}
    }
}

impl InitApi for Aerugo {
    fn create_tasklet<T, C>(
        &'static self,
        storage: &'static TaskletStorage<T, C>,
    ) -> Result<(), Self::Error> {
        storage.init(self)
    }

    fn create_message_queue<T, const N: usize>(
        &'static self,
        storage: &'static MessageQueueStorage<T, N>,
    ) -> Result<(), Self::Error> {
        storage.init(self)
    }
}

impl RuntimeApi for Aerugo {}

impl InternalApi for Aerugo {
    fn notify(&'static self, _notifier: &'static dyn Notifier) {
        cortex_m_semihosting::hprintln!("New data in queue");
    }
}
