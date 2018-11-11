use std::fmt::{Debug, Formatter, Result};
use chunks::*;
use debug::*;

#[derive(Clone, Copy)]
pub enum LoxValue {
    Number(f64),
    String
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum ExecutionResult {
    Ok,
    StaticError(&'static str),
    RuntimeError(&'static str)
}

pub struct VirtualMachine {
    ip: usize,
    stack: Vec<LoxValue>,
    diagnostics_enabled: bool,
    failure: Option<ExecutionResult>
}

impl VirtualMachine {
    pub fn create() -> VirtualMachine {
        VirtualMachine {
            ip: 0,
            stack: Vec::new(),
            diagnostics_enabled: false,
            failure: None
        }
    }
    
    pub fn enable_diagnostics(&mut self) {
        self.diagnostics_enabled = true;
    }
    
    pub fn run(&mut self, chunk: &Chunk) -> ExecutionResult {
        if let Some(previous_failure) = self.failure {
            previous_failure
        } else {
            let result = self.run_imp(chunk);
            
            if result != ExecutionResult::Ok {
                self.failure = Some(result);
            }
            
            return result;
        }
    }
    
    fn run_imp(&mut self, chunk: &Chunk) -> ExecutionResult {
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
                    if let Some(value) = self.pop() {
                        if let Some(computed) = value.negate() {
                            self.push(computed);
                        } else {
                            return ExecutionResult::RuntimeError("Only numbers can be negated");
                        }
                    } else {
                        return ExecutionResult::RuntimeError("Did not find 1 operand on the stack");
                    }
                },
                Instruction::Add => {
                    if let Some((left, right)) = self.pop_two() {
                        if let Some(computed)= left.add(&right) {
                            self.push(computed)
                        } else {
                            return ExecutionResult::RuntimeError("Only two numbers can be added");
                        }
                    } else {
                        return ExecutionResult::RuntimeError("Did not find 2 operands on the stack");
                    }
                },
                Instruction::Subtract => {
                    if let Some((left, right)) = self.pop_two() {
                        if let Some(computed)= left.subtract(&right) {
                            self.push(computed)
                        } else {
                            return ExecutionResult::RuntimeError("Only two numbers can be subtracted");
                        }
                    } else {
                        return ExecutionResult::RuntimeError("Did not find 2 operands on the stack");
                    }
                },
                Instruction::Multiply => {
                    if let Some((left, right)) = self.pop_two() {
                        if let Some(computed)= left.multiply(&right) {
                            self.push(computed)
                        } else {
                            return ExecutionResult::RuntimeError("Only two numbers can be multiplied");
                        }
                    } else {
                        return ExecutionResult::RuntimeError("Did not find 2 operands on the stack");
                    }
                }, 
                Instruction::Divide => {
                    if let Some((left, right)) = self.pop_two() {
                        if let Some(computed)= left.divide(&right) {
                            self.push(computed)
                        } else {
                            return ExecutionResult::RuntimeError("Only two numbers can be divided");
                        }
                    } else {
                        return ExecutionResult::RuntimeError("Did not find 2 operands on the stack");
                    }
                }
            }
        }
        
        return ExecutionResult::RuntimeError("Execution completed without a return statement");
    }
    
    fn pop(&mut self) -> Option<LoxValue> {
        self.stack.pop()
    }
    
    fn pop_two(&mut self) -> Option<(LoxValue, LoxValue)> {
        let right = self.pop();
        let left = self.pop();
        
        if let (Some(a), Some(b)) = (left, right) {
            return Some((a,b));
        } else {
            return None;
        }
    }
    
    fn push(&mut self, value: LoxValue) {
        self.stack.push(value)
    }
}

impl LoxValue {
    
    pub fn negate(&self) -> Option<LoxValue> {
        match *self {
            LoxValue::Number(value) => Some(LoxValue::Number(-value)),
            _ => None
        }
    }
    
    pub fn add(&self, other: &LoxValue) -> Option<LoxValue> {
        LoxValue::binary_numbers_action(self, other, &|left, right| left + right)
    }

    pub fn subtract(&self, other: &LoxValue) -> Option<LoxValue> {
        LoxValue::binary_numbers_action(self, other, &|left, right| left - right)
    }

    pub fn multiply(&self, other: &LoxValue) -> Option<LoxValue> {
        LoxValue::binary_numbers_action(self, other, &|left, right| left * right)
    }

    pub fn divide(&self, other: &LoxValue) -> Option<LoxValue> {
        LoxValue::binary_numbers_action(self, other, &|left, right| left / right)
    }
    
    fn binary_numbers_action(left: &LoxValue, right: &LoxValue, action: &Fn(f64, f64) -> f64)
        -> Option<LoxValue> {
        LoxValue::binary_numbers(left, right)
            .and_then(|tuple| Some(LoxValue::Number(action(tuple.0, tuple.1))))
    }

    fn binary_numbers(left: &LoxValue, right: &LoxValue) -> Option<(f64, f64)> {
        if let (LoxValue::Number(left_value), LoxValue::Number(right_value)) = (*left, *right) {
            Some((left_value, right_value))
        } else {
            None
        }
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