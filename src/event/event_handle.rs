//! TODO

use core::marker::PhantomData;

use crate::event::EventType;

/// TODO
pub struct EventHandle<T: EventType> {
    _marker: PhantomData<T>,
}

impl<T: EventType> EventHandle<T> {
    /// TODO
    pub(crate) const fn new() -> Self {
        EventHandle {
            _marker: PhantomData,
        }
    }
}
