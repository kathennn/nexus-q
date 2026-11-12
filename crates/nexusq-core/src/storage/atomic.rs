//! Platform-specific atomic file replacement helpers.

use std::fs;
use std::io;
use std::path::Path;

/// Atomically replaces destination with source on supported filesystems.
pub(crate) fn replace_file(source: &Path, destination: &Path) -> io::Result<()> {
    #[cfg(windows)]
    {
        use std::{iter::once, os::windows::ffi::OsStrExt};

        const MOVEFILE_REPLACE_EXISTING: u32 = 0x1;
        const MOVEFILE_WRITE_THROUGH: u32 = 0x8;

        #[link(name = "Kernel32")]
        unsafe extern "system" {
            fn MoveFileExW(existing: *const u16, new: *const u16, flags: u32) -> i32;
        }

        let source: Vec<u16> = source.as_os_str().encode_wide().chain(once(0)).collect();
        let destination: Vec<u16> = destination
            .as_os_str()
            .encode_wide()
            .chain(once(0))
            .collect();
        let result = unsafe {
            MoveFileExW(
                source.as_ptr(),
                destination.as_ptr(),
                MOVEFILE_REPLACE_EXISTING | MOVEFILE_WRITE_THROUGH,
            )
        };
        if result == 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }
    #[cfg(not(windows))]
    {
        fs::rename(source, destination)
    }
}

/// Flushes the containing directory after a rename where the platform supports it.
pub(crate) fn sync_parent_dir(path: &Path) -> io::Result<()> {
    #[cfg(unix)]
    {
        if let Some(parent) = path.parent()
            && !parent.as_os_str().is_empty()
        {
            match fs::File::open(parent)?.sync_all() {
                Ok(()) => {}
                Err(error) if error.kind() == io::ErrorKind::Unsupported => {}
                Err(error) => return Err(error),
            }
        }
    }
    #[cfg(not(unix))]
    {
        let _ = path;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn replace_file_overwrites_an_existing_destination() {
        let dir = tempdir().unwrap();
        let source = dir.path().join("source.tmp");
        let destination = dir.path().join("destination.bin");
        fs::write(&source, b"new contents").unwrap();
        fs::write(&destination, b"old contents").unwrap();

        replace_file(&source, &destination).unwrap();

        assert_eq!(fs::read(&destination).unwrap(), b"new contents");
        assert!(!source.exists());
    }
}
