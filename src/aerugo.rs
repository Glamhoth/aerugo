//! TODO

pub mod error;

use core::fmt::Debug;

use crate::api::{InitApi, InternalApi, RuntimeApi};
use crate::executor::Executor;
use crate::notifier::Notifier;
use crate::queue::{MessageQueueStorage, QueueHandle};
use crate::task::{TaskHandle, TaskletStorage};

static EXECUTOR: Executor = Executor::new();

pub struct Aerugo {}

impl Aerugo {
    pub const fn new() -> Self {
        Aerugo {}
    }

    pub fn start_scheduler(&self) -> ! {
        EXECUTOR.run()
    }
}

impl InitApi for Aerugo {
    fn create_tasklet<T: Debug, C>(
        &'static self,
        name: &'static str,
        storage: &'static TaskletStorage<T, C>,
    ) -> Result<(), Self::Error> {
        storage.init(name, self)
    }

    fn create_message_queue<T: Debug, const N: usize>(
        &'static self,
        storage: &'static MessageQueueStorage<T, N>,
    ) -> Result<(), Self::Error> {
        storage.init(self)
    }

    fn register_tasklet_to_queue<T: Debug>(
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
        for &t in notifier.get_registered_tasklets() {
            EXECUTOR.enqueue_task(t);
        }
    }
}
