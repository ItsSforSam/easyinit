pub mod util;
mod signals;
mod startup;
fn main() -> ! {
    // signal_hook::flag::register(signal, flag);

    // SAFETY: In the current execution, we can safely say that there are no other threads reading or writing to the environment
    unsafe {
        // These environment 
        std::env::remove_var("RUST_LIB_BACKTRACE");
        std::env::remove_var("RUST_BACKTRACE");
    }
    // SAFETY: No logging implementation is called previously
    unsafe { logging::init().unwrap_unchecked() }

    todo!()
    
}





