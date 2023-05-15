//! TODO

use super::Task;

use core::marker::PhantomData;

use crate::aerugo::error::InitError;
use crate::api::InternalApi;
use crate::data_provider::DataProvider;
use crate::data_receiver::DataReceiver;
use crate::internal_cell::InternalCell;

/// TODO
#[allow(dead_code)]
pub(crate) struct Tasklet<T: 'static, C> {
    name: &'static str,
    data_provider: InternalCell<Option<&'static dyn DataProvider<T>>>,
    api: &'static dyn InternalApi,
    _data_type_marker: PhantomData<T>,
    _context_type_marker: PhantomData<C>,
}

impl<T, C> Tasklet<T, C> {
    /// TODO
    pub(crate) fn new(name: &'static str, api: &'static dyn InternalApi) -> Self {
        Tasklet {
            name,
            data_provider: InternalCell::new(None),
            api,
            _data_type_marker: PhantomData,
            _context_type_marker: PhantomData,
        }
    }
}

impl<T, C> Task for Tasklet<T, C> {
    fn get_name(&self) -> &'static str {
        self.name
    }
}

impl<T, C> DataReceiver<T> for Tasklet<T, C> {
    fn set_data_provider(
        &'static self,
        data_provider: &'static dyn DataProvider<T>,
    ) -> Result<(), InitError> {
        if unsafe { self.data_provider.as_ref().is_some() } {
            return Err(InitError::DataProviderAlreadySet);
        }

        unsafe { *self.data_provider.as_mut_ref() = Some(data_provider) };
        Ok(())
    }
}
