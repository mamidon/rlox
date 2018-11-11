#[allow(dead_code)]
mod chunks;

#[allow(dead_code)]
mod debug;

#[allow(dead_code)]
mod runtime;

#[allow(dead_code)]
fn main() -> Result<(), i32> {
    use chunks::*;
    use runtime::*;
    
    let mut c= Chunk::create("test");
    let index = c.add_constant(LoxValue::Number(3.0)).unwrap();
    let index2 = c.add_constant(LoxValue::Number(7.0)).unwrap();
    
    let instructions = [
        Instruction::Constant(index),
        Instruction::Constant(index2),
        Instruction::Negate,
        Instruction::Multiply,
        Instruction::Return
    ];
    
    instructions.iter()
        .flat_map(|i| i.as_bytecode())
        .for_each(|b| {
            let count = c.code().count();
            c.write( count, b);
        });
    
    let mut vm = VirtualMachine::create();
    vm.enable_diagnostics();
    
    let outcome = vm.run(&c);
    
    match outcome {
        ExecutionResult::Ok => { println!("Execution completed!"); },
        ExecutionResult::StaticError(message) => { println!("{}", message); },
        ExecutionResult::RuntimeError(message) => { println!("{}", message); },
    }
    
    Ok(())
}
