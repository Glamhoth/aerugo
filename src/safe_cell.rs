//! TODO

use core::cell::UnsafeCell;

/// TODO
#[repr(transparent)]
pub(crate) struct SafeCell<T>(UnsafeCell<T>);

impl<T> SafeCell<T> {
    /// TODO
    #[inline(always)]
    pub const fn new(value: T) -> Self {
        SafeCell(UnsafeCell::new(value))
    }

    /// TODO
    #[inline(always)]
    pub fn as_ref(&self) -> &T {
        unsafe { &*self.0.get() }
    }

    /// TODO
    #[inline(always)]
    pub fn as_ref_mut(&self) -> &mut T {
        unsafe { &mut *self.0.get() }
    }
}

/// TODO
unsafe impl<T> Sync for SafeCell<T> where T: Send {}
