use std::io;
use std::path::Path;

#[cfg(any(target_os = "linux", target_os = "android"))]
pub fn reflink(from: &Path, to: &Path) -> io::Result<()> {
    use crate::request_code_write;
    use std::fs;
    use std::os::unix::io::AsRawFd;

    let src = fs::File::open(&from)?;

    // pass O_EXCL to mimic macos behaviour
    let dest = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&to)?;
    let ret = unsafe {
        // from /usr/include/linux/fs.h:
        // #define FICLONE		_IOW(0x94, 9, int)
        const FICLONE_TYPE: u8 = 0x94;
        const FICLONE_NR: u8 = 9;
        const FICLONE_SIZE: usize = std::mem::size_of::<libc::c_int>();
        // http://man7.org/linux/man-pages/man2/ioctl_ficlonerange.2.html
        libc::ioctl(
            dest.as_raw_fd(),
            request_code_write!(FICLONE_TYPE, FICLONE_NR, FICLONE_SIZE),
            src.as_raw_fd(),
        )
    };

    if ret == -1 {
        let err = io::Error::last_os_error();
        // remove the empty file that was created.
        let _ = fs::remove_file(to);
        Err(err)
    } else {
        Ok(())
    }
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub fn reflink(from: &Path, to: &Path) -> io::Result<()> {
    use std::ffi::CString;
    use std::os::unix::ffi::OsStrExt;

    fn cstr(path: &Path) -> io::Result<CString> {
        Ok(CString::new(path.as_os_str().as_bytes())?)
    }

    // const CLONE_NOFOLLOW: libc::c_int = 0x0001;
    const CLONE_NOOWNERCOPY: libc::c_int = 0x0002;

    extern "C" {
        // http://www.manpagez.com/man/2/clonefileat/
        // https://github.com/apple/darwin-xnu/blob/0a798f6738bc1db01281fc08ae024145e84df927/bsd/sys/clonefile.h
        // TODO We need weak linkage here (OSX > 10.12, iOS > 10.0), otherwise compilation will fail on older versions
        fn clonefile(
            src: *const libc::c_char,
            dest: *const libc::c_char,
            flags: libc::c_int,
        ) -> libc::c_int;
    }

    let src = cstr(from)?;
    let dest = cstr(to)?;

    let ret = unsafe { clonefile(src.as_ptr(), dest.as_ptr(), CLONE_NOOWNERCOPY) };

    if ret == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

#[cfg(not(any(
    target_os = "linux",
    target_os = "android",
    target_os = "macos",
    target_os = "ios"
)))]
pub fn reflink(_from: &Path, _to: &Path) -> io::Result<()> {
    super::_reflink_not_supported()
}
