use crate::xml::reader::IntReader;
use crate::xml::str_block::StringBlock;
use crate::xml::constants::*;
use crate::xml::namespace_stack::NamespaceStack;
use std::io::Read;

const ATTRIBUTE_IX_NAMESPACE_URI: i32 = 0;
const ATTRIBUTE_IX_NAME: i32 = 1;
const ATTRIBUTE_IX_VALUE_STRING: i32 = 2;
const ATTRIBUTE_IX_VALUE_TYPE: i32 = 3;
const ATTRIBUTE_IX_VALUE_DATA: i32 = 4;
const ATTRIBUTE_LENGHT: i32 = 5;

const CHUNK_AXML_FILE: i32 = 0x00080003;
const CHUNK_RESOURCEIDS: i32 = 0x00080180;
const CHUNK_XML_FIRST: i32 = 0x00100100;
const CHUNK_XML_START_NAMESPACE: i32 = 0x00100100;
const CHUNK_XML_END_NAMESPACE: i32 = 0x00100101;
const CHUNK_XML_START_TAG: i32 = 0x00100102;
const CHUNK_XML_END_TAG: i32 = 0x00100103;
const CHUNK_XML_TEXT: i32 = 0x00100104;
const CHUNK_XML_LAST: i32 = 0x00100104;

pub struct AXmlParser {
    reader: Option<IntReader>,
    operational: bool,
    strings: Option<StringBlock>,
    resource_ids: Option<Vec<i32>>,
    namespaces: NamespaceStack,
    decrease_depth: bool,
    event: i32,
    line_number: i32,
    name: i32,
    namespace_uri: i32,
    attributes: Vec<i32>,
    id_attribute: i32,
    class_attribute: i32,
    style_attribute: i32,
}

impl AXmlParser {
    pub fn new() -> AXmlParser {
        AXmlParser {
            reader: None,
            operational: false,
            strings: None,
            resource_ids: None,
            namespaces: NamespaceStack::new(),
            decrease_depth: false,
            event: -1,
            line_number: -1,
            name: -1,
            namespace_uri: -1,
            attributes: vec![],
            id_attribute: -1,
            class_attribute: -1,
            style_attribute: -1,
        }
    }
    pub fn open<T: Read>(&mut self, file: T) -> Result<()> {
        let reader = IntReader::new(file)?;
        self.reader = Some(reader);
        Ok(())
    }
    pub fn next(&mut self) -> i32 {
        if self.reader.is_none() {
            panic!("Parser is not opened")
        } else {
            self.do_next();
            self.event
        }
    }
    fn do_next(&mut self) {
        match &mut self.reader {
            Some(reader) => {
                if self.strings.is_none() {
                    let tp = reader.read_int();
                    if tp != CHUNK_AXML_FILE {
                        panic!("Not Expected chunk")
                    }
                    reader.skip_int();
                    self.strings = StringBlock::read(reader);
                    self.namespaces.increase_depth();
                    self.operational = true;
                }

                if self.event == END_DOCUMENT {
                    return;
                }
                let event = self.event;
                self.event = -1;
                self.line_number = -1;
                self.name = -1;
                self.namespace_uri = -1;
                self.attributes = vec![];
                self.id_attribute = -1;
                self.class_attribute = -1;
                self.style_attribute = -1;
                loop {
                    if self.decrease_depth {
                        self.decrease_depth = false;
                        self.namespaces.decrease_depth();
                    }
                    if event == END_TAG && self.namespaces.get_depth() == 1 && self.namespaces.get_current_count() == 0 {
                        self.event = END_DOCUMENT;
                        break;
                    }
                    let chunk_type = if event == START_DOCUMENT {
                        CHUNK_XML_START_TAG
                    } else { reader.read_int() };
                    if chunk_type == CHUNK_RESOURCEIDS {
                        let chunk_size = reader.read_int();
                        if chunk_size < 8 || (chunk_size % 4) != 0 {
                            panic!("Invalid resource ids size")
                        }
                        self.resource_ids = Some(reader.read_int_array((chunk_size / 4 - 2) as usize));
                        continue;
                    }
                    if chunk_type < CHUNK_XML_FIRST || chunk_type > CHUNK_XML_LAST {
                        panic!("Invalid chunk type")
                    }
                    if chunk_type == CHUNK_XML_START_TAG && event == -1 {
                        self.event = START_DOCUMENT;
                        break;
                    }
                    reader.skip_int();
                    let line_number = reader.read_int();
                    reader.skip_int();
                    if chunk_type == CHUNK_XML_START_NAMESPACE || chunk_type == CHUNK_XML_END_NAMESPACE {
                        if chunk_type == CHUNK_XML_START_NAMESPACE {
                            let prefix = reader.read_int();
                            let uri = reader.read_int();
                            self.namespaces.push(prefix, uri);
                        } else {
                            reader.skip_int();
                            reader.skip_int();
                            self.namespaces.pop();
                        }
                        continue;
                    }
                    self.line_number = line_number as i32;
                    if chunk_type == CHUNK_XML_START_TAG {
                        self.namespace_uri = reader.read_int() as i32;
                        self.name = reader.read_int() as i32;
                        reader.skip_int();
                        let mut ac = reader.read_int();
                        self.id_attribute = (ac >> 16 - 1) as i32;
                        ac &= 0xFFFF;
                        self.class_attribute = reader.read_int() as i32;
                        self.style_attribute = self.class_attribute >> 16 - 1;
                        self.class_attribute = self.class_attribute & 0xFFFF - 1;
                        self.attributes = reader.read_int_array((ac * ATTRIBUTE_LENGHT) as usize);
                        let mut i = ATTRIBUTE_IX_VALUE_TYPE;
                        while i < self.attributes.len() as i32 {
                            self.attributes[i as usize] = self.attributes[i as usize] >> 24;
                            i += ATTRIBUTE_LENGHT;
                        }
                        self.namespaces.increase_depth();
                        self.event = START_TAG;
                        break;
                    }
                    if chunk_type == CHUNK_XML_END_TAG {
                        self.namespace_uri = reader.read_int() as i32;
                        self.name = reader.read_int() as i32;
                        self.event = END_TAG;
                        self.decrease_depth = true;
                        break;
                    }
                    if chunk_type == CHUNK_XML_TEXT {
                        self.name = reader.read_int() as i32;
                        reader.skip_int();
                        reader.skip_int();
                        self.event = TEXT;
                        break;
                    }
                }
            }
            None => {
                panic!("Parser is not opened")
            }
        }
    }
    pub fn get_prefix(&mut self) -> Option<String> {
        let prefix = self.namespaces.find_prefix(self.namespace_uri);
        match &mut self.strings {
            Some(data) => {
                data.get_string(prefix)
            }
            None => None
        }
    }
    pub fn get_name(&mut self) -> Option<String> {
        if self.event != START_TAG && self.event != END_TAG {
            None
        } else {
            match &mut self.strings {
                Some(data) => data.get_string(self.name),
                None => None
            }
        }
    }
    pub fn get_depth(&self) -> i32 {
        self.namespaces.get_depth() - 1
    }
    pub fn get_text(&mut self) -> Option<String> {
        if self.event != TEXT {
            None
        } else {
            match &mut self.strings {
                Some(data) => data.get_string(self.name),
                None => None
            }
        }
    }
    pub fn get_namespace_count(&self, depth: i32) -> i32 {
        self.namespaces.get_accumulated_count(depth)
    }
    pub fn get_attribute_count(&self) -> isize {
        if self.event != START_TAG { -1 } else {
            (self.attributes.len() as i32 / ATTRIBUTE_LENGHT) as isize
        }
    }
    pub fn get_namespace_prefix(&mut self, index: i32) -> Option<String> {
        let prefix = self.namespaces.get_prefix(index);
        match &mut self.strings {
            Some(data) => data.get_string(prefix),
            None => None
        }
    }
    pub fn get_namespace_uri(&mut self, index: i32) -> Option<String> {
        let prefix = self.namespaces.get_uri(index);
        match &mut self.strings {
            Some(data) => data.get_string(prefix),
            None => None
        }
    }
    pub fn get_attribute_prefix(&mut self, index: i32) -> Option<String> {
        let offset = self.get_attribute_offset(index);
        let uri = self.attributes[(offset + ATTRIBUTE_IX_NAMESPACE_URI) as usize];
        let prefix = self.namespaces.find_prefix(uri as i32);
        match &mut self.strings {
            Some(data) => data.get_string(prefix),
            None => None
        }
    }
    pub fn get_attribute_name(&mut self, index: i32) -> Option<String> {
        let offset = self.get_attribute_offset(index);
        let name = self.attributes[(offset + ATTRIBUTE_IX_NAME) as usize];
        match &mut self.strings {
            Some(data) => data.get_string(name as i32),
            None => None
        }
    }
    pub fn get_attribute_value_type(&self, index: i32) -> i32 {
        let offset = self.get_attribute_offset(index);
        self.attributes[(offset + ATTRIBUTE_IX_VALUE_TYPE) as usize]
    }
    pub fn get_attribute_value_data(&self, index: i32) -> i32 {
        let offset = self.get_attribute_offset(index);
        self.attributes[(offset + ATTRIBUTE_IX_VALUE_DATA) as usize]
    }
    pub fn get_attribute_value1(&mut self, index: i32) -> Option<String> {
        let offset = self.get_attribute_offset(index);
        let value_type = self.attributes[(offset + ATTRIBUTE_IX_VALUE_TYPE) as usize];
        if value_type == TYPE_STRING {
            let value_string = self.attributes[(offset + ATTRIBUTE_IX_VALUE_STRING) as usize];
            match &mut self.strings {
                Some(data) => data.get_string(value_string),
                None => None
            }
        } else {
            let _ = self.attributes[(offset + ATTRIBUTE_IX_VALUE_DATA) as usize];
            return Some(String::from(""));
        }
    }

    pub fn get_attribute_value(&mut self, index: i32) -> String {
        let tp = self.get_attribute_value_type(index);
        let data = self.get_attribute_value_data(index);
        match tp {
            TYPE_STRING => self.get_attribute_value1(index).unwrap_or(String::from("")),
            TYPE_ATTRIBUTE => format!("?{}{:X}", get_package(data), data),
            TYPE_REFERENCE => format!("@{}{:X}", get_package(data), data),
            TYPE_FLOAT => format!("{}", data),
            TYPE_INT_HEX => format!("0x{:08X}", data),
            TYPE_INT_BOOLEAN => if data != 0 { String::from("true") } else { String::from("false") },
            TYPE_DIMENSION => format!("{}{}", complex_to_float(data as i64), DIMENSION_UNITS[(data & COMPLEX_UNIT_MASK) as usize]),
            TYPE_FRACTION => format!("{}{}", complex_to_float(data as i64), FRACTION_UNITS[(data & COMPLEX_UNIT_MASK) as usize]),
            TYPE_FIRST_COLOR_INT..=TYPE_LAST_COLOR_INT => format!("0x{:08X}", data),
            TYPE_FIRST_INT..=TYPE_LAST_INT => format!("{}", data),
            _ => format!("<0x{:X}, type 0x{:b}>", data, tp)
        }
    }
    fn get_attribute_offset(&self, index: i32) -> i32 {
        if self.event != START_TAG {
            panic!("Current event is not START_TAG.")
        }
        let offset = index * 5;
        if offset >= self.attributes.len() as i32 {
            panic!("Invalid attribute index.")
        }
        offset
    }
}


fn get_package(id: i32) -> &'static str {
    if id >> 24 == 1 {
        "android:"
    } else {
        ""
    }
}

fn complex_to_float(complex: i64) -> f64 {
    (complex & 0xFFFFFF00) as f64 * RADIX_MULTS[((complex >> 4) & 3) as usize]
}

