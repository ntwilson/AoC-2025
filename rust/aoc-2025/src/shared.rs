use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read};

pub fn load_input_lines(filename: &str) -> io::Result<Vec<String>> {
  let file =
    File::open("../../puzzleInput/".to_string() + filename).expect("Failed to open input file");
  let reader = BufReader::new(file);
  reader.lines().collect::<io::Result<Vec<String>>>()
}

pub fn load_input_str(filename: &str) -> io::Result<String> {
  let file =
    File::open("../../puzzleInput/".to_string() + filename).expect("Failed to open input file");
  let mut reader = BufReader::new(file);
  let mut buf = String::new();
  reader.read_to_string(&mut buf)?;
  Ok(buf)
}
