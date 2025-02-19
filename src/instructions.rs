use crate::enums::jump_condition::JumpCondition;
use crate::enums::register_8::Register8;

pub enum Instruction {
    Add(Register8),
    JpCondImm(JumpCondition),
}

impl Instruction {
    pub fn from_byte(byte: u8, prefixed: bool) -> Self {
        if prefixed {
            Instruction::from_byte_prefixed(byte)
        } else {
            Instruction::from_byte_unprefixed(byte)
        }
    }

    pub fn from_byte_unprefixed(byte: u8) -> Self {
        match byte {
            0b1000_0000 => Instruction::Add(Register8::B),  // 0x80
            0b1000_0001 => Instruction::Add(Register8::C),  // 0x81
            0b1000_0010 => Instruction::Add(Register8::D),  // 0x82
            0b1000_0011 => Instruction::Add(Register8::E),  // 0x83
            0b1000_0100 => Instruction::Add(Register8::H),  // 0x84
            0b1000_0101 => Instruction::Add(Register8::L),  // 0x85
            0b1000_0110 => Instruction::Add(Register8::HL), // 0x86
            0b1000_0111 => Instruction::Add(Register8::A),  // 0x87
            0b1100_0010 => Instruction::JpCondImm(JumpCondition::NotZero), // 0xC2
            0b1100_1010 => Instruction::JpCondImm(JumpCondition::Zero), // 0xCA
            0b1101_0010 => Instruction::JpCondImm(JumpCondition::NotCarry), // 0xD2
            0b1101_1010 => Instruction::JpCondImm(JumpCondition::Carry), // 0xDA
            _ => panic!("Unknown unprefixed instruction byte: {:x}", byte),
        }
    }

    pub fn from_byte_prefixed(byte: u8) -> Self {
        match byte {
            _ => panic!("Unknown prefixed instruction byte: {:x}", byte),
        }
    }
}
