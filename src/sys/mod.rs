#[cfg(unix)]
mod unix;
#[cfg(unix)]
pub use self::unix::reflink;

#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use self::windows::reflink;

#[cfg(not(any(unix, windows)))]
mod others;
#[cfg(not(any(unix, windows)))]
pub use self::others::reflink;

#[allow(dead_code)]
fn reflink_not_supported() -> std::io::Result<()> {
    Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        format!(
            "Operation not supported on {}-{}-{}",
            std::env::consts::ARCH,
            std::env::consts::OS,
            std::env::consts::FAMILY
        ),
    ))
}
