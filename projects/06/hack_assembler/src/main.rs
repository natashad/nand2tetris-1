use std::io;
use std::io::Read;
use std::io::Write;
use std::fs::File;
use std::env;

extern crate getopts;
use getopts::Options;

pub mod parser;
pub mod code;
pub mod symbol_table;

fn main() {

    ///////////////////////////////////////
    // Read and parse command line args //
    /////////////////////////////////////

    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optopt("o", "", "set output file name", "NAME");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    let in_fname = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        println!("Incorrect usage.");
        return;
    };
    let out_fname = match matches.opt_str("o") {
        Some(name) => name,
        None => in_fname.replace(".asm", ".hack")
    };

    let mut symbol_table = symbol_table::SymbolTable::new();

    let asm = read_source(in_fname);

    parser::parse_symbols(asm.clone(), &mut symbol_table);
    let instructions = parser::parse_asm(asm.clone(), &mut symbol_table);

    let mut it = instructions.iter();
    let mut output = String::new();
    loop {
        match it.next() {
            Some(x) =>  {
                output.push_str(&code::Code::new(x).to_string());
                output.push_str("\n");
            }
            None => break
        }
    }

    write_out(out_fname, output).ok().expect("failed to write to file");
}

/////////////////
// IO helpers //
///////////////

fn read_source(filename: String) -> String {
    let mut str = String::new();
    File::open(filename).unwrap().read_to_string(&mut str).unwrap();
    str
}

fn write_out(filename: String, out: String) ->io::Result<()> {
    let mut file = try!(File::create(filename));
    file.write_all((&out).as_bytes())

}