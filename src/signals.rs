use std::sync::atomic::AtomicBool;

pub fn setup_signals(){
    use signal_hook::consts::{TERM_SIGNALS,signal as sigs};
    
    let sig_running = std::sync::Arc::new(AtomicBool::new(false));
    
    for sig in TERM_SIGNALS{
        unsafe { libc::signal(*sig,libc::SIG_IGN)};
    }
}