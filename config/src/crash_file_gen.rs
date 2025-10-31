//! Generates the filename of a crash report
//! 
//! The file would be prefixed with `easyinit-` followed by the date and identifier.
//! 
//! The date would be in `YY-MM-DD-#` format, where `#` means is to prevent overwriting preexisting crash logs
//! 
//! This should be used to lazily get the file name. If it gets created after the fact, by a alternative program, while it would technically create data 
//! loss, with a time-of-check to time-of-use ([TOCTOU]) race condition, but due to the unique format this generally is not a concern.
//! 
//! [TOCTOU]: std::fs#time-of-check-to-time-of-use-toctou
use std::ffi::{OsStr, OsString};
use std::io::Result;
use std::fs::read_dir;
use std::ops::{Add, AddAssign};
use std::path::{Path, PathBuf};


static  CRASH_FILE_EXTENSION:&str = ".crash";

/// View [module level documentation] for more info
/// 
/// [module level documentation]: self
#[cfg(feature="coreutils")]
pub fn gen_filename(dir:&PathBuf) -> Result<PathBuf>{
    let date = std::process::Command::new("/bin/date")
                        .env_clear()
                        .arg("+%Y-%m-%d")
                        .env("LC_ALL", "C") // prevent locales to become an issue
                        .output()?;
    let mut out: OsString = "easyinit-".to_string().into();
    let date = String::from_utf8_lossy(&date.stdout);
    
    let date = date.trim_end_matches('\x0a'); //removes newline the the command generates, for some reason
    out.push(date);
    out.push("-");
    get_available_file(dir, &out);
    


    Ok(out)
                        
}

fn get_available_file(dir:&PathBuf,prefix:&OsString) -> Result<PathBuf> {

    // let direntry = read_dir(dir)?;
    // direntry.


    let ext = OsString::from(CRASH_FILE_EXTENSION);
    let mut id:u32 = 0;
    let mut id_str:OsString = id.to_string().into();
    // let options = std::fs::OpenOptions::new().write(false).append(false).read(read);
    let mut out = OsString::with_capacity(dir.capacity() + prefix.capacity() + ext.capacity() + id_str.capacity()); // the +1 is for the aditional number 
    // out.reserve();
    loop {
        out.push(&[dir, pre,id_str]);
        match std::fs::exists(&out){
            Ok(v) if !v => {
                return Ok(out)

            }
            
            Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied =>{
                log::error!("Permission denied while accessing directory {}",out.display());
                id = id.saturating_add(1);
            }

            _ => id = id.saturating_add(1),

        }
    };

}


#[cfg(not(doc))]
#[cfg(not(feature="coreutils"))]
pub fn gen_filename() -> Result<String>{
    todo!()
}