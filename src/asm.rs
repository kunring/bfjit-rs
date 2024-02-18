/* Naming uses Intel asm syntax order */
pub static MOV_AL_BYTE_PTR_RBX: &[u8] = &[0x8a, 0x03];
pub static TEST_AL_AL: &[u8] = &[0x84, 0xc0];
pub static MOV_RBX_RDI: &[u8] = &[0x48, 0x89, 0xfb];
pub static MOV_RSI_RBX: &[u8] = &[0x48, 0x89, 0xde];
pub static XOR_RAX_RAX: &[u8] = &[0x48, 0x31, 0xc0];
pub static JZ_INCOMPLETE: &[u8] = &[0x0f, 0x84];
pub static JNZ_INCOMPLETE: &[u8] = &[0x0f, 0x85];
pub static RET: &[u8] = &[0xc3];
pub static SYSCALL: &[u8] = &[0x0f, 0x05];

pub fn write_rbx(code: &mut Vec<u8>) {
    code.extend_from_slice(&mov_reg_u32(MovRegU32Registers::RAX, 1));
    code.extend_from_slice(&mov_reg_u32(MovRegU32Registers::RDI, 1));
    code.extend_from_slice(&mov_reg_u32(MovRegU32Registers::RDX, 1));
    code.extend_from_slice(MOV_RSI_RBX);
    code.extend_from_slice(SYSCALL);
}
#[repr(u8)]
pub enum MovRegU32Registers {
    RAX = 0xc0,
    RDI = 0xc7,
    RDX = 0xc2,
}
pub fn mov_reg_u32(register: MovRegU32Registers, data: u32) -> [u8; 7] {
    [
        0x48,
        0xc7,
        register as u8,
        data.to_le_bytes()[0],
        data.to_le_bytes()[1],
        data.to_le_bytes()[2],
        data.to_le_bytes()[3],
    ]
}
pub fn add_byte_ptr_rbx(data: u8) -> [u8; 3] {
    [0x80, 0x03, data.to_le_bytes()[0]]
}
pub fn sub_byte_ptr_rbx_(data: u8) -> [u8; 3] {
    [0x80, 0x2b, data.to_le_bytes()[0]]
}
pub fn add_rbx_u32(data: u32) -> [u8; 7] {
    [
        0x48,
        0x81,
        0xc3,
        data.to_le_bytes()[0],
        data.to_le_bytes()[1],
        data.to_le_bytes()[2],
        data.to_le_bytes()[3],
    ]
}
pub fn sub_rbx_u32(data: u32) -> [u8; 7] {
    [
        0x48,
        0x81,
        0xeb,
        data.to_le_bytes()[0],
        data.to_le_bytes()[1],
        data.to_le_bytes()[2],
        data.to_le_bytes()[3],
    ]
}
pub fn add_rbx_i8(data: i8) -> [u8; 4] {
    [0x48, 0x83, 0xc3, data.to_le_bytes()[0]]
}
pub fn sub_rbx_i8(data: i8) -> [u8; 4] {
    [0x48, 0x83, 0xeb, data.to_le_bytes()[0]]
}
