use super::status::Status;
use std::fs::{metadata, File};
use std::io::Read;

pub fn test_array() {
    let arr = read_f32("latent42_100x100.bin");
    println!("{arr:?}")
}

pub fn new_array<T: Copy>(size: usize, _default: T) -> Vec<T> {
    let mut arr = Vec::new();
    for _ in 0..size {
        arr.push(_default);
    }
    return arr;
}

pub fn read_u8(filename: &str) -> Vec<u8> {
    let mut f = File::open(filename).expect(&format!("no file '{filename}' found."));
    let meta = metadata(filename).expect(&format!("unable to read '{filename}' metadata."));
    let mut buffer = vec![0; meta.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");
    return buffer;
}

pub fn read_f32(filename: &str) -> Vec<f32> {
    let buffer = read_u8(filename);
    return parse_f32(&buffer);
}

pub fn read_status(filename: &str) -> Vec<Status> {
    let buffer = read_u8(filename);
    return parse_status(&buffer);
}

pub fn parse_f32(bts: &Vec<u8>) -> Vec<f32> {
    let chunks = _chunks_4(&bts);
    let array: Vec<f32> = chunks.into_iter().map(|a| f32::from_le_bytes(a)).collect();
    return array;
}

pub fn parse_status(bts: &Vec<u8>) -> Vec<Status> {
    let mut array: Vec<Status> = Vec::new();
    for b in bts.into_iter() {
        let s: Status = (*b).into();
        array.push(s);
    }
    return array;
}

fn _chunks_4(barry: &Vec<u8>) -> Vec<[u8; 4]> {
    let n = 4;
    let mut chunks = Vec::new();
    let length = barry.len();

    if length % n != 0 {
        panic!("This should never happen.")
    }

    for i in (0..length).step_by(n) {
        let mut array = [0u8; 4];
        for (j, p) in array.iter_mut().enumerate() {
            *p = barry[i + j];
        }
        chunks.push(array);
    }

    return chunks;
}
