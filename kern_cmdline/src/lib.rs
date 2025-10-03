//! Parses the command line that the kernel was booted with and extracts
//! easyinit specific options.
// cSpell:words Cmdline
use std::fs::File;
use std::io::prelude::*;
/// Represents parameters passed to easyinit via the kernel command line.
pub struct Cmdline{
    /// This uses the kernels `quite` and `easyinit.loglevel` options.
    /// 
    /// `quite` is an alias for `easyinit.loglevel=0`.
    /// 
    /// `easyinit.loglevel` will take priority if both are present.
    loglevel: bool,
}
impl Cmdline{
    /// Reads the command line from `/proc/cmdline` and parses it.
    /// 
    /// # Panics
    /// 
    /// Panics if `/proc/cmdline` cannot be opened.
    pub fn new()->Self{
        Cmdline::use_file(File::open("/proc/cmdline").expect("Cannot open `/proc/cmdline`"))
    }
    /// Uses a specific file as the source of the command line.
    /// 
    /// Used for testing.
    pub fn use_file(f:File)->Self{
        todo!()
    }
}
impl Default for Cmdline{
    /// Returns the default options for `Cmdline`.
    fn default()->Self{
        Self::new()
    }
}
