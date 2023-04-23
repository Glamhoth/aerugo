//! TODO

use crate::{InternalApi, SafeCell};
use core::marker::PhantomData;
use core::any::Any;

pub(crate) trait Queue {
    fn as_any(&self) -> &dyn Any;
}

/// TODO
#[derive(Debug)]
pub(crate) struct QueueHandle(pub *const dyn Queue);

/// TODO
unsafe impl Send for QueueHandle {}
/// TODO
unsafe impl Sync for QueueHandle {}

/// TODO
pub struct MessageQueue<T> {
    /// TODO
    internal_api: &'static (dyn InternalApi + Sync),
    /// TODO
    id: SafeCell<u32>,
    _marker: PhantomData<T>,
}

impl<T> MessageQueue<T> {
    /// TODO
    pub const fn new(internal_api: &'static (dyn InternalApi + Sync)) -> Self {
        let id = SafeCell::new(0);

        MessageQueue { internal_api, id, _marker: PhantomData }
    }

    /// TODO
    pub(crate) fn set_id(&self, id: u32) {
        *self.id.as_ref_mut() = id;
    }

    /// TODO
    pub fn send_data(&self) {
        self.internal_api.queue_notify(*self.id.as_ref());
    }
}

impl<T: 'static> Queue for MessageQueue<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
