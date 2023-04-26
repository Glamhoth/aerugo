#![no_std]
#![no_main]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

extern crate panic_semihosting;

use cortex_m_rt::entry;
use cortex_m_semihosting::debug;

use aerugo::{Executor, TaskletTcb};

static EXECUTOR: Executor = Executor::new();

struct TaskDataA {
    a: u8,
    b: u8,
    c: u8,
}

struct TaskDataB {
    a: u32,
    b: u32,
    c: u32,
}

static task_a: TaskletTcb<TaskDataA> = TaskletTcb::new();
static task_b: TaskletTcb<TaskDataB> = TaskletTcb::new();

#[entry]
fn main() -> ! {
    EXECUTOR.create_tasklet("TaskletA", &task_a);
    EXECUTOR.create_tasklet("TaskletB", &task_b);

    cortex_m_semihosting::hprintln!("Done");

    debug::exit(debug::EXIT_SUCCESS);

    #[allow(clippy::empty_loop)]
    loop {}
}
