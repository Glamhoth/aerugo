#![no_std]
#![no_main]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

extern crate panic_semihosting;

use cortex_m_rt::entry;
use cortex_m_semihosting::debug;

use aerugo::{Executor, InitApi, QueueTcb, TaskletTcb};

static EXECUTOR: Executor = Executor::new();

struct TaskAData {
    a: u8,
    b: u8,
}

static task_a: TaskletTcb<u16, TaskAData> = TaskletTcb::new();

static queue_x: QueueTcb<u16> = QueueTcb::new();

#[entry]
fn main() -> ! {
    let task_a_handle = EXECUTOR.create_tasklet("TaskA", &task_a).unwrap();

    EXECUTOR.create_queue(&queue_x);
    let queue_x_handle = queue_x.create_handle();

    EXECUTOR.subscribe_tasklet_to_queue(&task_a_handle, &queue_x_handle);

    debug::exit(debug::EXIT_SUCCESS);

    #[allow(clippy::empty_loop)]
    loop {}
}

mod some_mod {
    use crate::{EXECUTOR, queue_x};
    use aerugo::{QueueHandle, RuntimeApi};
    use cortex_m_rt::exception;

    #[exception]
    fn SysTick() {
        static queue_x_handle: QueueHandle<u16> = queue_x.create_handle();

        EXECUTOR.send_data_to_queue(&queue_x_handle, 42).unwrap();
    }
}
