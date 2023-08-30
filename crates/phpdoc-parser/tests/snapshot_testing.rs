use std::{env, fs, io, path::PathBuf};

use pretty_assertions::assert_str_eq;

#[test]
pub fn run_tests() -> io::Result<()> {
    let manifest = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let tests = manifest.join("tests/snapshots");

    let mut entries = fs::read_dir(tests)?
        .flatten()
        .map(|entry| entry.path())
        .filter(|entry| entry.is_dir())
        .collect::<Vec<PathBuf>>();

    entries.sort();

    for entry in entries {
        dbg!(entry);
    }

    Ok(())
}
