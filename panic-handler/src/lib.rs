//! Panic handler for easyinit
//! 
//! Unwinds the stack to show an error message
// use std::sync::atomic::{AtomicPtr,Ordering};
// static PANIC_INNER: AtomicPtr<fn(&std::panic::PanicHookInfo) -> !> = AtomicPtr::new(std::ptr::null_mut());

use std::io::{stderr, stdout, Write};
// use std::ffi::{OsStr, OsString};


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
    let mut err = stderr().lock();
    // flushing the standard outputs, we don't need stdout handles
    let _ = stdout().flush();
    let _ = err.flush();
    PANIC_PROTECTION.with(|p| {
        let mut v =  p.borrow_mut();
        if *v > 0 {
            // Ignoring further errors as not much can be done and
            // if we use use eprintln! then it could possibly panic
            // If recursive panic detected, we should attempt to write the panic message
            let _ = err.write_all(format!("Recursive panic message: {:?}\n", payload_as_str(info)).as_bytes());
            let _ = err.write_all("Recursive panic detected, aborting\n".as_bytes());
            abort();
        } else {
            *v += 1;
        }
    });
    
    // BACKTRACING NOW
    // Currently info returns only Some varient, but stated it may return None in future
    let plocation = info.location().expect("Location cannot be determined");
    
    
    let _ = err.write_all(format!("Panic occurred at {} on line {}\n",
                                 plocation.file(),
                                plocation.line(),
                            ).as_bytes());
    
    // let mut 
    backtrace::trace(|frame| {
        let ip = frame.ip();
        let addr = frame.symbol_address();
        backtrace::resolve_frame(frame, |sym|{
            let name: String = format!("{}", 
                sym.name()
                .unwrap_or(
                    backtrace::SymbolName::new(b"<unknown symbol>")
                )
            );
    
            let filename= match sym.filename() {
                Some(f) => {
                    let path_str = f.display();
                    if let Some(l) = sym.lineno() {
                        format!("{} at line {}",path_str,l)
                    } else {
                        format!("{} at <unknown line>",path_str)
                    }
                },
                None => "<unknown file>".to_string(),
            };
            let _ = err.write_all(format!("{}",filename).as_bytes());
        });
        
        

        true // continue tracing

    });

}
/// When using aborting panic
fn abort_panic(){

}
/// Wrapper around [`std::process::abort`] to avoid having to use [expect attribute] with [`clippy::disallowed_methods`]
/// 
/// [expect attribute]: https://doc.rust-lang.org/reference/attributes/diagnostics.html#r-attributes.diagnostics.lint.expect
/// [`clippy::disallowed_methods`]: https://doc.rust-lang.org/clippy/lint_configuration.html#disallowed-methods
#[expect(clippy::disallowed_methods, reason = "Only crate allowed to call abort")]
#[inline]
fn abort() -> ! {
    std::process::abort();
}
/// Pollyfill for [`PanicHookInfo::payload_as_str`][std::panic::PanicHookInfo::payload_as_str]
///
#[inline]
pub fn payload_as_str<'a>(info: &'a std::panic::PanicHookInfo<'_>) -> Option<&'a str> {
    if let Some(s) = info.payload().downcast_ref::<&str>() {
        Some(s)
    } else if let Some(s) = info.payload().downcast_ref::<String>() {
        Some(s)
    } else {
        None
    }
}