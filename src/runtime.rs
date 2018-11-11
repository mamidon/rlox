use std::fmt::{Debug, Formatter, Result};
use chunks::*;
use debug::*;

#[derive(Clone, Copy)]
pub enum LoxValue {
    Number(f64),
    String
}

pub enum ExecutionResult {
    Ok,
    StaticError,
    RuntimeError
}

pub struct VirtualMachine {
    ip: usize,
    stack: Vec<LoxValue>,
    diagnostics_enabled: bool
}

impl VirtualMachine {
    pub fn create() -> VirtualMachine {
        VirtualMachine {
            ip: 0,
            stack: Vec::new(),
            diagnostics_enabled: false
        }
    }
    
    pub fn enable_diagnostics(&mut self) {
        self.diagnostics_enabled = true;
    }
    
    pub fn run(&mut self, chunk: &Chunk) -> ExecutionResult {
        self.ip = 0;
        let mut bytecode = chunk.code();
        
        if self.diagnostics_enabled {
            dissassemble_chunk(chunk);
        }
        
        while let (consumed, Some(instruction)) = Instruction::from_bytecode(&mut bytecode) {
            self.ip += consumed;
            
            if self.diagnostics_enabled {
                match self.stack.last() {
                    Some(value) => println!("sp[{}] => '{:?}'", self.stack.len() - 1, value),
                    None => println!("sp[0] => 'nil'")
                }
                
                disassemble_instruction(chunk, &instruction);
            }
            
            match instruction {
                Instruction::Return => { 
                    let popped = self.pop();
                    match popped {
                        Some(value) => println!("sp[0] => '{:?}'", value),
                        None => println!("sp[0] => 'nil'")
                    };
                    return ExecutionResult::Ok;
                },
                Instruction::Constant(index) => { 
                    self.push(chunk.constant(index).clone()); 
                },
                Instruction::Negate => {
                    match self.pop() {
                        Some(value) => self.push(value.negate()),
                        None => return ExecutionResult::RuntimeError
                    }
                },
                Instruction::Add => {
                    match self.pop() {
                        Some(right) => {
                            match self.pop() {
                                Some(left) => self.push(left.add(&right)),
                                None => return ExecutionResult::RuntimeError
                            }
                        },
                        None => return ExecutionResult::RuntimeError
                    }
                },
                Instruction::Subtract => {
                    match self.pop() {
                        Some(right) => {
                            match self.pop() {
                                Some(left) => self.push(left.subtract(&right)),
                                None => return ExecutionResult::RuntimeError
                            }
                        },
                        None => return ExecutionResult::RuntimeError
                    }
                },
                Instruction::Multiply => {
                    match self.pop() {
                        Some(right) => {
                            match self.pop() {
                                Some(left) => self.push(left.multiply(&right)),
                                None => return ExecutionResult::RuntimeError
                            }
                        },
                        None => return ExecutionResult::RuntimeError
                    }
                }, 
                Instruction::Divide => {
                    match self.pop() {
                        Some(right) => {
                            match self.pop() {
                                Some(left) => self.push(left.divide(&right)),
                                None => return ExecutionResult::RuntimeError
                            }
                        },
                        None => return ExecutionResult::RuntimeError
                    }
                }
            }
        }
        
        return ExecutionResult::RuntimeError
    }
    
    fn pop(&mut self) -> Option<LoxValue> {
        self.stack.pop()
    }
    
    fn push(&mut self, value: LoxValue) {
        self.stack.push(value)
    }
}

impl LoxValue {
    pub fn negate(&self) -> LoxValue {
        if let LoxValue::Number(value) = self {
            return LoxValue::Number(-*value);
        }
        
        panic!();
    }
    
    pub fn add(&self, other: &LoxValue) -> LoxValue {
        if let (LoxValue::Number(left), LoxValue::Number(right)) = (self, other) {
            return LoxValue::Number(left + right);
        }
        
        panic!();
    }

    pub fn subtract(&self, other: &LoxValue) -> LoxValue {
        if let (LoxValue::Number(left), LoxValue::Number(right)) = (self, other) {
            return LoxValue::Number(left - right);
        }

        panic!();
    }

    pub fn multiply(&self, other: &LoxValue) -> LoxValue {
        if let (LoxValue::Number(left), LoxValue::Number(right)) = (self, other) {
            return LoxValue::Number(left * right);
        }

        panic!();
    }

    pub fn divide(&self, other: &LoxValue) -> LoxValue {
        if let (LoxValue::Number(left), LoxValue::Number(right)) = (self, other) {
            return LoxValue::Number(left / right);
        }

        panic!();
    }
}

impl Debug for LoxValue {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            LoxValue::Number(foo) => write!(f, "{}", foo),
            _ => write!(f, "")
        }
    }
}