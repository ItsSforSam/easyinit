//! Utility functions for easyinit, used across multiple crates.


pub mod signals;
/// Affects how backtraces are printed in logs.
///
/// This currently is the `RUST_LIB_BACKTRACE` and `RUST_BACKTRACE` variables. These affect
/// how a backtrace is printed, and makes it difficult to read logs. Sets `RUST_BACKTRACE` to `full`, and removes
/// `RUST_LIB_BACKTRACE`.
/// 
/// # Safety
///
/// Should be called before any threads are spawned that may read the environment.
/// Concurrently, no other threads should be reading or writing the environment, as otherwise would
/// produce undefined behavior.
#[cold]
pub unsafe fn correct_env(){
    // SAFETY: the caller guarantees that no other threads are reading or writing the environment
    unsafe {
        // These environment 
        std::env::remove_var("RUST_LIB_BACKTRACE");
        std::env::set_var("RUST_BACKTRACE","full");
    }
}


#[derive(Debug)]
#[repr(u8)]
/// Log priority levels, similar to syslog levels
/// 
/// Details can be found [here][https://documentation.solarwinds.com/en/success_center/orionplatform/content/core-syslog-message-priorities-sw2141.htm#Syslog2]
#[expect(missing_docs, reason="Details are in listed documentation")]
pub enum Priority{
    Emergency,
    Alert,
    Critical,
    Error,
    Warning,
    Notice,
    Informational,
    Debug
    
}
impl From<log::Level> for Priority{
    fn from(level: log::Level) -> Self {
        match level{
            log::Level::Error => Priority::Error,
            log::Level::Warn => Priority::Warning,
            log::Level::Info => Priority::Informational,
            log::Level::Debug => Priority::Debug,
            log::Level::Trace => Priority::Debug, // Trace is more verbose than debug, but debug is the lowest syslog level
        }
    }
}