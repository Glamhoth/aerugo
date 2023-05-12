//! TODO

use super::Task;

use core::marker::PhantomData;

use crate::api::InternalApi;

/// TODO
#[allow(dead_code)]
pub(crate) struct Tasklet<T, C> {
    api: &'static dyn InternalApi,
    _data_type_marker: PhantomData<T>,
    _context_type_marker: PhantomData<C>,
}

impl<T, C> Tasklet<T, C> {
    /// TODO
    pub(crate) fn new(api: &'static dyn InternalApi) -> Self {
        Tasklet {
            api,
            _data_type_marker: PhantomData,
            _context_type_marker: PhantomData,
        }
    }
}

impl<T, C> Task for Tasklet<T, C> {}
