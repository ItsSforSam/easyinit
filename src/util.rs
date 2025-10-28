//! General purpose functions

use std::sync::atomic::{AtomicBool,Ordering};

#[non_exhaustive]
pub enum ShutdownReason{
    /// Functions the same as [`ShutdownReason::User`], except it just reboots
    /// the system instead of letting the system stay shutdown. Allowing a cold boot
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
    /// into a new kernel without having to go through the BIOS/UEFI stage.
    /// 
    /// Faster reboots, but may cause issues with some hardware not being re-initialized properly.
    /// If [KHO] is supported, it may be used.
    /// 
    /// Generally not recommended unless you know what you're doing.
    /// 
    /// [kexec(8)]: https://man.archlinux.org/man/kexec.8
    /// [KHO]: https://docs.kernel.org/next/kho/usage.html
    KExec
}
/// Can only be shuting down once, prevents race conditions
static SHUTING_DOWN: AtomicBool = AtomicBool::new(false);

/// This is ran if a shutdown of the init is sent for one reason or another
/// 
/// Depending on the reason it may react differently.
/// 
/// If it is called multiple times, it will ignore all but the first call.
#[cold] // Shouldn't be called too often.
pub fn shutdown(reason:ShutdownReason)->!{
    match SHUTING_DOWN.compare_exchange_weak(
            false, 
                true,
                    Ordering::AcqRel, 
                    Ordering::Relaxed
                ){
                    Ok(_) => {

                        shutdown_branch(reason);
                        SHUTING_DOWN.store(true, Ordering::Release);
                        // We should now actually tell the kernel to shutdown
                        todo!()

                    }
                    Err(_) => {
                        // Already shutting down, ignore
                        while SHUTING_DOWN.load(Ordering::Relaxed){ // true
                            std::hint::spin_loop();
                        }
                        unreachable!("SHUTING_DOWN shouldn't turn back to false");
                    }
                }

}

fn shutdown_branch(reason:ShutdownReason){
    match reason{
        ShutdownReason::Reboot => {
            // Reboot the system
            todo!()
        }
        ShutdownReason::User => {
            // Gracefully shutdown the system
            todo!()
        }
        ShutdownReason::Power => {
            // Power failure, hastily shutdown the system
            todo!()
        }
        ShutdownReason::Watchdog => {
            // Watchdog triggered, try to gracefully shutdown the system
            todo!()
        }
        ShutdownReason::KExec => {
            // KExec into a new kernel
            todo!()
        }
    }
}

/// A command that can be sent to the kernel via SysRq
#[non_exhaustive]
enum SysRqCommand{
    Reboot,
    /// Immediately sync all filesystems
    Sync,
    /// Immediately remount all filesystems as read-only
    RemountReadOnly,
    /// Immediately trigger a kernel panic
    Panic,
    /// Immediately shutdown the system, does not sync
    Shutdown,
    /// Immediately terminate all processes except init
    TerminateAllProcesses,
    /// Immediately kill all processes except init
    KillAllProcesses,
}
fn sysrq(cmd:SysRqCommand){
    use std::io::Write;
    let mut f = std::fs::OpenOptions::new()
        .write(true)
        .read(false)
        .create(false)
        .open("/proc/sysrq-trigger")
        .expect("Failed to open /proc/sysrq-trigger");
    let cmd_char = match cmd{
        SysRqCommand::Reboot => 'b',
        SysRqCommand::Sync => 's',
        SysRqCommand::RemountReadOnly => 'u',
        SysRqCommand::Panic => 'c',
        SysRqCommand::Shutdown => 'o',
        SysRqCommand::TerminateAllProcesses => 'e',
        SysRqCommand::KillAllProcesses => 'i',
    };
    f.write_all(&[cmd_char as u8]).expect("Failed to write to /proc/sysrq-trigger");


}

/// Tells the kernel to shutdown the system
/// 
/// # Safety
/// This does not sync files, umount filesystems, or anything else.
/// It just tells the kernel to shutdown. Use with caution.
unsafe fn kernel_shutdown()->!{

    use std::io::prelude::*;
    // std::io::
    todo!()
}