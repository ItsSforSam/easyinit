//! This is stuff that needs to be started up like the mounted stuff, reads

use nix::mount::{MsFlags,mount};

const REQUIRED_MOUNTS: &[(&str,&str)] = &[
    // mnt point, fs type
    ( "/var",    "devtmpfs")//,"/proc","/sys"
    ( "/proc",   "proc"    ),
    ( "/sys",    "sysfs"   ),
    ( "/run",    "tmpfs"   ),
    
    ];

/// This mounts filesystems that are required to run the system.
/// 
/// This includes, but not limited to, `/dev`, `/proc`, and `/sys`.
/// 
/// Optionally `/usr`, but easyinit discourages `/usr` on a separate partition
/// 
/// These are required for easyinit to successfully boot.
/// 
/// These will be mounted with specific options but may be remounted later
pub fn mount_needed_fs(){


    mount(
        None::<&str>, // Errors rust due to it cannot being inferred
        "/proc",
        Some("proc"),
        MsFlags::MS_MGC_VAL | MsFlags::MS_NOEXEC | MsFlags::MS_NOSUID | MsFlags::MS_NODEV | MsFlags::MS_RELATIME
        
        ,None::<&str>
    ).unwrap_or_else(|e|{handle_needed_fs_errors(e, "/proc", "proc");});
    

    mount(
        None::<&str>,
        "/sys",
        Some("sysfs"),
        MsFlags::MS_MGC_VAL | MsFlags::MS_NOEXEC | MsFlags::MS_NOSUID | MsFlags::MS_NODEV | MsFlags::MS_RELATIME
        
        ,None::<&str>
    ).unwrap_or_else(|e|{handle_needed_fs_errors(e, "/sys", "sysfs");});
}

fn handle_needed_fs_errors(e: nix::Error, label:&str,fstype:&str){
    use nix::errno::Errno::*;
    // Some of these panic as t
    match e{
        ENODEV => panic!("Kernel does not support {fstype} filesystem, cannot continue"),   

        _ => panic!("Unhandled errno: {e:?}" ) // Used if we miss one
        
        
    }
}