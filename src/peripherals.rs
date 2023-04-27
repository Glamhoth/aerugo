//! TODO

#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;

/// TODO
pub struct Peripherals {}

/// TODO
impl Peripherals {
    /// TODO
    #[allow(dead_code)]
    pub(crate) fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }

    /// TODO
    #[allow(dead_code)]
    pub(crate) unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;

        Peripherals {}
    }
}
