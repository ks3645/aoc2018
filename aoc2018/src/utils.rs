use std::fs::File;
use std::io::Read;
use std::io;

#[derive(Debug, PartialEq)]
pub enum Part {
    One,
    Two
}

pub fn read_input_to_string(buf:&mut string, day:u8) -> io::Result<usize> {
    let filename = format!("input/day{0}.txt", day);
    let mut file = File::open(filename).expect("Invalid Day Input");
    file.read_to_string(buf)
}