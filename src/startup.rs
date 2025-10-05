//! This is stuff that needs to be started up like the mounted stuff, reads

use nix::mount::{MsFlags,mount};

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
    mount(None,"/proc","")
}
