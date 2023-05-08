#![no_std]
#![no_main]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

extern crate panic_semihosting;

use cortex_m_rt::entry;
use cortex_m_semihosting::debug;

use aerugo::{EventTcb, EventType, Executor, InitApi, Peripherals, QueueTcb, TaskletTcb};

static EXECUTOR: Executor = Executor::new();

struct TaskAData {
    a: u8,
    b: u8,
}
static task_a: TaskletTcb<u16, TaskAData> = TaskletTcb::new();

static queue_x: QueueTcb<u16> = QueueTcb::new();

enum ImportantEvents {
    Event1,
    Event2,
}
impl EventType for ImportantEvents {}

static event_1: EventTcb<ImportantEvents> = EventTcb::new();

fn init_hardware(_peripherals: &Peripherals) {}

#[entry]
fn main() -> ! {
    let task_a_handle = EXECUTOR.create_tasklet("TaskA", &task_a).unwrap();
    let queue_x_handle = EXECUTOR.create_queue(&queue_x).unwrap();
    let event_1_handle = EXECUTOR
        .create_event(ImportantEvents::Event1, &event_1)
        .unwrap();

    EXECUTOR.subscribe_tasklet_to_queue(&task_a_handle, &queue_x_handle).unwrap();

    EXECUTOR.init_hardware(init_hardware);

    debug::exit(debug::EXIT_SUCCESS);
    EXECUTOR.start_scheduler()
}

mod some_mod {
    use crate::{queue_x, EXECUTOR};
    use aerugo::{QueueHandle, RuntimeApi};
    use cortex_m_rt::exception;

    #[exception]
    fn SysTick() {
        static queue_x_handle: QueueHandle<u16> = queue_x.create_handle();

        EXECUTOR.send_data_to_queue(&queue_x_handle, 42).unwrap();
    }
}
