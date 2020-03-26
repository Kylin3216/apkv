use crate::xml::parser::AXmlParser;
use crate::xml::constants::*;
use std::ops::Add;
use std::io::Read;

mod xml;

pub fn parse_android_manifest<T: Read>(file: T) -> Result<String> {
    let mut parser = AXmlParser::new();
    parser.open(file)?;
    let mut indent = String::with_capacity(10);
    let indent_step = "   ";
    let mut result = String::new();
    loop {
        let tp = parser.next();
        if tp == END_DOCUMENT {
            break;
        }
        match tp {
            START_DOCUMENT => {
                // println!("<?xml version=\"1.0\" encoding=\"utf-8\"?>")
                result = result.add("<?xml version=\"1.0\" encoding=\"utf-8\"?>");
                result = result.add("\r\n");
            }
            START_TAG => {
                // println!("{}<{}{}", indent, get_namespace_prefix(parser.get_prefix().unwrap_or(String::from(
                //     ""
                // ))), parser.get_name().unwrap());
                result = result.add(format!("{}<{}{}", indent, get_namespace_prefix(parser.get_prefix().unwrap_or(String::from(
                    ""
                ))), parser.get_name().unwrap()).as_str());
                result = result.add("\r\n");
                indent = indent.add(indent_step);
                let ncb = parser.get_namespace_count(parser.get_depth() - 1);
                let nc = parser.get_namespace_count(parser.get_depth());
                let mut i = ncb;
                while i != nc {
                    //println!("{}xmlns:{}=\"{}\"", indent, parser.get_namespace_prefix(i as i32).unwrap_or(String::from("")), parser.get_namespace_uri(i as i32).unwrap_or(String::from("")));
                    result = result.add(format!("{}xmlns:{}=\"{}\"", indent, parser.get_namespace_prefix(i as i32).unwrap_or(String::from("")), parser.get_namespace_uri(i as i32).unwrap_or(String::from(""))).as_str());
                    result = result.add("\r\n");
                    i += 1;
                }
                let mut j: i32 = 0;
                while j != parser.get_attribute_count() as i32 {
                    //println!("{}{}{}=\"{}\"", indent, get_namespace_prefix(parser.get_attribute_prefix(j).unwrap_or(String::from(""))), parser.get_attribute_name(j).unwrap_or(String::from("")), parser.get_attribute_value(j));
                    result = result.add(format!("{}{}{}=\"{}\"", indent, get_namespace_prefix(parser.get_attribute_prefix(j).unwrap_or(String::from(""))), parser.get_attribute_name(j).unwrap_or(String::from("")), parser.get_attribute_value(j)).as_str());
                    result = result.add("\r\n");
                    j += 1;
                }
                //println!("{}>", indent);
                result = result.add(format!("{}>", indent).as_str());
                result = result.add("\r\n");
            }
            END_TAG => {
                indent.truncate(indent.len() - indent_step.len());
                //println!("{}</{}{}>", indent, get_namespace_prefix(parser.get_prefix().unwrap_or(String::from(""))), parser.get_name().unwrap_or(String::from("")));
                result = result.add(format!("{}</{}{}>", indent, get_namespace_prefix(parser.get_prefix().unwrap_or(String::from(""))), parser.get_name().unwrap_or(String::from(""))).as_str());
                result = result.add("\r\n");
            }
            TEXT => {
                //println!("{}{}", indent, parser.get_text().unwrap_or(String::from("")));
                result = result.add(format!("{}{}", indent, parser.get_text().unwrap_or(String::from(""))).as_str());
                result = result.add("\r\n");
            }
            _ => ()
        }
    }
    Ok(result)
}

pub fn get_version(xml: String) -> String {
    let tmp: Vec<_> = xml.split("\r\n").collect();
    let mut version_code_str: Option<String> = None;
    let mut version_name_str: Option<String> = None;
    for str in tmp {
        if str.contains("versionCode") {
            version_code_str = Some(String::from(str));
        } else if str.contains("versionName") {
            version_name_str = Some(String::from(str));
        }
    }
    println!("{}",version_code_str.unwrap().replace("android:",""));
    println!("{}",version_name_str.unwrap().replace("android:",""));
    String::from("")
}


fn get_namespace_prefix(prefix: String) -> String {
    if prefix.len() == 0 {
        String::from("")
    } else {
        String::from(prefix).add(":")
    }
}

