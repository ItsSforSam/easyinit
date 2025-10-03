//! The internal logging crate for easyinit.
//! 
//! This provides the formatting for logs of the console and files.




/// Initializes logging system
pub fn init()-> Result<(),log::SetLoggerError>{
    // Initialize the logger here
    todo!()
    
}
#[derive(Debug)]
pub struct Logger{

}

impl Logger{
    
}


pub mod prelude{
    pub use log::{info,debug,error,warn,trace};
}
