#[cfg(feature = "calculator-build")]
mod calculator {
    #[cfg(feature = "calculator-build")]
    /// Sleeps for the specified number of milliseconds on the calculator.
    pub fn sleep(ms: u32) {
        unsafe { ndless_sys::msleep(ms) };
    }
}

#[cfg(not(feature = "calculator-build"))]
mod desktop {
    /// Sleeps for the specified number of milliseconds on desktop platforms.
    pub fn sleep(ms: u32) {
        std::thread::sleep(std::time::Duration::from_millis(ms as u64));
    }
}

#[cfg(feature = "calculator-build")]
pub use calculator::sleep;

#[cfg(not(feature = "calculator-build"))]
pub use desktop::sleep;
