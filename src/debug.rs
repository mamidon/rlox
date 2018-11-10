use chunks::*;

pub fn dissassemble_chunk(chunk: &Chunk) {
    println!("=== {} ===", chunk.name());
    
    let mut bytecode = chunk.code();
    
    let mut offset = 0;
    while let (bytes_consumed, Some(instruction)) = Instruction::from_bytecode(&mut bytecode) {
        print!("{:04x?}\t{:>4}\t", offset, chunk.line(offset));
        println!("{}", dissassemble_instruction( chunk, instruction ));
        offset = offset + bytes_consumed;
    }
}

pub fn dissassemble_instruction(chunk: &Chunk, instruction: Instruction) -> String {
    match instruction {
        Instruction::Return => format!("RET"),
        Instruction::Constant(index) => format!("CONST  c[{:02x?}] '{:?}'", index, chunk.constant(index))
    }
}
