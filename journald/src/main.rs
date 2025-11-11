//! This is the implementation of the journal daemon for easyinit
//! 
//! 
//! 
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    add_lock()?;


    exit_cleanup();
    Ok(())

}

fn add_lock()->Result<(),SetupError>{
    match std::fs::OpenOptions::new().create_new(true).write(false).open("/run/easyinit-journal.lock"){
        Ok(_) => {},
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => return Err(SetupError::AlreadyLocked),
        Err(e) => return Err(SetupError::IO(e)),
    }

    

    Ok(())
}

pub struct Journal{

}
pub struct Entry{
    
}

fn exit_cleanup(){
    
    let _ = std::fs::remove_file("/run/easyinit-journal.lock");

}

/// Error for setting up the journal
#[derive(thiserror::Error,Debug)]
pub enum SetupError{
    /// The journal lock is already held by another process
    #[error("Journal lock file is already held by another process")]
    AlreadyLocked,

    /// A generic IO error
    #[error(transparent)]
    IO(#[from] std::io::Error)
}

impl std::process::Termination for SetupError{
    fn report(self) -> std::process::ExitCode {
        use std::process::ExitCode;
        match self{
            SetupError::AlreadyLocked => ExitCode::from(2u8),
            SetupError::IO(_) => ExitCode::from(101) // 101 is for general errors, it is used for developer errors like panics
        }
    }
}

