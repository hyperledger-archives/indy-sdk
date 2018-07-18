use byteorder::{LittleEndian, WriteBytesExt, ReadBytesExt};
use std::io::Cursor;

pub fn usize_to_byte_array(n: usize) -> Vec<u8> {
    let mut wtr: Vec<u8> = Vec::new();
    wtr.write_u64::<LittleEndian>(n as u64).unwrap();
    wtr
}

pub fn byte_array_to_usize(v: Vec<u8>) -> usize {
    let mut rdr = Cursor::new(v);
    rdr.read_u64::<LittleEndian>().unwrap() as usize
}