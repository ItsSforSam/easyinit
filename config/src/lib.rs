//! Parses the kernel's command line and 
mod crash_file_gen;

// cSpell:words Cmdline
use std::fs::File;
use log::LevelFilter;
use std::path::PathBuf;
use std::sync::LazyLock;
use std::io::prelude::*;
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


    /// An init system crashing is bad news. So a crash report should be generated if the is the case.
    /// 
    /// Uses the `easyinit.crash-prefix` options. Gets ignored if `easyinit.crash-path` option is set
    /// 
    /// The default is `/var/log/`
    /// The default should be fine with a system compliant with [LHS].
    /// 
    /// 
    /// [LHS]: https://refspecs.linuxfoundation.org/FHS_3.0/fhs/index.html
    crash_report_prefix: PathBuf,
    /// The crash report is a system
    /// 
    /// Uses the `easyinit.crash-path` and preferred over the crash prefix option, 
    /// and if used it uses a set path and overwrites any existing file.
    crash_report_file: LazyLock<PathBuf>
    
}
impl Cmdline{
    /// Reads the command line from `/proc/cmdline` and parses it.
    /// 
    /// # Panics
    /// 
    /// Panics if `/proc/cmdline` cannot be opened.
    pub fn new()->Self{
        Cmdline::use_file("/proc/cmdline".as_ref())
    }
    /// Uses a specific file as the source of the command line.
    /// 
    /// Used for testing.
    pub fn use_file(path:&std::path::Path)->Self{
        let mut r = Cmdline::default();

        let buf = std::fs::read_to_string(path);

        r
    }

    pub fn crash_report_path(&self) -> PathBuf{
        todo!()
        
    }
}
impl Default for Cmdline{
    /// Returns the default options for `Cmdline`.
    fn default()->Self{
        let prefix =  PathBuf::from("/var/log/");
        
        Cmdline {
            loglevel: LevelFilter::Warn,
            crash_report_prefix:prefix,
            crash_report_file: todo!()
            // crash_report_file: LazyLock::new(&||{crash_file_gen::gen_filename(&prefix)}),
            
        }
    }
}

pub enum CrashFile{

}
/// The idea that is that, is it implicitly set in the command line, or 
#[derive(Debug,PartialEq, Eq, PartialOrd, Ord)]
pub enum IsSet<T>{ // TODO: Better name
    Implicit(T),
    Explicit(T),
    /// Find it out when needed
    Lazily
}
