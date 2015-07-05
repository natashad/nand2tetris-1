pub struct Parser {
    input : String,
    pos : usize,
}

pub struct Instruction {
    instr_type: InstructionType,
    symbol: String,
    dest: String,
    cmd: String,
    jump: String,
}

pub enum InstructionType {
    // If starts with 0 then A, if starts with 1 then C
    A,
    C,
}

pub type InstructionList = Vec<Instruction>;


pub fn parse_asm(input : String) -> InstructionList
{

    let mut instructions = InstructionList::new();
    let lines: Vec<&str> = input.lines().collect();
    let mut it = lines.iter();
    loop {
        match it.next() {
            Some(raw_line) => {
                let mut line = strip_comments_and_trim(raw_line.to_string());
                if (line.is_empty()) {
                    continue;
                }
                if line.starts_with('@') {
                    assert!(line.remove(0) == '@');
                    let mut val = String::new();
                    match val.parse::<u32>() {
                        Ok(_) => {
                            val = u32::from_str_radix(&line, 2).unwrap().to_string();
                        }
                        Err(_) => {
                            val = String::new() //handle symbol
                        }
                    }
                    instructions.push(Instruction {
                        instr_type: InstructionType::A,
                        symbol: val,
                        dest: String::new(),
                        cmd: String::new(),
                        jump: String::new(),
                    });
                } else {
                    let mut dest = String::new();
                    let mut cmd = String::new();
                    let mut jmp = String::new();
                    let mut char_it = line.chars();
                    loop {
                        match char_it.next() {
                            Some(x) => {
                                if (x == '=') {
                                    match char_it.next() {
                                        Some(y) => {
                                            if (y == ';') {
                                                match char_it.next() {
                                                    Some(z) => {
                                                        jmp.push(z);
                                                    }
                                                    None => break,
                                                }
                                            } else {
                                                cmd.push(y);
                                            }
                                        }
                                        None => break,
                                    }
                                } else {
                                    dest.push(x);
                                }
                            }
                            None => break,
                        }
                    }
                    instructions.push(Instruction {
                        instr_type: InstructionType::C,
                        symbol: String::new(),
                        dest: dest,
                        cmd: cmd,
                        jump: jmp,
                    })
                }
            }
            None => break,
        }
    }
    return instructions;
}

fn parse_symbol(line: String) -> String {
    String::new()
}

fn strip_comments_and_trim(line: String) -> String {
    let mut stripped = String::new();
    let mut it = line.chars();
    loop {
        match it.next() {
            Some(x) => {
                stripped.push(x);
                if x == '/' {
                    match it.next() {
                        Some(y) => {
                            if y == '/' {
                                stripped.pop();
                                break;
                            } else {
                                stripped.push(y)
                            }
                        }
                        None => break,
                    }
                }

            }
            None => break,
        }
    }
    return stripped.trim().to_string();
}


