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
use std::fs::read_dir;
use std::io::Result;
use std::ops::{Add, AddAssign};
use std::path::{Path, PathBuf};

static CRASH_FILE_EXTENSION: &str = ".crash";




/// View [module level documentation] for more info
///
/// [module level documentation]: self
#[cfg(feature = "coreutils")]
fn gen_filename(dir: &PathBuf) -> Result<PathBuf> {
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
    get_available_file(dir, &out)
}
#[cfg(feature = "coreutils")]
fn get_available_file(dir: &PathBuf, prefix: &OsString) -> Result<PathBuf> {
    // let direntry = read_dir(dir)?;
    // direntry.

    let ext = OsString::from(CRASH_FILE_EXTENSION);
    let mut id: u32 = 0;
    let mut id_str: OsString = id.to_string().into();

    let mut out = OsString::with_capacity(dir.capacity() + prefix.len() + ext.len() + id_str.len());
    // out.reserve();
    loop {
        out.push(dir.as_os_str());
        out.push(prefix);
        out.push(&id_str);
        out.push(CRASH_FILE_EXTENSION);

        match std::fs::exists(&out) {
            Ok(v) if !v => return Ok(out.into()),

            Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
                unreachable!("Permission should not be denied as program should be ran as root");
                // log::error!("Permission denied while accessing directory {}",out.display());
            }

            _ => {
                id = id.saturating_add(1);
                id_str = id.to_string().into();
                out.clear();
            }
        }
    }
}

#[cfg(not(doc))]
#[cfg(not(feature = "coreutils"))]
pub fn gen_filename() -> Result<String> {
    todo!()
}
