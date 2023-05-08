//! TODO

use core::marker::PhantomData;

use crate::event::{EventHandle, EventType};

pub struct EventTcb<T: EventType> {
    _marker: PhantomData<T>,
}

impl<T: EventType> EventTcb<T> {
    /// TODO
    pub const fn new() -> Self {
        EventTcb {
            _marker: PhantomData,
        }
    }

    /// TODO
    pub const fn create_handle(&self) -> EventHandle<T> {
        EventHandle::new()
    }
}
