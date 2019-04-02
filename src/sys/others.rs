use std::io;
use std::path::Path;

pub fn reflink(_from: &Path, _to: &Path) -> io::Result<()> {
    super::_reflink_not_supported()
}
