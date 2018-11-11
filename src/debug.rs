use chunks::*;

pub fn dissassemble_chunk(chunk: &Chunk) {
    println!("=== {} ===", chunk.name());
    
    let mut bytecode = chunk.code();
    
    let mut offset = 0;
    while let (bytes_consumed, Some(instruction)) = Instruction::from_bytecode(&mut bytecode) {
        print!("{:04x?}\t{:>4}\t", offset, chunk.line(offset));
        disassemble_instruction(chunk, &instruction);
        offset = offset + bytes_consumed;
    }

    println!("=== {} ===", chunk.name());
    println!();
}

pub fn disassemble_instruction(chunk: &Chunk, instruction: &Instruction) {
    match instruction {
        Instruction::Return => println!("RET"),
        Instruction::Constant(index) => println!("CONST  c[{:02x?}] '{:?}'", index, chunk.constant(*index)),
        Instruction::Negate => println!("NEG    sp[-1]"),
        Instruction::Add => println!("ADD   sp[-1]  sp[-2]"),
        Instruction::Subtract => println!("SUB  sp[-2]  sp[-1]"),
        Instruction::Multiply => println!("MULT sp[-2]  sp[-1]"),
        Instruction::Divide => println!("DIV    sp[-2]  sp[-1]")
    }
}
