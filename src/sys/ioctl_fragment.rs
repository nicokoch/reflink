// To keep the difference between this and the original file to a minimum
// do not run `cargo fmt` on this file.
#![allow(non_camel_case_types)]
#![allow(dead_code)]

// ****************************************************************************
// Based on the file src/sys/ioctl/linux.rs from the nix crate as of commit
// 10e69dbc99812775ad68b30c8d20cdae2346bca2 from 2020-01-12. The nix license
// is also included, then follow all the constants plus the `ioc` and
// `request_code_write` macros, with `#[doc(hidden)]` added to the latter
// and renaming `::ioctl::` to `::ioctl_fragment::`.
// ****************************************************************************

/*
The MIT License (MIT)

Copyright (c) 2015 Carl Lerche + nix-rust Authors

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
*/

/************ %< ********************************************* %< ************/
/// The datatype used for the ioctl number
#[cfg(any(target_os = "android", target_env = "musl"))]
#[doc(hidden)]
pub type ioctl_num_type = ::libc::c_int;
#[cfg(not(any(target_os = "android", target_env = "musl")))]
#[doc(hidden)]
pub type ioctl_num_type = ::libc::c_ulong;
/// The datatype used for the 3rd argument
#[doc(hidden)]
pub type ioctl_param_type = ::libc::c_ulong;

#[doc(hidden)]
pub const NRBITS: ioctl_num_type = 8;
#[doc(hidden)]
pub const TYPEBITS: ioctl_num_type = 8;

#[cfg(any(target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "sparc64"))]
mod consts {
    #[doc(hidden)]
    pub const NONE: u8 = 1;
    #[doc(hidden)]
    pub const READ: u8 = 2;
    #[doc(hidden)]
    pub const WRITE: u8 = 4;
    #[doc(hidden)]
    pub const SIZEBITS: u8 = 13;
    #[doc(hidden)]
    pub const DIRBITS: u8 = 3;
}

// "Generic" ioctl protocol
#[cfg(any(target_arch = "x86",
          target_arch = "arm",
          target_arch = "s390x",
          target_arch = "x86_64",
          target_arch = "aarch64",
          target_arch = "riscv64"))]
mod consts {
    #[doc(hidden)]
    pub const NONE: u8 = 0;
    #[doc(hidden)]
    pub const READ: u8 = 2;
    #[doc(hidden)]
    pub const WRITE: u8 = 1;
    #[doc(hidden)]
    pub const SIZEBITS: u8 = 14;
    #[doc(hidden)]
    pub const DIRBITS: u8 = 2;
}

pub use self::consts::*;

#[doc(hidden)]
pub const NRSHIFT: ioctl_num_type = 0;
#[doc(hidden)]
pub const TYPESHIFT: ioctl_num_type = NRSHIFT + NRBITS as ioctl_num_type;
#[doc(hidden)]
pub const SIZESHIFT: ioctl_num_type = TYPESHIFT + TYPEBITS as ioctl_num_type;
#[doc(hidden)]
pub const DIRSHIFT: ioctl_num_type = SIZESHIFT + SIZEBITS as ioctl_num_type;

#[doc(hidden)]
pub const NRMASK: ioctl_num_type = (1 << NRBITS) - 1;
#[doc(hidden)]
pub const TYPEMASK: ioctl_num_type = (1 << TYPEBITS) - 1;
#[doc(hidden)]
pub const SIZEMASK: ioctl_num_type = (1 << SIZEBITS) - 1;
#[doc(hidden)]
pub const DIRMASK: ioctl_num_type = (1 << DIRBITS) - 1;

/// Encode an ioctl command.
#[macro_export]
#[doc(hidden)]
macro_rules! ioc {
    ($dir:expr, $ty:expr, $nr:expr, $sz:expr) => (
        (($dir as $crate::sys::ioctl_fragment::ioctl_num_type & $crate::sys::ioctl_fragment::DIRMASK) << $crate::sys::ioctl_fragment::DIRSHIFT) |
        (($ty as $crate::sys::ioctl_fragment::ioctl_num_type & $crate::sys::ioctl_fragment::TYPEMASK) << $crate::sys::ioctl_fragment::TYPESHIFT) |
        (($nr as $crate::sys::ioctl_fragment::ioctl_num_type & $crate::sys::ioctl_fragment::NRMASK) << $crate::sys::ioctl_fragment::NRSHIFT) |
        (($sz as $crate::sys::ioctl_fragment::ioctl_num_type & $crate::sys::ioctl_fragment::SIZEMASK) << $crate::sys::ioctl_fragment::SIZESHIFT))
}

/// Generate an ioctl request code for a command that writes.
///
/// This is equivalent to the `_IOW()` macro exposed by the C ioctl API.
///
/// You should only use this macro directly if the `ioctl` you're working
/// with is "bad" and you cannot use `ioctl_write!()` directly.
///
/// The read/write direction is relative to userland, so this
/// command would be userland is writing and the kernel is
/// reading.
#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! request_code_write {
    ($ty:expr, $nr:expr, $sz:expr) => (ioc!($crate::sys::ioctl_fragment::WRITE, $ty, $nr, $sz))
}
