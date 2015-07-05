pub struct Parser {
    input : String,
    pos : usize,
}

pub struct InstructionList {
    instructions : Vec<Instruction>,
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
    Invalid
}

impl Parser {
    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn parse_instruction(&mut self) -> Instruction {
        let mut inst = Instruction {
            instr_type : InstructionType::Invalid,
            symbol : String::new(),
            dest : String::new(),
            cmd : String::new(),
            jump : String::new()
        };
        while !self.eof() {
            self.consume_whitespace();
            if self.next_char() == '@' {
                assert!(self.consume_char()=='@');
                inst.instr_type = InstructionType::A;
                inst.symbol = self.parse_symbol();
            } else {
                inst.instr_type = InstructionType::C;
                inst.dest = self.parse_dest();
                inst.cmd = self.parse_command();
                inst.jump = self.parse_jump();
            }
        }
        return inst;

    }

    fn parse_symbol(&mut self) -> String {
        let mut val = self.consume_till_newline();
        match val.parse::<u32>() {
            Ok(_) => u32::from_str_radix(&self.input, 2).unwrap().to_string(),
            Err(_) => String::new() //handle symbol
        }
    }

    fn parse_dest(&mut self) -> String {
        let mut val = self.consume_while(|c| c != '=');
        assert!(self.consume_char() == '=');
        return val;
    }

    fn parse_command(&mut self) -> String {
        //Instead, do a search for ';' in the string. pass in command at a time
        let mut val = self.consume_while(|c| c != ';' || c != '\n' || c != '/');
        val.trim();
        if self.next_char() == ';' {
            self.consume_char();
        }
        return val;
    }

    fn parse_jump(&mut self) -> String {
        String::new();
    }

    fn consume_till_newline(&mut self) -> String {
        let ret = self.consume_while(|c| c != '\n' || c != '/');
        self.consume_whitespace();
        return ret;
    }

    fn consume_whitespace(&mut self) -> String {
        self.consume_while(char::is_whitespace)
    }

    fn consume_comment(&mut self) {
        if self.next_char() == '/' {
            self.consume_till_newline();
        }
    }

    fn consume_while<F>(&mut self, f: F) -> String where F : Fn(char) -> bool {
        let mut ret = String::new();
        while !self.eof() && f(self.next_char()) {
            ret.push(self.consume_char());
        }
        return ret;
    }

    fn consume_char(&mut self) -> char {
        let mut it = self.input[self.pos..].char_indices();
        let (_, ret) = it.next().unwrap();
        let (next_pos, _) = it.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        return ret;
    }

    fn next_char(&mut self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }
}

impl InstructionList {
    fn has_more_instructions(&self) -> bool {
        self.pos >= self.instructions.len()
    }
}
