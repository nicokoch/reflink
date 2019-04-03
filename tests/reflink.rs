use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use tempfile::tempdir;

use reflink::{reflink, reflink_or_copy};

#[test]
fn reflink_file_does_not_exist() {
    let from = Path::new("test/nonexistent-bogus-path");
    let to = Path::new("test/other-bogus-path");

    match reflink(&from, &to) {
        Ok(..) => panic!(),
        Err(..) => {
            assert!(!from.exists());
            assert!(!to.exists());
        }
    }
}

#[test]
fn reflink_src_does_not_exist() -> io::Result<()> {
    let tmpdir = tempdir()?;
    let from = Path::new("test/nonexistent-bogus-path");
    let to = tmpdir.path().join("out.txt");
    File::create(&to)?.write(b"hello")?;
    assert!(reflink(&from, &to).is_err());
    assert!(!from.exists());
    let mut v = Vec::new();
    File::open(&to)?.read_to_end(&mut v)?;
    assert_eq!(v, b"hello");
    Ok(())
}

#[test]
fn reflink_dest_is_dir() -> io::Result<()> {
    let dir = tempdir()?;
    let src_file_path = dir.path().join("src.txt");
    let _src_file = File::create(&src_file_path)?;
    match reflink(&src_file_path, dir.path()) {
        Ok(()) => panic!(),
        Err(e) => {
            println!("{:?}", e);
            if !cfg!(windows) {
                assert_eq!(e.kind(), io::ErrorKind::AlreadyExists);
            }
        }
    }
    Ok(())
}

#[test]
fn reflink_src_is_dir() -> io::Result<()> {
    let dir = tempdir()?;
    let dest_file_path = dir.path().join("dest.txt");

    match reflink(dir.path(), &dest_file_path) {
        Ok(()) => panic!(),
        Err(e) => {
            println!("{:?}", e);
            assert_eq!(e.kind(), io::ErrorKind::InvalidInput)
        }
    }
    Ok(())
}

#[test]
fn reflink_existing_dest_results_in_error() -> io::Result<()> {
    let dir = tempdir()?;
    let src_file_path = dir.path().join("src.txt");
    let dest_file_path = dir.path().join("dest.txt");

    let _src_file = File::create(&src_file_path)?;
    let _dest_file = File::create(&dest_file_path)?;

    match reflink(&src_file_path, &dest_file_path) {
        Ok(()) => panic!(),
        Err(e) => {
            println!("{:?}", e);
            assert_eq!(e.kind(), io::ErrorKind::AlreadyExists)
        }
    }
    Ok(())
}

#[test]
fn reflink_ok() -> io::Result<()> {
    let dir = tempdir()?;
    let src_file_path = dir.path().join("src.txt");
    let dest_file_path = dir.path().join("dest.txt");

    let mut src_file = File::create(&src_file_path)?;
    src_file.write(b"this is a test")?;

    match reflink(&src_file_path, &dest_file_path) {
        Ok(()) => {}
        Err(e) => {
            println!("{:?}", e);
            // do not panic for now, CI envs are old and will probably error out
            return Ok(());
        }
    }
    let mut v = Vec::new();
    File::open(&dest_file_path)?.read_to_end(&mut v)?;
    assert_eq!(v, b"this is a test");
    Ok(())
}

#[test]
fn reflink_or_copy_ok() -> io::Result<()> {
    let tmpdir = tempdir()?;
    let input = tmpdir.path().join("in.txt");
    let out = tmpdir.path().join("out.txt");

    File::create(&input)?.write(b"hello")?;
    reflink_or_copy(&input, &out)?;
    let mut v = Vec::new();
    File::open(&out)?.read_to_end(&mut v)?;
    assert_eq!(v, b"hello");

    assert_eq!(
        input.metadata()?.permissions(),
        out.metadata()?.permissions()
    );
    Ok(())
}
