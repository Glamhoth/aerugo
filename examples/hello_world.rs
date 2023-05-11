#![no_std]
#![no_main]

#![allow(unused_variables)]
#![allow(non_upper_case_globals)]

extern crate panic_semihosting;

use cortex_m_rt::entry;
use cortex_m_semihosting::debug;

use aerugo::{Aerugo, InitApi, MessageQueueStorage};

static AERUGO: Aerugo = Aerugo::new();

static queue_x: MessageQueueStorage<u8, 16> = MessageQueueStorage::new();

#[entry]
fn main() -> ! {
    AERUGO.create_message_queue(&queue_x).unwrap();
    let queue_x_handle = queue_x.create_queue_handle().unwrap();

    queue_x_handle.send_data(1).unwrap();

    debug::exit(debug::EXIT_SUCCESS);

    #[allow(clippy::empty_loop)]
    loop {}
}
