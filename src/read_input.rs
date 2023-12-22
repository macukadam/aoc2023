use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
};

pub fn read_input_lines(path: &str) -> Lines<BufReader<File>> {
    let file = File::open(path).unwrap();
    let buf_reader = BufReader::new(file);
    buf_reader.lines()
}
