mod sys;

use std::io;
use std::path::Path;

pub fn reflink<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> io::Result<()> {
    let (from, to) = (from.as_ref(), to.as_ref());
    if !from.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "the source path is not an existing regular file",
        ));
    }
    sys::reflink(from, to)
}

#[cfg(test)]
mod test {
    use super::reflink;
    #[test]
    fn test_reflink() {
        let res = reflink("src/lib.rs", "reflink_lib.rs");
        println!("{:?}", res);
    }
}
