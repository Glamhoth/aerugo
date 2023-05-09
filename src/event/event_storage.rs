//! TODO

use core::marker::PhantomData;

use crate::event::{EventHandle, EventType};

/// TODO
pub struct EventStorage<T: EventType> {
    _marker: PhantomData<T>,
}

impl<T: EventType> EventStorage<T> {
    /// TODO
    pub const fn new() -> Self {
        EventStorage {
            _marker: PhantomData,
        }
    }

    /// TODO
    pub const fn create_handle(&self) -> EventHandle<T> {
        EventHandle::new()
    }
}
