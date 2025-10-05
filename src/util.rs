//! General purpose functions

use std::sync::atomic::AtomicU8;

#[non_exhaustive]
pub enum ShutdownReason{
    /// Functions the same as [`ShutdownReason::User`], except it just reboots
    /// the system instead of letting the system stay shutdown
    Reboot,
    /// When the user requests a system shutdown.
    /// 
    /// Should try to close all the child process (which is all other process 
    /// since this is for init process), but if requests are ignored after a certain duration
    User,
    /// Power failure, sent by kernel via the `SIGPWR`
    /// 
    /// This should hastily try to prevent the system to be in a inconsistent state
    Power,
    /// If configured, this will simply mean that the watchdog daemon reports a failed
    /// test and will function similarity 
    /// 
    /// While not currently implemented, the concept is to follow the same philosophy as
    /// [watchdog(8)]
    /// 
    /// [watchdog(8)]:https://linux.die.net/man/8/watchdog
    Watchdog,
    // CSpell:words kexec
    /// Same as [`ShutdownReason::Reboot`] but uses [kexec(8)] which allows you to boot directly 
    /// into a new kernel without having 
    /// 
    /// [kexec(8)]: https://man.archlinux.org/man/kexec.8
    KExec
}
/// Can only be shuting down once, prevents race conditions
static SHUTING_DOWN: AtomicU8 = AtomicU8::new(0);

/// This is ran if a shutdown of the init is sent for one reason or another
/// 
/// Depending on the reason it may react differently
#[cold]
pub fn shutdown(reason:ShutdownReason)->!{
    todo!()

}