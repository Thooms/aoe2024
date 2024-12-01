use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

// Return the trimmed lines from the file.
pub fn read_lines(path: &str) -> Vec<String> {
    let p = Path::new(path);
    let file = File::open(&p).expect("unable to open file");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.expect("unable to get line").trim().to_owned())
        .collect()
}
