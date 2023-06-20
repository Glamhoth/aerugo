//! Unit of computation in the system.
//!
//! Tasklet is a fine-grained units of computation, that execute a processing step in a finite
//! amount of time.
//!
//! Tasklet should be thought of as a small building block, which processes a given type of data,
//! one element at the time.
mod tasklet_config;
mod tasklet_handle;
mod tasklet_ptr;
mod tasklet_storage;
mod tasklet_vtable;

pub(crate) use self::tasklet_ptr::TaskletPtr;
pub(crate) use self::tasklet_vtable::{tasklet_vtable, TaskletVTable};

pub use self::tasklet_config::TaskletConfig;
pub use self::tasklet_handle::TaskletHandle;
pub use self::tasklet_storage::TaskletStorage;

use core::marker::PhantomData;

use crate::aerugo::error::InitError;
use crate::arch::{logln, Mutex};
use crate::data_provider::DataProvider;
use crate::data_receiver::DataReceiver;
use crate::internal_cell::InternalCell;
use crate::task::{Task, TaskStatus};

/// Tasklet structure.
///
/// * `T` - Type that is processed by the tasklet.
/// * `C` - Type of tasklet context data.
pub(crate) struct Tasklet<T: 'static, C> {
    /// Tasklet name.
    name: &'static str,
    /// Tasklet status
    status: Mutex<TaskStatus>,
    /// Source of the data.
    _data_provider: InternalCell<Option<&'static dyn DataProvider<T>>>,
    /// Marker for tasklet context data type.
    _context_type_marker: PhantomData<C>,
}

impl<T, C> Tasklet<T, C> {
    /// Creates new `Tasklet`.
    pub(crate) const fn new(config: TaskletConfig) -> Self {
        Tasklet {
            name: config.name,
            status: Mutex::new(TaskStatus::Sleeping),
            _data_provider: InternalCell::new(None),
            _context_type_marker: PhantomData,
        }
    }
}

impl<T, C> Task for Tasklet<T, C> {
    fn get_name(&self) -> &'static str {
        self.name
    }

    fn get_status(&self) -> TaskStatus {
        self.status.lock(|s| *s)
    }

    fn set_status(&self, status: TaskStatus) {
        self.status.lock(|s| *s = status)
    }

    fn has_work(&self) -> bool {
        // TODO: Stub until we have working data providers.
        true
    }

    fn execute(&self) {
        logln!("Executing {}", self.name);
    }
}

impl<T, C> DataReceiver<T> for Tasklet<T, C> {
    fn subscribe(
        &'static self,
        _data_provider: &'static dyn DataProvider<T>,
    ) -> Result<(), InitError> {
        todo!()
    }
}
