use std::slice;
use runtime::{LoxValue};

pub enum Instruction {
    Return,
    Constant(u8)
}

impl Instruction {
    pub fn as_bytecode(&self) -> Vec<u8> {
        let opcode = self.get_opcode();
        let mut bytecode: Vec<u8> = Vec::new();
        bytecode.push(opcode );
        
        match *self {
            Instruction::Return => { },
            Instruction::Constant(index) => bytecode.push(index )
        };
        
        return bytecode;
    }
    
    pub fn from_bytecode(bytecode: &mut slice::Iter<u8>) -> (usize, Option<Instruction>) {
        let opcode = match bytecode.next() {
            Some(opcode) => *opcode,
            None => return (0, None)
        };
        
        let instruction = match opcode {
            0 => (1, Some(Instruction::Return)),
            1 => {
                let (bytes_consumed, operands) = Instruction::get_single_operand(bytecode);
                (bytes_consumed + 1, operands.and_then(|o| Some(Instruction::Constant(o))))
            }
            _ => (1, None)
        };
        
        return instruction;
    }
    
    fn get_single_operand(bytecode: &mut slice::Iter<u8>) -> (usize, Option<u8>) {
        match bytecode.next() {
            Some(operand) => (1, Some(*operand)),
            None => (0, None)
        }
    }
    
    fn get_double_operands(bytecode: &mut slice::Iter<u8>) -> (usize, Option<(u8, u8)>) {
        let &left = match bytecode.next() {
            Some(operand) => operand,
            None => return (0, None)
        };
        
        let &right = match bytecode.next() {
            Some(operand) => operand,
            None => return (1, None)
        };
        
        return (2, Some((left, right)));
    }
    
    fn get_opcode(&self) -> u8 {
        match self {
            Instruction::Return => 0,
            Instruction::Constant(_) => 1
        }
    }
}


pub struct Chunk {
    name: String,
    lines: Vec<usize>,
    code: Vec<u8>,
    constants: Vec<LoxValue>,
}

impl Chunk {
    pub fn create(name: &str) -> Chunk {
        Chunk {
            name: name.to_owned(),
            lines: Vec::new(),
            code: Vec::new(),
            constants: Vec::new()
        }
    }
    
    pub fn write(&mut self, line: usize, byte: u8) {
        self.lines.push(line );
        self.code.push(byte );
    }
    
    pub fn add_constant(&mut self, constant: LoxValue) -> Option<u8> {
        if self.constants.len() >= (!0 as u8) as usize {
            return None;
        }
        
        self.constants.push(constant );
        return Some((self.constants.len() - 1) as u8);
    }
    
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
    
    pub fn code(&self) -> slice::Iter<u8> {
        self.code.iter()
    }
    
    pub fn line(&self, offset: usize) -> usize {
        self.lines[offset]
    }
    
    pub fn constant(&self, index: u8) -> &LoxValue {
        &self.constants[index as usize]
    }
    
    pub fn constants(&self) -> slice::Iter<LoxValue> {
        self.constants.iter()
    }
}
