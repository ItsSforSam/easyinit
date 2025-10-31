//! This is stuff that needs to be started up like the mounted stuff, reads

use nix::mount::{MsFlags,mount};


/// This mounts filesystems that are required to run the system.
/// 
/// This includes, but not limited to, `/tmp`, `/run`
/// 
/// Optionally `/usr`, but easyinit discourages `/usr` on a separate partition
/// 
/// These are required for easyinit to successfully boot.
/// 
/// These will be mounted with specific options but may be remounted later
pub fn mount_needed_fs(){

    
    ensure_mountpoint_safe(std::path::Path::new("/tmp")).expect("Failed to ensure /tmp is safe to mount on");
    
    mount(
        None::<&str>, // Errors Rust due to it cannot being inferred
        "/tmp",
        Some("tmpfs"),
        MsFlags::MS_MGC_VAL | MsFlags::MS_NOEXEC | MsFlags::MS_NOSUID | MsFlags::MS_NODEV | MsFlags::MS_RELATIME
        
        ,Some("usrquota")
    ).unwrap_or_else(|e|{handle_needed_fs_errors(e, "/tmp", "tmpfs");});
    
    ensure_mountpoint_safe(std::path::Path::new("/run")).expect("Failed to ensure /run is safe to mount on");
    mount(
        None::<&str>,
        "/run",
        Some("tmpfs"),
        MsFlags::MS_MGC_VAL | MsFlags::MS_NOEXEC | MsFlags::MS_NOSUID | MsFlags::MS_NODEV | MsFlags::MS_RELATIME
        
        ,None::<&str>
    ).unwrap_or_else(|e|{handle_needed_fs_errors(e, "/run", "sysfs");});
}

fn handle_needed_fs_errors(e: nix::Error, label:&str,fstype:&str){
    use nix::errno::Errno::*;
    // Some of these panic as t
    match e{
        ENODEV => panic!("Kernel does not support {fstype} filesystem, cannot continue"),   

        _ => panic!("Unhandled errno: {e:?}" ) // Used if we miss one
        
        
    }
}

/// Helper function to ensure that the mountpoint exists (if not, create it), and that it has no contents. This is for required mounts.
/// 
/// This is to avoid mounting over a non-empty directory, which can cause data loss.
fn ensure_mountpoint_safe(path:&std::path::Path)->std::io::Result<()>{
    use std::fs;
   
    match fs::metadata(path){
        Ok(meta) => {
            if !meta.is_dir(){
                return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("{path:?} exists but is not a directory")));
            }
            let mut entries = fs::read_dir(path)?;
            if entries.next().is_some(){
                return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("{path:?} is not empty")));
            }
            Ok(())
        },
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            fs::create_dir(path)?;
            // If we created it, it's definitely empty
            Ok(())
        },
        Err(e) => Err(e),
    }
}