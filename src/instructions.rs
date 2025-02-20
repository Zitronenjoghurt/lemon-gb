use crate::enums::parameter_groups::{JumpCondition, R16Stack, R16, R8};

pub enum Instruction {
    /// Add the specified register to register A
    Add(R8),
    /// Unconditional jump to the address specified in the HL register
    JpHL,
    /// Unconditional jump to the address specified in the following 2 bytes
    JpImm,
    /// Conditional jump to the address specified in the following 2 bytes
    JpCondImm16(JumpCondition),
    /// Loads the following 2 bytes into the specified register
    LoadR16Imm16(R16),
    /// Does nothing, will stall a cycle
    Nop,
    /// Pop 2 bytes from the stack to the specified register
    PopR16(R16Stack),
    /// Push 2 bytes from the specified register to the stack
    PushR16(R16Stack),
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
            0b0000_0000 => Instruction::Nop,                   // 0x00
            0b0000_0001 => Instruction::LoadR16Imm16(R16::BC), // 0x01
            0b0001_0001 => Instruction::LoadR16Imm16(R16::DE), // 0x11
            0b0010_0001 => Instruction::LoadR16Imm16(R16::HL), // 0x21
            0b0011_0001 => Instruction::LoadR16Imm16(R16::SP), // 0x31
            0b1000_0000 => Instruction::Add(R8::B),            // 0x80
            0b1000_0001 => Instruction::Add(R8::C),            // 0x81
            0b1000_0010 => Instruction::Add(R8::D),            // 0x82
            0b1000_0011 => Instruction::Add(R8::E),            // 0x83
            0b1000_0100 => Instruction::Add(R8::H),            // 0x84
            0b1000_0101 => Instruction::Add(R8::L),            // 0x85
            0b1000_0110 => Instruction::Add(R8::HL),           // 0x86
            0b1000_0111 => Instruction::Add(R8::A),            // 0x87
            0b1100_0001 => Instruction::PopR16(R16Stack::BC),  // 0xC1
            0b1100_0010 => Instruction::JpCondImm16(JumpCondition::NotZero), // 0xC2
            0b1100_0011 => Instruction::JpImm,                 // 0xC3
            0b1100_0101 => Instruction::PushR16(R16Stack::BC), // 0xC5
            0b1100_1010 => Instruction::JpCondImm16(JumpCondition::Zero), // 0xCA
            0b1101_0001 => Instruction::PopR16(R16Stack::DE),  // 0xD1
            0b1101_0010 => Instruction::JpCondImm16(JumpCondition::NotCarry), // 0xD2
            0b1101_0101 => Instruction::PushR16(R16Stack::DE), // 0xD5
            0b1101_1010 => Instruction::JpCondImm16(JumpCondition::Carry), // 0xDA
            0b1110_0001 => Instruction::PopR16(R16Stack::HL),  // 0xE1
            0b1110_0101 => Instruction::PushR16(R16Stack::HL), // 0xE5
            0b1110_1001 => Instruction::JpHL,                  // 0xE9
            0b1111_0001 => Instruction::PopR16(R16Stack::AF),  // 0xF1
            0b1111_0101 => Instruction::PushR16(R16Stack::AF), // 0xF5
            _ => panic!("Unknown unprefixed instruction byte: {:x}", byte),
        }
    }

    pub fn from_byte_prefixed(byte: u8) -> Self {
        match byte {
            _ => panic!("Unknown prefixed instruction byte: {:x}", byte),
        }
    }
}
