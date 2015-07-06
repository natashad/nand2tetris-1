use symbol_table;

#[derive(Debug)]
pub struct Instruction {
    pub instr_type: InstructionType,
    pub symbol: String,
    pub dest: String,
    pub comp: String,
    pub jump: String,
}

#[derive(Debug)]
pub enum InstructionType {
    // If starts with @ then A, C otherwise.
    A,
    C,
}

pub type InstructionList = Vec<Instruction>;


pub fn parse_symbols(input : String, table: &mut symbol_table::SymbolTable) {
    let lines: Vec<&str> = input.lines().collect();
    let mut it = lines.iter();

    let mut line_count = 0;
    loop {
        match it.next() {
            Some(raw_line) => {
                let mut line = strip_comments_and_trim(raw_line.to_string());
                if line.is_empty() {
                    continue;
                }
                if line.starts_with('(') {
                    assert!(line.remove(0) == '(');
                    assert!(line.pop().unwrap() == ')');
                    let sym = line.trim();
                    assert!(!table.contains(sym.to_string()));
                    table.insert(sym.to_string(), line_count);
                } else {
                    line_count+=1;
                }
            }
            None => break
        }
    }
}

pub fn parse_asm(input : String, table: &mut symbol_table::SymbolTable) -> InstructionList
{

    let mut instructions = InstructionList::new();
    let lines: Vec<&str> = input.lines().collect();
    let mut it = lines.iter();
    let mut variable_counter = 16;
    loop {
        match it.next() {
            Some(raw_line) => {
                let mut line = strip_comments_and_trim(raw_line.to_string());
                if line.is_empty() || line.starts_with('(') {
                    continue;
                }
                if line.starts_with('@') {
                    assert!(line.remove(0) == '@');
                    let mut val;
                    match line.parse::<i16>() {
                        Ok(x) => {
                            val = format!("{:015b}", x);
                        }
                        Err(_) => {
                            if table.contains(line.clone()) {
                                val = format!("{:015b}", &table.get(line));
                            }
                            else {
                                val = format!("{:015b}", variable_counter);
                                table.insert(line, variable_counter);
                                variable_counter += 1;
                            }
                        }
                    }
                    instructions.push(Instruction {
                        instr_type: InstructionType::A,
                        symbol: val,
                        dest: String::new(),
                        comp: String::new(),
                        jump: String::new(),
                    });
                } else {
                    let mut dest = String::new();
                    let mut comp = String::new();
                    let mut jmp = String::new();
                    let mut char_it = line.chars();
                    loop {
                        match char_it.next() {
                            Some(x) => {
                                if !line.contains('=') || x == '=' {
                                    if x != '=' {
                                        comp.push(x);
                                    }
                                    loop {
                                        match char_it.next() {
                                            Some(y) => {
                                                if y == ';' {
                                                    loop {
                                                        match char_it.next() {
                                                            Some(z) => {
                                                                jmp.push(z);
                                                            }
                                                            None => break,
                                                        }
                                                    }
                                                } else {
                                                    comp.push(y);
                                                }
                                            }
                                            None => break,
                                        }
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
                        comp: comp,
                        jump: jmp,
                    })
                }
            }
            None => break,
        }
    }
    return instructions;
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


