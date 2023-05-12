//! TODO

use core::marker::PhantomData;
use heapless::Vec;

use crate::aerugo::error::InitError;
use crate::api::InternalApi;
use crate::internal_cell::InternalCell;
use crate::task::{Tasklet, TaskHandle};

/// TODO
pub(crate) type TaskletBuffer = Vec<u8, { core::mem::size_of::<Tasklet<(), ()>>() }>;

/// TODO
#[allow(dead_code)]
pub struct TaskletStorage<T, C> {
    /// Marks whether this storage is initialized.
    ///
    /// SAFETY: This is only modified in the [`Self::init()`].
    initialized: InternalCell<bool>,
    /// Buffer for the tasklet strucure.
    ///
    /// SAFETY: This is only modified in the [`Self::init()`].
    tasklet_buffer: InternalCell<TaskletBuffer>,
    /// Marker for the tasklet data type.
    _data_type_marker: PhantomData<T>,
    /// Marker for the tasklet internal context type.
    _context_type_marker: PhantomData<C>,
}

impl<T, C> TaskletStorage<T, C> {
    /// TODO
    pub const fn new() -> Self {
        let tasklet_buffer = InternalCell::new(TaskletBuffer::new());

        TaskletStorage {
            initialized: InternalCell::new(false),
            tasklet_buffer,
            _data_type_marker: PhantomData,
            _context_type_marker: PhantomData,
        }
    }

    /// TODO
    pub(crate) fn init(&self, api: &'static dyn InternalApi) -> Result<(), InitError> {
        // SAFETY: This is safe, because it can't be borrowed externally and is only modified in
        // this function.
        if unsafe { *self.initialized.as_ref() } {
            return Err(InitError::AlreadyInitialized);
        }

        let tasklet = Tasklet::<T, C>::new(api);

        // SAFETY: This is safe, because it is borrowed mutably only here. It can be
        // modified (initialized) only once, because it is guarded by the `initialized` field. No
        // external borrow can be made before initialization can be made.
        //
        // Because the `Tasklet` structure is of a constant size, the `tasklet_buffer` field
        // is statically allocated with enough memory for a 'placement new'.
        unsafe {
            let tasklet_buffer = self.tasklet_buffer.as_mut_ref().as_mut_ptr() as *mut Tasklet<T, C>;
            core::ptr::write(tasklet_buffer, tasklet);
        }

        // SAFETY: This is safe, because it is only modified only here, and can't be externally
        // borrowed.
        unsafe {
            *self.initialized.as_mut_ref() = true;
        }

        Ok(())
    }

    /// TODO
    pub fn create_task_handle(&'static self) -> Option<TaskHandle<T>> {
        match self.tasklet() {
            Some(t) => Some(TaskHandle::new(t)),
            None => None,
        }
    }

    /// TODO
    fn tasklet(&self) -> Option<&'static Tasklet<T, C>> {
        // SAFETY: This is safe, because it can't be borrowed externally and is only modified in
        // the `init` function.
        match unsafe { *self.initialized.as_ref() } {
            // SAFETY: This is safe, because the only mutable borrow happens in the `init`
            // function.
            true => Some( unsafe {
                &*(self.tasklet_buffer.as_ref().as_ptr() as *const Tasklet<T, C>)
            }),
            false => None,
        }
    }
}
