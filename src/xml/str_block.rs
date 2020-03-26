use std::ops::Add;
use crate::xml::reader::IntReader;
use std::char;

pub const CHUNK_TYPE: i32 = 0x001C0001;

pub struct StringBlock {
    string_offsets: Vec<i32>,
    strings: Vec<i32>,
    // style_offsets: Vec<i32>,
    // styles: Vec<i32>,
}

impl StringBlock {
    pub fn get_string(&mut self, index: i32) -> Option<String> {
        if index < 0 || index > self.string_offsets.len() as i32 {
            None
        } else {
            let mut offset = self.string_offsets[index as usize];
            let mut length = self.get_short(offset);
            let mut result = String::new();
            while length != 0 {
                offset += 2;
                let tmp = self.get_short(offset);
                let c = match char::from_u32(tmp) {
                    Some(c) => c,
                    None => ' '
                };
                result = result.add(format!("{}", c).as_str());
                length -= 1;
            }
            Some(result)
        }
    }

    fn get_short(&self, offset: i32) -> u32 {
        let value = self.strings[(offset / 4) as usize] as u32;
        if (offset % 4) / 2 == 0 {
            value & 0xFFFF
        } else {
            value >> 16
        }
    }

    pub fn read(reader: &mut IntReader) -> Option<StringBlock> {
        let tp = reader.read_int();
        if tp != CHUNK_TYPE {
            None
        } else {
            let chunk_size = reader.read_int();
            let string_count = reader.read_int();
            let style_offset_count = reader.read_int();
            reader.read_int();
            let strings_offset = reader.read_int();
            let styles_offset = reader.read_int();
            let string_offsets = reader.read_int_array(string_count as usize);
            let _ = if style_offset_count != 0 {
                reader.read_int_array(style_offset_count as usize)
            } else { vec![] };
            let size = if styles_offset == 0 { chunk_size - strings_offset } else { styles_offset - strings_offset };
            let strings = if (size % 4) != 0 {
                vec![]
            } else {
                reader.read_int_array((size / 4) as usize)
            };
            let _ = if styles_offset != 0 {
                let size = chunk_size - styles_offset;
                if (size % 4) != 0 {
                    vec![]
                } else {
                    reader.read_int_array((size / 4) as usize)
                }
            } else {
                vec![]
            };
            let block = StringBlock {
                string_offsets,
                strings,
                // style_offsets,
                // styles,
            };
            Some(block)
        }
    }
}

