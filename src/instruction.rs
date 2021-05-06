use crate::opcode::OpCode;

#[derive(Debug)]
pub struct Instruction {
    pub bit: u16,
    pub first: u16,
    pub second: u16,
    pub third: u16,
    pub fourth: u16,
    pub opcode: OpCode
}

impl Instruction {
    pub fn new(bit: u16) -> Self {
        Instruction {
            bit,
            first: bit >> 12,
            second: bit >> 8 & 0xF,
            third: bit >> 4 & 0xF,
            fourth: bit & 0xF,
            opcode: OpCode::from(bit)
        }
    }
}