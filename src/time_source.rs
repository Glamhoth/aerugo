//! Module containing Aerugo's time source module, providing configurable timestamps for the system
//! Should be used internally by the system.

use aerugo_hal::system_hal::SystemHal;

use crate::hal::Hal;
use crate::internal_cell::InternalCell;
use crate::time::{TimerDurationU64, TimerInstantU64};

/// Type representing TimeSource's timestamp.
pub type Timestamp = TimerInstantU64<1_000_000>;
/// Type representing TimeSource's duration.
pub type Duration = TimerDurationU64<1_000_000>;

/// Time source, responsible for creating timestamps.
///
/// Allows time tracking/timestamp generation since three points in time:
/// * Hardware initialization (call to [`Aerugo::initialize`](crate::Aerugo::initialize))
/// * Start of Aerugo scheduler (call to [`Aerugo::start`](crate::Aerugo::start))
/// * User-defined offset
pub struct TimeSource {
    /// Time since system's scheduler start.
    system_start_offset: InternalCell<Option<Duration>>,
    /// User-defined offset.
    user_offset: InternalCell<Option<Duration>>,
}

impl TimeSource {
    /// Creates new instance of TimeSource
    pub const fn new() -> Self {
        TimeSource {
            system_start_offset: InternalCell::new(None),
            user_offset: InternalCell::new(None),
        }
    }

    /// Returns time since system initialization (call to [`Aerugo::initialize`](crate::Aerugo::initialize), start of the hardware timer)
    pub fn time_since_init() -> Timestamp {
        Hal::get_system_time()
    }

    /// Returns time since system's scheduler start (call to [`Aerugo::start`](crate::Aerugo::start)), or `None` if system hasn't started yet.
    pub fn time_since_start(&self) -> Option<Timestamp> {
        // SAFETY: This is safe as long as used in single-core context.
        if let Some(start_offset) = unsafe { *self.system_start_offset.as_ref() } {
            let current_time = TimeSource::time_since_init();
            return current_time.checked_sub_duration(start_offset);
        }
        None
    }

    /// Returns time since user-defined offset, or `None` if offset is not defined, or cannot be subtracted from system time.
    pub fn time_since_user_offset(&self) -> Option<Timestamp> {
        // SAFETY: This is safe as long as used in single-core context.
        if let Some(user_offset) = unsafe { *self.user_offset.as_ref() } {
            let current_time = TimeSource::time_since_init();
            return current_time.checked_sub_duration(user_offset);
        }
        None
    }

    /// Sets user-defined offset.
    ///
    /// Specified duration will be subtracted from time since system initialization when a timestamp
    /// is generated using [`TimeSource::time_since_user_offset`].
    ///
    /// See [`TimeSource::time_since_init`] for details about time since system initialization.
    ///
    /// # Safety
    /// This should never be called in IRQ context.
    ///
    /// # Parameters
    /// * `duration` - Duration to offset the time source with.
    pub fn set_user_offset(&self, duration: Duration) {
        // SAFETY: This is safe, as it's the only place where this member is mutated, and IRQs should not access this member.
        let offset_ref = unsafe { self.user_offset.as_mut_ref() };
        offset_ref.replace(duration);
    }

    /// Returns the duration between system initialization and start of the scheduler, or `None` if system hasn't started yet.
    pub fn startup_duration(&self) -> Option<Duration> {
        // SAFETY: This is safe as long as used in single-core context.
        unsafe { *self.system_start_offset.as_ref() }
    }

    /// Saves current timestamp as the moment of system start. Should be called by `Aerugo` right before starting the scheduler.
    ///
    /// # Safety
    /// This should never be called in IRQ context.
    pub(crate) fn mark_system_start(&self) {
        let current_time = TimeSource::time_since_init();
        // SAFETY: This is safe, as it's the only place where this member is mutated, and IRQs should not access this member.
        let offset_ref = unsafe { self.system_start_offset.as_mut_ref() };
        offset_ref.replace(current_time.duration_since_epoch());
    }
}
