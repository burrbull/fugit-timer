#![no_std]

/// Reexport `fugit`
pub use fugit::*;

/// Provides non-blocking `CountDown` timing capabilities
pub trait Timer<const TIMER_HZ: u32> {
    /// An error that might happen during waiting
    type Error: core::fmt::Debug;

    /// The backing storage used for the timer. Can be either `u32` or `u64`.
    type TimeStorage;

    /// Return current time `Instant`
    fn now(&mut self) -> Instant<Self::TimeStorage, 1, TIMER_HZ>;

    /// Start timer with a `duration`
    fn start(&mut self, duration: Duration<Self::TimeStorage, 1, TIMER_HZ>) -> Result<(), Self::Error>;

    /// Tries to stop this timer.
    /// An error will be returned if the timer has already been canceled or was never started.
    /// An error is also returned if the timer is not `Periodic` and has already expired.
    fn cancel(&mut self) -> Result<(), Self::Error>;

    /// Wait until timer `duration` has expired.
    /// Must return `nb::Error::WouldBlock` if timer `duration` is not yet over.
    /// Must return `OK(())` as soon as timer `duration` has expired.
    fn wait(&mut self) -> nb::Result<(), Self::Error>;
}

/// Provides blocking `Delay`
pub trait Delay<const TIMER_HZ: u32> {
    /// An error that might happen during waiting
    type Error: core::fmt::Debug;

    /// The backing storage used for the timer. Can be either `u32` or `u64`.
    type TimeStorage;

    /// Wait until timer `duration` has expired.
    fn delay(&mut self, duration: Duration<Self::TimeStorage, 1, TIMER_HZ>) -> Result<(), Self::Error>;
}
