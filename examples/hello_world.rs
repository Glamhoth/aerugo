#![no_std]
#![no_main]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

extern crate panic_semihosting;

use cortex_m_rt::entry;
use cortex_m_semihosting::debug;

use aerugo::{
    BooleanConditionSet, BooleanConditionSetType, BooleanConditionTcb, EventTcb, EventType,
    Executor, InitApi, Peripherals, QueueTcb, TaskletTcb,
};

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

static task_b: TaskletTcb<ImportantEvents, ()> = TaskletTcb::new();

static event_1: EventTcb<ImportantEvents> = EventTcb::new();

static condition_u: BooleanConditionTcb = BooleanConditionTcb::new();
static condition_v: BooleanConditionTcb = BooleanConditionTcb::new();

static task_c: TaskletTcb<(), ()> = TaskletTcb::new();

fn init_hardware(_peripherals: &Peripherals) {}

#[entry]
fn main() -> ! {
    let task_a_handle = EXECUTOR.create_tasklet("TaskA", &task_a).unwrap();
    let queue_x_handle = EXECUTOR.create_queue(&queue_x).unwrap();

    EXECUTOR
        .subscribe_tasklet_to_queue(&task_a_handle, &queue_x_handle)
        .unwrap();

    let task_b_handle = EXECUTOR.create_tasklet("TaskB", &task_b).unwrap();
    let event_1_handle = EXECUTOR
        .create_event(ImportantEvents::Event1, &event_1)
        .unwrap();

    EXECUTOR
        .subscribe_tasklet_to_event(&task_b_handle, &event_1_handle)
        .unwrap();

    let condition_u_handle = EXECUTOR.create_boolean_condition(&condition_u).unwrap();
    let condition_v_handle = EXECUTOR.create_boolean_condition(&condition_v).unwrap();

    let mut condition_set = BooleanConditionSet::new(BooleanConditionSetType::AND);
    condition_set.add(condition_u_handle);
    condition_set.add(condition_v_handle);

    EXECUTOR
        .subscribe_tasklet_to_condition_set(&task_a_handle, condition_set)
        .unwrap();

    let task_c_handle = EXECUTOR.create_tasklet("TaskC", &task_c).unwrap();

    EXECUTOR.subscribe_tasklet_to_cycling_execution(&task_c_handle, 60.0).unwrap();

    EXECUTOR.init_hardware(init_hardware);

    debug::exit(debug::EXIT_SUCCESS);
    EXECUTOR.start_scheduler()
}

mod some_mod {
    use crate::{event_1, queue_x, ImportantEvents, EXECUTOR};
    use aerugo::{EventHandle, QueueHandle, RuntimeApi};
    use cortex_m_rt::exception;

    #[exception]
    fn SysTick() {
        static queue_x_handle: QueueHandle<u16> = queue_x.create_handle();
        static event_1_handle: EventHandle<ImportantEvents> = event_1.create_handle();

        EXECUTOR.send_data_to_queue(&queue_x_handle, 42).unwrap();
        EXECUTOR.emit_event(&event_1_handle).unwrap();
    }
}
