//! Parses the kernel's command line and 
// cSpell:words Cmdline
use std::fs::File;
use std::io::prelude::*;
use log::LevelFilter;


/// Represents parameters passed to easyinit via the kernel command line.
#[derive(Debug)]
#[non_exhaustive]
pub struct Cmdline{
    /// This uses the kernels `quite` and `easyinit.loglevel` options.
    /// 
    /// `quite` is an alias for `easyinit.loglevel=0`.
    /// 
    /// `easyinit.loglevel` will take priority if both are present.
    /// 
    /// Default is Warning level (3)
    pub loglevel: LevelFilter,

    

    
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
        let mut r = Cmdline::default();
        

        r
    }
}
impl Default for Cmdline{
    /// Returns the default options for `Cmdline`.
    fn default()->Self{
        Cmdline {
            loglevel: LevelFilter::Warn
        }
    }
}
