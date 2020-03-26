use std::io::Read;
use std::convert::TryInto;
use crate::xml::constants::Result;

pub struct IntReader {
    data: Vec<i32>,
    position: usize,
}

impl IntReader {
    pub fn new<T: Read>(mut file: T) -> Result<IntReader> {
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        let mut iter = buf.chunks(4);
        let mut tmp = iter.next();
        let mut data: Vec<i32> = Vec::new();
        while tmp.is_some() {
            let int = i32::from_le_bytes(tmp.unwrap().try_into().unwrap());
            data.push(int);
            tmp = iter.next();
        }
        Ok(IntReader {
            data,
            position: 0,
        })
    }
    pub fn read_int(&mut self) -> i32 {
        let int = self.data[self.position];
        self.position += 1;
        int
    }

    pub fn read_int_array(&mut self, length: usize) -> Vec<i32> {
        if length == 0 {
            vec![]
        } else {
            let slice = self.data[self.position..(self.position + length)].to_vec();
            self.position += length;
            slice
        }
    }

    pub fn skip_int(&mut self) {
        self.position += 1;
    }
}