use std::io::Read;
use std::fs::File;

pub mod parser;
pub mod code;

fn main() {
    let asm = read_source("../pong/PongL.asm".to_string());
    let instructions = parser::parse_asm(asm);
    //println!("{:?}", instructions);

    let mut it = instructions.iter();
    loop {
        match it.next() {
            Some(x) =>  println!("{}", code::Code::new(x).to_string()),
            None => break
        }
    }
}

fn read_source(filename: String) -> String {
    let mut str = String::new();
    File::open(filename).unwrap().read_to_string(&mut str).unwrap();
    str
}