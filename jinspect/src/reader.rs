use std::fs::File;
use std::io::{BufReader, Read};
use std::str;

pub fn read_n(r: &mut BufReader<File>, limit: usize) -> Vec<u8> {
    let mut buf = Vec::<u8>::with_capacity(limit);
    for _i in 0..limit {
        buf.push(0);
    }
    let _ = r.read_exact(&mut buf);
    buf
}

pub fn read_u8(r: &mut BufReader<File>) -> u8 {
    let mut buf: [u8; 1] = [0; 1];
    let _ = r.read_exact(&mut buf);
    u8::from_be_bytes(buf)
}

pub fn read_u16(r: &mut BufReader<File>) -> u16 {
    let mut buf: [u8; 2] = [0; 2];
    let _ = r.read_exact(&mut buf);
    u16::from_be_bytes(buf)
}

pub fn read_u32(r: &mut BufReader<File>) -> u32 {
    let mut buf: [u8; 4] = [0; 4];
    let _ = r.read_exact(&mut buf);
    u32::from_be_bytes(buf)
}

pub fn read_str(r: &mut BufReader<File>, length: usize) -> String {
    let buf = read_n(r, length);
    str::from_utf8(&buf).unwrap().to_string()
}
