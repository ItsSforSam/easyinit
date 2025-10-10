
use std::sync::atomic::{AtomicPtr,Ordering};
static PANIC_INNER: AtomicPtr<fn(&std::panic::PanicHookInfo) -> !> = AtomicPtr::new(std::ptr::null_mut());

pub fn switch_panic(){
    std::panic::set_hook(Box::new(panic_handler));
}

fn panic_handler(info: &std::panic::PanicHookInfo) -> ! {
    
    
    loop {
        
    }
}