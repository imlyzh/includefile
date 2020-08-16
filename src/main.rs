extern crate regex;

use std::convert::identity;
use std::env;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use regex::Regex;

const HELP_STR: &str = "参数应为target path";

fn main() {
    let mut args = env::args();

    args.next().map_or_else(|| { println!("{}", HELP_STR); }, |_|{});

    let target = args
        .next()
        .map_or_else(|| "../target.hpp".to_string(), identity);
    let path = args
        .next().map_or_else(|| "./".to_string(), identity);

    println!("target: {}, path: {}", target, path);

    let regex = Regex::new(r"[.](hpp|h)$").unwrap();

    let paths = fs::read_dir(path).unwrap();

    File::create(target.clone()).unwrap();
    let mut file = OpenOptions::new()
            .append(true).open(target).unwrap();

    for (counter, path) in paths.enumerate() {
        if let Ok(d) = path {
            if let Ok(f) = d.file_type() {
                if f.is_file()
                && regex.is_match(d.file_name().to_str().unwrap()) {
                    let filename = d.path()
                        .file_name().unwrap()
                        .to_str().unwrap()
                        .to_string();
                     
                    println!("{}: {}", counter, filename);
                    file.write(format!("#include <{}>\n", filename).as_bytes())
                        .unwrap();
                }
            }
        }
    }
}
