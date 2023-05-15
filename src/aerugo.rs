//! TODO

pub mod error;

use crate::api::{InitApi, InternalApi, RuntimeApi};
use crate::notifier::Notifier;
use crate::queue::{MessageQueueStorage, QueueHandle};
use crate::task::{TaskHandle, TaskletStorage};

pub struct Aerugo {}

impl Aerugo {
    pub const fn new() -> Self {
        Aerugo {}
    }
}

impl InitApi for Aerugo {
    fn create_tasklet<T, C>(
        &'static self,
        name: &'static str,
        storage: &'static TaskletStorage<T, C>,
    ) -> Result<(), Self::Error> {
        storage.init(name, self)
    }

    fn create_message_queue<T, const N: usize>(
        &'static self,
        storage: &'static MessageQueueStorage<T, N>,
    ) -> Result<(), Self::Error> {
        storage.init(self)
    }

    fn register_tasklet_to_queue<T>(
        &'static self,
        tasklet: &TaskHandle<T>,
        queue: &QueueHandle<T>,
    ) -> Result<(), Self::Error> {
        queue.as_notifier().register_tasklet(tasklet.as_task())?;
        tasklet
            .as_data_receiver()
            .set_data_provider(queue.as_data_provider())?;

        Ok(())
    }
}

impl RuntimeApi for Aerugo {}

impl InternalApi for Aerugo {
    fn notify(&'static self, notifier: &'static dyn Notifier) {
        let tasklet_count = notifier.get_registered_tasklets().len();

        for t in notifier.get_registered_tasklets() {
            cortex_m_semihosting::hprintln!("{}", t.get_name());
        }

        cortex_m_semihosting::hprintln!("New data in queue; {}", tasklet_count);
    }
}
