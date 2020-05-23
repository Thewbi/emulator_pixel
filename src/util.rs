use std::fs::{metadata, File};
use std::io::Read;
use std::string::String;

#[derive(Clone, Debug)]
pub struct Util {}

impl Util {
    pub fn get_file_as_byte_vec(filename: &String) -> Vec<u8> {
        let mut file = File::open(&filename).expect("no file found");
        let metadata = metadata(&filename).expect("unable to read metadata");
        let mut buffer = vec![0; metadata.len() as usize];
        file.read(&mut buffer).expect("buffer overflow");
        buffer
    }
}
