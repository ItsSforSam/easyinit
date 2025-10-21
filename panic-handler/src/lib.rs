//! Panic handler for easyinit
//! 
//! Unwinds the stack to show an error message
// use std::sync::atomic::{AtomicPtr,Ordering};
// static PANIC_INNER: AtomicPtr<fn(&std::panic::PanicHookInfo) -> !> = AtomicPtr::new(std::ptr::null_mut());

use std::ops::Add;

thread_local! {
    static PANIC_PROTECTION:std::cell::RefCell<u8> = const {std::cell::RefCell::new(0)}
}
/// Switches the panic handler to the custom one
pub fn switch_panic(){
    std::panic::set_hook(Box::new(panic_handler));
}
/// The panic handler
#[cold]
fn panic_handler(info: &std::panic::PanicHookInfo) {
    PANIC_PROTECTION.with(|p| {
        let mut v =  p.borrow_mut();
        if *v > 0 {
            eprint!("");
            // reentrant panic, abort
            #[expect(clippy::disallowed_methods, reason = "Cannot panic, recurring panic would cause recursion in panic handler, which would stack overflow")]
            std::process::abort();
        } else {
            *v += 1;
        }
    });
    // BACKTRACING NOW
    let panic_msg = format!("{:?}", &info.payload());
    eprintln!("PANIC: {}", &panic_msg);
    backtrace::trace(|frame| {
        todo!()
    });

}