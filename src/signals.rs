use std::sync::atomic::AtomicBool;
use nix::sys::signal::{sigaction,SaFlags,Signal,SigAction,SigHandler,SigSet};
pub fn setup_signals(){
    
    
    let sig_running = std::sync::Arc::new(AtomicBool::new(false));
    sig_ignore(Signal::SIGUSR1).unwrap();
    sig_ignore(Signal::SIGUSR2).unwrap();
}


fn sig_ignore(signal: Signal)->nix::Result<SigAction>{
    // SAFETY: The function just ignores the signal
    unsafe {
        sigaction(signal, &SigAction::new(
            SigHandler::SigIgn,
             SaFlags::SA_NOCLDWAIT ,
             SigSet::empty()))
    }
}