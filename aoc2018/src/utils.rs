use std::fs::File;
use std::io;
use std::io::Read;

#[derive(Debug, PartialEq)]
pub enum Part {
    One,
    Two,
}

pub fn read_input_to_string(buf: &mut String, day: u8) -> io::Result<usize> {
    let filename = format!("input/day{}.txt", day);
    let mut file = File::open(filename).expect("Invalid Day Input");
    file.read_to_string(buf)
}
