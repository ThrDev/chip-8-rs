use crate::instruction::Instruction;
use crate::register::Register;

pub struct Stack {
    pub i: u16,
    pub counter: u16,
    pub registers: Register,
    pub call_stack: Vec<u16>,
    pub memory: Vec<u8>,
}

impl Stack {
    pub fn get_next_instruction(&mut self) -> Option<Instruction> {
        if (self.counter + 2) as usize >= self.memory.len() {
            return None
        }

        let x = &self.memory[self.counter as usize..(self.counter + 2) as usize];
        let bit = ((x[0] as u16) << 8) | x[1] as u16;
        if bit == 0x00000000 {
            return None
        }
        self.counter += 2;

        return Some(Instruction::new(bit))
    }
}