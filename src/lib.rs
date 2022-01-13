/// Reexport `fugit`
pub use fugit::*;

/// Provides `CountDown` timing capabilities
pub trait Timer<const TIMER_HZ: u32> {
    /// An error that might happen during waiting
    type Error;

    /// Return current time `Instant`
    fn now(&mut self) -> TimerInstantU32<TIMER_HZ>;

    /// Start timer with a `duration`
    fn start(&mut self, duration: TimerDurationU32<TIMER_HZ>) -> Result<(), Self::Error>;

    /// Tries to stop this timer.
    /// An error will be returned if the timer has already been canceled or was never started.
    /// An error is also returned if the timer is not `Periodic` and has already expired.
    fn cancel(&mut self) -> Result<(), Self::Error>;

    /// Wait until timer `duration` has expired.
    /// Must return `nb::Error::WouldBlock` if timer `duration` is not yet over.
    /// Must return `OK(())` as soon as timer `duration` has expired.
    fn wait(&mut self) -> nb::Result<(), Self::Error>;
}
