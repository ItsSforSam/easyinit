pub mod util;

fn main() {
    // SAFETY: In the current execution, we can safely say that there are no other threads reading or writing to the environment
    unsafe {
        // These environment 
        std::env::remove_var("RUST_LIB_BACKTRACE");
        std::env::remove_var("RUST_BACKTRACE");
    }
}
