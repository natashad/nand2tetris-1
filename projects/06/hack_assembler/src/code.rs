use std::collections::HashMap;


use parser;

pub struct Code {
    pub instruction : Instruction
}

pub enum Instruction {
    A(String),
    C(String, String, String),
}

impl Code {
    pub fn new( sym : &parser::Instruction) -> Code {

        let mut compMap = HashMap::<String, String>::new();
        compMap.insert("0".to_string(), "0101010".to_string());
        compMap.insert("1".to_string(), "0111111".to_string());
        compMap.insert("-1".to_string(), "0111010".to_string());
        compMap.insert("D".to_string(), "0001100".to_string());
        compMap.insert("A".to_string(), "0110000".to_string());
        compMap.insert("!D".to_string(), "0001101".to_string());
        compMap.insert("!A".to_string(), "0110001".to_string());
        compMap.insert("-D".to_string(), "0001111".to_string());
        compMap.insert("-A".to_string(), "0110011".to_string());
        compMap.insert("D+1".to_string(), "0011111".to_string());
        compMap.insert("A+1".to_string(), "0110111".to_string());
        compMap.insert("D-1".to_string(), "0001110".to_string());
        compMap.insert("A-1".to_string(), "0110010".to_string());
        compMap.insert("D+A".to_string(), "0000010".to_string());
        compMap.insert("D-A".to_string(), "0010011".to_string());
        compMap.insert("A-D".to_string(), "0000111".to_string());
        compMap.insert("D&A".to_string(), "0000000".to_string());
        compMap.insert("D|A".to_string(), "0010101".to_string());
        compMap.insert("M".to_string(), "1110000".to_string());
        compMap.insert("!M".to_string(), "1110001".to_string());
        compMap.insert("-M".to_string(), "1110011".to_string());
        compMap.insert("M+1".to_string(), "1110111".to_string());
        compMap.insert("M-1".to_string(), "1110010".to_string());
        compMap.insert("D+M".to_string(), "1000010".to_string());
        compMap.insert("D-M".to_string(), "1010011".to_string());
        compMap.insert("M-D".to_string(), "1000111".to_string());
        compMap.insert("D&M".to_string(), "1000000".to_string());
        compMap.insert("D|M".to_string(), "1010101".to_string());


        let mut jumpMap = HashMap::<String, String>::new();
        jumpMap.insert("null".to_string(), "000".to_string());
        jumpMap.insert("JGT".to_string(), "001".to_string());
        jumpMap.insert("JEQ".to_string(), "010".to_string());
        jumpMap.insert("JGE".to_string(), "011".to_string());
        jumpMap.insert("JLT".to_string(), "100".to_string());
        jumpMap.insert("JNE".to_string(), "101".to_string());
        jumpMap.insert("JLE".to_string(), "110".to_string());
        jumpMap.insert("JMP".to_string(), "111".to_string());


        match sym.instr_type {
            parser::InstructionType::A =>
                Code {instruction : Instruction::A( sym.symbol.clone() )},
            parser::InstructionType::C => {
                Code { instruction:
                    Instruction::C(dest(sym.dest.clone()), comp(sym.comp.clone(), compMap), jump(sym.jump.clone(), jumpMap)) }
            }
        }

    }

    pub fn to_string(&self) -> String {
        match self.instruction {
            Instruction::A(ref val) => format!("0{}", val),
            Instruction::C(ref dest, ref comp, ref jmp) => format!("111{}{}{}", comp, dest, jmp)
        }
    }

}

pub fn jump(sym : String, map: HashMap<String, String>) -> String {
    if sym.is_empty() {
        return "000".to_string();
    }
    assert!(map.contains_key(&sym));

    (*map.get(&sym).unwrap()).clone()

}

pub fn comp(sym: String, map :HashMap<String,String>) -> String {
    assert!(!sym.is_empty());
    assert!(map.contains_key(&sym));

    (*map.get(&sym).unwrap()).clone()

}

pub fn dest(sym : String) -> String {
    if sym.is_empty() {
        return "000".to_string();
    }
    let mut d = String::new();
    if sym.contains('A') {
        d.push('1');
    } else {
        d.push('0');
    }

    if sym.contains('M') {
        d.push('1');
    } else {
        d.push('0');
    }

    if sym.contains('D') {
        d.push('1');
    } else {
        d.push('0');
    }

    return d;

}
