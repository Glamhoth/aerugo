#![no_std]
#![no_main]
#![allow(unused_variables)]
#![allow(non_upper_case_globals)]

extern crate panic_semihosting;

use cortex_m_rt::entry;
use cortex_m_semihosting::debug;

use aerugo::{Aerugo, InitApi, MessageQueueStorage, TaskletStorage};

static AERUGO: Aerugo = Aerugo::new();

#[allow(dead_code)]
struct TaskAData {
    a: u8,
    b: u32,
}

#[allow(dead_code)]
struct TaskBData {
    a: u16,
    b: u16,
}

static tasklet_a: TaskletStorage<u8, TaskAData> = TaskletStorage::new();
static tasklet_b: TaskletStorage<u8, TaskBData> = TaskletStorage::new();
static queue_x: MessageQueueStorage<u8, 16> = MessageQueueStorage::new();

#[entry]
fn main() -> ! {
    AERUGO.create_tasklet("TASK_A", &tasklet_a).unwrap();
    let tasklet_a_handle = tasklet_a.create_task_handle().unwrap();

    AERUGO.create_tasklet("TASK_B", &tasklet_b).unwrap();
    let tasklet_b_handle = tasklet_b.create_task_handle().unwrap();

    AERUGO.create_message_queue(&queue_x).unwrap();
    let queue_x_handle = queue_x.create_queue_handle().unwrap();

    AERUGO
        .register_tasklet_to_queue(&tasklet_a_handle, &queue_x_handle)
        .unwrap();
    AERUGO
        .register_tasklet_to_queue(&tasklet_b_handle, &queue_x_handle)
        .unwrap();

    queue_x_handle.send_data(1).unwrap();

    debug::exit(debug::EXIT_SUCCESS);

    #[allow(clippy::empty_loop)]
    loop {}
}
