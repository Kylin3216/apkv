extern crate zip;

use std::fs::File;
use apkv::{parse_android_manifest, get_version};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let name = std::path::Path::new(&args[1]);
        match File::open(&name) {
            Ok(file) => {
                match zip::ZipArchive::new(file) {
                    Ok(mut archive) => {
                        for i in 0..archive.len() {
                            let file = archive.by_index(i).unwrap();
                            let out_path = file.sanitized_name();
                            let name = out_path.file_name().unwrap();
                            if name == "AndroidManifest.xml" {
                                match parse_android_manifest(file) {
                                    Ok(res) => {
                                       // println!("{}", res)
                                        get_version(res);
                                    },
                                    Err(e) => println!("{}", e)
                                };

                                break;
                            }
                        }
                    }
                    Err(e) => println!("{}", e)
                }
            }
            Err(e) => println!("{}", e)
        };
    } else {
        println!("请输入要解析的文件")
    }
}
