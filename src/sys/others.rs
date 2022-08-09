use std::io;
use std::path::Path;

use super::reflink_not_supported;

pub fn reflink(_from: &Path, _to: &Path) -> io::Result<()> {
    reflink_not_supported()
}
