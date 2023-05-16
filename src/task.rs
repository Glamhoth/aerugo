//! TODO

mod task_handle;
mod tasklet;
mod tasklet_storage;

pub(crate) use self::tasklet::Tasklet;

pub use self::task_handle::TaskHandle;
pub use self::tasklet_storage::TaskletStorage;

/// TODO
pub(crate) trait Task {
    /// TODO
    fn get_name(&self) -> &'static str;

    /// TODO
    fn is_ready(&self) -> bool;

    /// TODO
    fn execute(&self);
}

pub(crate) struct TaskPtr(pub *const dyn Task);

unsafe impl Sync for TaskPtr {}
unsafe impl Send for TaskPtr {}
