#![no_std]
#![no_main]
#![allow(non_upper_case_globals)]

extern crate panic_semihosting;

use cortex_m_rt::entry;
use cortex_m_semihosting::debug;

use aerugo::{Event, Executor, InitApi, MessageQueue, Tasklet};

static EXECUTOR: Executor = Executor::new();

static task_a: Tasklet<u8> = Tasklet::new(&EXECUTOR);
static task_b: Tasklet<u16> = Tasklet::new(&EXECUTOR);
static task_c: Tasklet<u32> = Tasklet::new(&EXECUTOR);

static queue: MessageQueue<u8> = MessageQueue::new(&EXECUTOR);

struct UartDataEvent;
impl Event for UartDataEvent {}

#[entry]
fn main() -> ! {
    let task_a_id = EXECUTOR.register_tasklet(&task_a);
    let task_b_id = EXECUTOR.register_tasklet(&task_b);
    let task_c_id = EXECUTOR.register_tasklet(&task_c);

    let queue_id = EXECUTOR.register_queue(&queue);

    let event_id = EXECUTOR.register_event(UartDataEvent);

    EXECUTOR.subscribe_task_to_queue(task_a_id, queue_id);
    EXECUTOR.subscribe_task_to_queue(task_b_id, queue_id);

    queue.send_data();

    cortex_m_semihosting::hprintln!("Done");

    debug::exit(debug::EXIT_SUCCESS);

    #[allow(clippy::empty_loop)]
    loop {}
}
