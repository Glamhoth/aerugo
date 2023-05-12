//! TODO

mod task_handle;
mod tasklet;
mod tasklet_storage;

pub(crate) use self::tasklet::Tasklet;

pub use self::tasklet_storage::TaskletStorage;
pub use self::task_handle::TaskHandle;

/// TODO
pub(crate) trait Task {}
