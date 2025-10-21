pub mod util;


fn main() -> ! {
    // signal_hook::flag::register(signal, flag);

    // SAFETY: In the current execution, we can safely say that there are no other threads reading or writing to the environment
    unsafe {::utils::correct_env();}
    panic_handler::switch_panic();
    // SAFETY: No logging implementation is called previously
    unsafe { logging::init().unwrap_unchecked() }

    todo!()
    
}





