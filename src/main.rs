#[allow(dead_code)]
mod chunks;

#[allow(dead_code)]
mod debug;

#[allow(dead_code)]
mod runtime;

#[allow(dead_code)]
fn main() -> Result<(), i32> {
    use chunks::*;
    use debug::*;
    use runtime::*;
    
    let mut c= Chunk::create("test" );
    let index = c.add_constant(LoxValue::Number(6.14)).unwrap();
    let index2 = c.add_constant(LoxValue::Number(3.14)).unwrap();
    
    let instructions = [
        Instruction::Constant(index),
        Instruction::Constant(index2), 
        Instruction::Return
    ];
    
    instructions.iter()
        .flat_map(|i| i.as_bytecode())
        .for_each(|b| {
            let count = c.code().count();
            c.write( count, b);
        });
    
    dissassemble_chunk(&c );
    
    Ok(())
}
