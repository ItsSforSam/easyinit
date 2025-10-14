//! Handing signals
use std::sync::atomic::AtomicBool;
use nix::sys::signal::{sigaction,SaFlags,Signal,SigAction,SigHandler,SigSet, self as sig};
/// Sets 
pub fn basic_signal_handling(){
    
    
    let sig_running = std::sync::Arc::new(AtomicBool::new(false));
    // Don't block signals these being blocked can cause undefined behavior, besides 
    // https://man.archlinux.org/man/sigprocmask.2.en#NOTES
    let sigmask = {
        let mut s = sig::SigSet::all();
        s.remove(sig::SIGBUS);
        s.remove(sig::SIGFPE);
        s.remove(sig::SIGILL);
        s.remove(sig::SIGSEGV);
        s
    };
    sig_ignore(Signal::SIGUSR1).unwrap();
    sig_ignore(Signal::SIGUSR2).unwrap();
    
}

/// Set a signal to be ignored
/// 
/// Returns the previous action on success.
pub fn sig_ignore(signal: Signal)->nix::Result<SigAction>{
    // SAFETY: The function just ignores the signal, so no unsafe behavior from function, and signal should
    // only be installed via this implementation
    unsafe {
        sigaction(signal, &SigAction::new(
            SigHandler::SigIgn,
             SaFlags::SA_NOCLDWAIT ,
             SigSet::all()))
    }
}
// // pub type SigFunction = extern "C" fn
// pub struct SigThreadFunctions{
//     Option<>
// }

// impl SigThreadFunctions{
//     pub fn new()->Self{
//         Self{
//             nix::sys::signal::S
//         }
//     }
// }