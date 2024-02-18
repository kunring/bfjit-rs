mod executable_memory;

mod asm;

mod lexer;
use lexer::Token;

fn compile(intermediate: Vec<lexer::Token>) -> Vec<u8> {
    let mut code: Vec<u8> = Vec::new();
    code.extend_from_slice(asm::MOV_RBX_RDI);
    let mut jz_backpatch_positions: std::collections::HashMap<usize, usize> =
        std::collections::HashMap::new();
    for t in intermediate {
        match t {
            Token::Forwards(amt) => match i8::try_from(amt) {
                Ok(converted) => code.extend_from_slice(&asm::add_rbx_i8(converted)),
                Err(_) => code.extend_from_slice(&asm::add_rbx_u32(amt)),
            },
            Token::Backwards(amt) => match i8::try_from(amt) {
                Ok(converted) => code.extend_from_slice(&asm::sub_rbx_i8(converted)),
                Err(_) => code.extend_from_slice(&asm::sub_rbx_u32(amt)),
            },
            Token::Increment(amt) => code.extend_from_slice(&asm::add_byte_ptr_rbx(amt)),
            Token::Decrement(amt) => code.extend_from_slice(&asm::sub_byte_ptr_rbx_(amt)),
            Token::Print => asm::write_rbx(&mut code),
            Token::LoopStart(id) => {
                code.extend_from_slice(asm::XOR_RAX_RAX);
                code.extend_from_slice(asm::MOV_AL_BYTE_PTR_RBX);
                code.extend_from_slice(asm::TEST_AL_AL);
                code.extend_from_slice(asm::JZ_INCOMPLETE);
                jz_backpatch_positions.insert(id, code.len());
                // This temporarily value for the jump destination will be backpatched when codegen
                // is ending the current loop. The destination address is unknown before this.
                code.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);
            }
            Token::LoopEnd(id) => {
                code.extend_from_slice(asm::XOR_RAX_RAX);
                code.extend_from_slice(asm::MOV_AL_BYTE_PTR_RBX);
                code.extend_from_slice(asm::TEST_AL_AL);
                code.extend_from_slice(asm::JNZ_INCOMPLETE);
                let src_position = jz_backpatch_positions
                    .remove(&id)
                    .expect("looppi päin persettä???");
                let distance_to_start: i32 = src_position as i32 - code.len() as i32;
                let jump_amt_buf = distance_to_start.to_le_bytes();
                code.extend_from_slice(&jump_amt_buf);
                code.splice(
                    src_position..src_position + 4,
                    (-distance_to_start).to_le_bytes(),
                );
            }
        }
    }
    code.extend_from_slice(&asm::mov_reg_u32(asm::MovRegU32Registers::RAX, 0));
    code.extend_from_slice(asm::RET);
    code
}

fn get_executable_memory(instructions: Vec<u8>) -> executable_memory::ExecutableMemory {
    executable_memory::ExecutableMemory::try_from(instructions.as_slice()).expect("mmap failed")
}

fn execute(mem_size: usize, code_input: &str) {
    let mut jit_mem: Vec<u8> = vec![0; mem_size];

    let tokenised = lexer::parse(code_input);
    println!("tokenisation:\n{:?}", tokenised);
    let jit_code = compile(tokenised);
    println!("jit machine code hex dump:");
    jit_code.iter().for_each(|val| print!("{:02x} ", val));
    println!("\nmachine code length: {} bytes\n", jit_code.len());
    let vmem = get_executable_memory(jit_code);
    println!("jit stdout:");
    let jit_return = vmem.execute(&mut jit_mem);
    println!("");
    println!("jit return value: {}", jit_return);
    println!("jit memdump: {:?}", jit_mem);
}

fn main() {
    let usage = "Usage: `bfjit-rs [memory size in bytes] [code]`";
    let mut args = std::env::args().skip(1);
    let mem_size = usize::from_str_radix(&args.next().expect(usage), 10).expect("unable to parse mem size");
    let code_input = &args.next().expect(usage);
    execute(mem_size, code_input);
}
