#[cfg(unix)] mod unix;
#[cfg(unix)] pub use self::unix::reflink;
#[cfg(windows)] mod windows;
#[cfg(windows)] pub use self::windows::reflink;
