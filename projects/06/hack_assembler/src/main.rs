use std::io::Read;
use std::fs::File;

pub mod parser;

fn main() {
    let asm = read_source("../add/Add.asm".to_string());
    let instructions = parser::parse_asm(asm);
    println!("{:?}", instructions);
}

fn read_source(filename: String) -> String {
    let mut str = String::new();
    File::open(filename).unwrap().read_to_string(&mut str).unwrap();
    str
}