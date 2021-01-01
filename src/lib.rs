use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;

pub fn stdin_lines() -> impl Iterator<Item = io::Result<String>> {
    BufReader::new(io::stdin()).lines()
}

pub fn file_lines(
    path: impl AsRef<PathBuf>,
) -> io::Result<impl Iterator<Item = io::Result<String>>> {
    let f = File::open(path.as_ref())?;
    Ok(BufReader::new(f).lines())
}
