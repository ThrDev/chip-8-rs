use crate::instruction::Instruction;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum OpCode {
    CLS,
    RET,
    SYS,
    JMP,
    CALL,
    H3XNN,
    H4XNN,
    H5XY0,
    H6XNN,
    H7XNN,
    H8XY0,
    H8XY1,
    H8XY2,
    H8XY3,
    H8XY4,
    H8XY5,
    H8XY6,
    H8XY7,
    H8XYE,
    H9XY0,
    HANNN,
    HBNNN,
    HCXNN,
    HDXYN,
    HEX9E,
    HEXA1,
    HFX07,
    HFX0A,
    HFX15,
    HFX18,
    HFX1E,
    HFX29,
    HFX33,
    HFX55,
    HFX65,
    ILLEGAL,
}

impl From<u16> for OpCode {
    fn from(v: u16) -> Self {
        let first = v >> 12;
        let third = v >> 4 & 0xF;
        let fourth = v & 0xF;
        match v {
            0x00E0 => OpCode::CLS,
            0x00EE => OpCode::RET,
            x if first == 0x1 => OpCode::JMP,
            x if first == 0x2 => OpCode::CALL,
            x if first == 0x3 => OpCode::H3XNN,
            x if first == 0x4 => OpCode::H4XNN,
            x if first == 0x5 => OpCode::H5XY0,
            x if first == 0x6 => OpCode::H6XNN,
            x if first == 0x7 => OpCode::H7XNN,
            x if first == 0x8 => {
                match v {
                    y if fourth == 0x0 => OpCode::H8XY0,
                    y if fourth == 0x1 => OpCode::H8XY1,
                    y if fourth == 0x2 => OpCode::H8XY2,
                    y if fourth == 0x3 => OpCode::H8XY3,
                    y if fourth == 0x4 => OpCode::H8XY4,
                    y if fourth == 0x5 => OpCode::H8XY5,
                    y if fourth == 0x6 => OpCode::H8XY6,
                    y if fourth == 0x7 => OpCode::H8XY7,
                    y if fourth == 0xE => OpCode::H8XYE,
                    _ => OpCode::ILLEGAL,
                }
            },
            x if first == 0x9 => OpCode::H9XY0,
            x if first == 0xA => OpCode::HANNN,
            x if first == 0xB => OpCode::HBNNN,
            x if first == 0xC => OpCode::HCXNN,
            x if first == 0xD => OpCode::HDXYN,
            x if first == 0xE => {
                match v {
                    z if third == 0x9 && fourth == 0xE => OpCode::HEX9E,
                    z if third == 0xA && fourth == 0x1 => OpCode::HEXA1,
                    _ => OpCode::ILLEGAL,
                }
            },
            x if first == 0xF => {
                match v {
                    z if third == 0x0 && fourth == 0x7 => OpCode::HFX07,
                    z if third == 0x0 && fourth == 0xA => OpCode::HFX0A,
                    z if third == 0x1 && fourth == 0x5 => OpCode::HFX15,
                    z if third == 0x1 && fourth == 0x8 => OpCode::HFX18,
                    z if third == 0x1 && fourth == 0xE => OpCode::HFX1E,
                    z if third == 0x2 && fourth == 0x9 => OpCode::HFX29,
                    z if third == 0x3 && fourth == 0x3 => OpCode::HFX33,
                    z if third == 0x5 && fourth == 0x5 => OpCode::HFX55,
                    z if third == 0x6 && fourth == 0x5 => OpCode::HFX65,
                    _ => OpCode::ILLEGAL,
                }
            }
            _ => OpCode::ILLEGAL,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_opcode() {
        let opcode = OpCode::from(0x00E0);
        assert_eq!(OpCode::CLS, opcode);
        let opcode = OpCode::from(0x00EE);
        assert_eq!(OpCode::RET, opcode);
        let opcode = OpCode::from(0x1000);
        assert_eq!(OpCode::JMP, opcode);
    }

    #[test]
    fn test_crazy_opcode() {
        let opcode = OpCode::from(0xF907);
        assert_eq!(OpCode::HFX07, opcode);
    }
}