use crate::enums::parameter_groups::{JumpCondition, R16Mem, R16Stack, R16, R8};
use crate::game_boy::components::cpu::PREFIX_INSTRUCTION_BYTE;
use std::error::Error;

#[derive(Debug, Default, Clone, PartialEq)]
pub enum Instruction {
    /// Add value from the specified register to the HL register
    AddHLR16(R16),
    /// Add the specified register to register A
    AddR8(R8),
    /// Complement register A => bitwise NOT
    ComplementA,
    /// Negates the carry flag
    ComplementCarryFlag,
    /// To be used after an addition or subtraction
    /// Will adjust the result in register A to be valid BCD (Binary-Coded Decimal)
    DAA,
    /// Decrement the specified register
    DecR8(R8),
    /// Decrement the specified register
    DecR16(R16),
    /// Disables interrupts immediately
    DisableInterrupts,
    /// Enables interrupts after the next instruction
    EnableInterrupts,
    /// Enter CPU low-power consumption mode until an interrupt occurs (with some interesting behavior though)
    Halt,
    /// Increment the specified register
    IncR8(R8),
    /// Increment the specified register
    IncR16(R16),
    /// Unconditional jump to the address specified in the HL register
    JpHL,
    /// Unconditional jump to the address specified in the following 2 bytes
    JpImm16,
    /// Conditional jump to the address specified in the following 2 bytes
    JpCondImm16(JumpCondition),
    /// Unconditional relative jump incrementing the program counter by the specified byte
    JrImm8,
    /// Conditional relative jump incrementing the program counter by the specified byte
    JrCondImm8(JumpCondition),
    /// Loads the value from the address stored in the provided register into register A
    /// For HL-, HL is decremented after the operation
    /// For HL+, HL is incremented after the operation
    LoadAR16(R16Mem),
    /// Loads the value in register A into the address stored in the provided register
    /// For HL-, HL is decremented after the operation
    /// For HL+, HL is incremented after the operation
    LoadR16A(R16Mem),
    /// Loads the following 2 bytes into the specified register
    LoadR16Imm16(R16),
    /// Loads the following byte into the specified register
    LoadR8Imm8(R8),
    /// Load the value from the register on the right into the register on the left
    LoadR8R8((R8, R8)),
    /// Load the value at the top of the stack into the address specified by the following 2 bytes
    LoadImm16SP,
    /// Does nothing, will stall a cycle
    #[default]
    Nop,
    /// Pop 2 bytes from the stack to the specified register
    PopR16(R16Stack),
    /// Push 2 bytes from the specified register to the stack
    PushR16(R16Stack),
    /// Return from a previous function call
    Return,
    /// Return from a previous function call if a certain condition is met
    ReturnCondition(JumpCondition),
    /// Return from a previous function call and enable interrupts
    ReturnEnableInterrupts,
    /// Rotate register A left by 1 bit, through the carry flag
    /// ```
    ///   ┏━ Flags ━┓ ┏━━━━━━━ A ━━━━━━━┓
    /// ┌─╂─   C   ←╂─╂─ b7 ← ... ← b0 ←╂─┐
    /// │ ┗━━━━━━━━━┛ ┗━━━━━━━━━━━━━━━━━┛ │
    /// └─────────────────────────────────┘
    /// ```
    RotateLeftA,
    /// Rotate register A right by 1 bit, through the carry flag
    /// ```
    ///   ┏━━━━━━━ A ━━━━━━━┓ ┏━ Flags ━┓
    /// ┌─╂→ b7 → ... → b0 ─╂─╂→   C   ─╂─┐
    /// │ ┗━━━━━━━━━━━━━━━━━┛ ┗━━━━━━━━━┛ │
    /// └─────────────────────────────────┘
    /// ```
    RotateRightA,
    /// Rotate register A left by 1 bit
    /// ```
    /// ┏━ Flags ━┓   ┏━━━━━━━ A ━━━━━━━┓
    /// ┃    C   ←╂─┬─╂─ b7 ← ... ← b0 ←╂─┐
    /// ┗━━━━━━━━━┛ │ ┗━━━━━━━━━━━━━━━━━┛ │
    ///             └─────────────────────┘
    /// ```
    RotateLeftCarryA,
    /// Rotate register A right by 1 bit
    /// ```
    ///   ┏━━━━━━━ A ━━━━━━━┓   ┏━ Flags ━┓
    /// ┌─╂→ b7 → ... → b0 ─╂─┬─╂→   C    ┃
    /// │ ┗━━━━━━━━━━━━━━━━━┛ │ ┗━━━━━━━━━┛
    /// └─────────────────────┘
    /// ```
    RotateRightCarryA,
    /// Sets the carry flag to 1
    SetCarryFlag,
}

impl Instruction {
    pub fn from_byte(byte: u8, prefixed: bool) -> Result<Self, Box<dyn Error>> {
        if prefixed {
            Self::from_byte_prefixed(byte)
        } else {
            Self::from_byte_unprefixed(byte)
        }
    }

    pub fn from_byte_unprefixed(byte: u8) -> Result<Self, Box<dyn Error>> {
        match byte {
            0b0000_0000 => Ok(Instruction::Nop),                   // 0x00
            0b0000_0001 => Ok(Instruction::LoadR16Imm16(R16::BC)), // 0x01
            0b0000_0010 => Ok(Instruction::LoadR16A(R16Mem::BC)),  // 0x02
            0b0000_0011 => Ok(Instruction::IncR16(R16::BC)),       // 0x03
            0b0000_0100 => Ok(Instruction::IncR8(R8::B)),          // 0x04
            0b0000_0101 => Ok(Instruction::DecR8(R8::B)),          // 0x05
            0b0000_0110 => Ok(Instruction::LoadR8Imm8(R8::B)),     // 0x06
            0b0000_0111 => Ok(Instruction::RotateLeftCarryA),      // 0x07
            0b0000_1000 => Ok(Instruction::LoadImm16SP),           // 0x08
            0b0000_1001 => Ok(Instruction::AddHLR16(R16::BC)),     // 0x09
            0b0000_1010 => Ok(Instruction::LoadAR16(R16Mem::BC)),  // 0x0A
            0b0000_1011 => Ok(Instruction::DecR16(R16::BC)),       // 0x0B
            0b0000_1100 => Ok(Instruction::IncR8(R8::C)),          // 0x0C
            0b0000_1101 => Ok(Instruction::DecR8(R8::C)),          // 0x0D
            0b0000_1110 => Ok(Instruction::LoadR8Imm8(R8::C)),     // 0x0E
            0b0000_1111 => Ok(Instruction::RotateRightCarryA),     // 0x0F
            0b0001_0001 => Ok(Instruction::LoadR16Imm16(R16::DE)), // 0x11
            0b0001_0010 => Ok(Instruction::LoadR16A(R16Mem::DE)),  // 0x12
            0b0001_0011 => Ok(Instruction::IncR16(R16::DE)),       // 0x13
            0b0001_0100 => Ok(Instruction::IncR8(R8::D)),          // 0x14
            0b0001_0101 => Ok(Instruction::DecR8(R8::D)),          // 0x15
            0b0001_0110 => Ok(Instruction::LoadR8Imm8(R8::D)),     // 0x16
            0b0001_0111 => Ok(Instruction::RotateLeftA),           // 0x17
            0b0001_1000 => Ok(Instruction::JrImm8),                // 0x18
            0b0001_1001 => Ok(Instruction::AddHLR16(R16::DE)),     // 0x19
            0b0001_1010 => Ok(Instruction::LoadAR16(R16Mem::DE)),  // 0x1A
            0b0001_1011 => Ok(Instruction::DecR16(R16::DE)),       // 0x1B
            0b0001_1100 => Ok(Instruction::IncR8(R8::E)),          // 0x1C
            0b0001_1101 => Ok(Instruction::DecR8(R8::E)),          // 0x1D
            0b0001_1110 => Ok(Instruction::LoadR8Imm8(R8::E)),     // 0x1E
            0b0001_1111 => Ok(Instruction::RotateRightA),          // 0x1F
            0b0010_0000 => Ok(Instruction::JrCondImm8(JumpCondition::NotZero)), // 0x20
            0b0010_0001 => Ok(Instruction::LoadR16Imm16(R16::HL)), // 0x21
            0b0010_0010 => Ok(Instruction::LoadR16A(R16Mem::HLI)), // 0x22
            0b0010_0011 => Ok(Instruction::IncR16(R16::HL)),       // 0x23
            0b0010_0100 => Ok(Instruction::IncR8(R8::H)),          // 0x24
            0b0010_0101 => Ok(Instruction::DecR8(R8::H)),          // 0x25
            0b0010_0110 => Ok(Instruction::LoadR8Imm8(R8::H)),     // 0x26
            0b0010_0111 => Ok(Instruction::DAA),                   // 0x27
            0b0010_1000 => Ok(Instruction::JrCondImm8(JumpCondition::Zero)), // 0x28
            0b0010_1001 => Ok(Instruction::AddHLR16(R16::HL)),     // 0x29
            0b0010_1111 => Ok(Instruction::ComplementA),           // 0x2F
            0b0011_0000 => Ok(Instruction::JrCondImm8(JumpCondition::NotCarry)), // 0x30
            0b0010_1010 => Ok(Instruction::LoadAR16(R16Mem::HLI)), // 0x2A
            0b0010_1011 => Ok(Instruction::DecR16(R16::HL)),       // 0x2B
            0b0010_1100 => Ok(Instruction::IncR8(R8::L)),          // 0x2C
            0b0010_1101 => Ok(Instruction::DecR8(R8::L)),          // 0x2D
            0b0010_1110 => Ok(Instruction::LoadR8Imm8(R8::L)),     // 0x2E
            0b0011_0001 => Ok(Instruction::LoadR16Imm16(R16::SP)), // 0x31
            0b0011_0010 => Ok(Instruction::LoadR16A(R16Mem::HLD)), // 0x32
            0b0011_0011 => Ok(Instruction::IncR16(R16::SP)),       // 0x33
            0b0011_0100 => Ok(Instruction::IncR8(R8::HL)),         // 0x34
            0b0011_0101 => Ok(Instruction::DecR8(R8::HL)),         // 0x35
            0b0011_0110 => Ok(Instruction::LoadR8Imm8(R8::HL)),    // 0x36
            0b0011_0111 => Ok(Instruction::SetCarryFlag),          // 0x37
            0b0011_1000 => Ok(Instruction::JrCondImm8(JumpCondition::Carry)), // 0x38
            0b0011_1001 => Ok(Instruction::AddHLR16(R16::SP)),     // 0x39
            0b0011_1010 => Ok(Instruction::LoadAR16(R16Mem::HLD)), // 0x3A
            0b0011_1011 => Ok(Instruction::DecR16(R16::SP)),       // 0x3B
            0b0011_1100 => Ok(Instruction::IncR8(R8::A)),          // 0x3C
            0b0011_1101 => Ok(Instruction::DecR8(R8::A)),          // 0x3D
            0b0011_1110 => Ok(Instruction::LoadR8Imm8(R8::A)),     // 0x3E
            0b0011_1111 => Ok(Instruction::ComplementCarryFlag),   // 0x3F
            0b0100_0000 => Ok(Instruction::LoadR8R8((R8::B, R8::B))), // 0x40
            0b0100_0001 => Ok(Instruction::LoadR8R8((R8::B, R8::C))), // 0x41
            0b0100_0010 => Ok(Instruction::LoadR8R8((R8::B, R8::D))), // 0x42
            0b0100_0011 => Ok(Instruction::LoadR8R8((R8::B, R8::E))), // 0x43
            0b0100_0100 => Ok(Instruction::LoadR8R8((R8::B, R8::H))), // 0x44
            0b0100_0101 => Ok(Instruction::LoadR8R8((R8::B, R8::L))), // 0x45
            0b0100_0110 => Ok(Instruction::LoadR8R8((R8::B, R8::HL))), // 0x46
            0b0100_0111 => Ok(Instruction::LoadR8R8((R8::B, R8::A))), // 0x47
            0b0100_1000 => Ok(Instruction::LoadR8R8((R8::C, R8::B))), // 0x48
            0b0100_1001 => Ok(Instruction::LoadR8R8((R8::C, R8::C))), // 0x49
            0b0100_1010 => Ok(Instruction::LoadR8R8((R8::C, R8::D))), // 0x4A
            0b0100_1011 => Ok(Instruction::LoadR8R8((R8::C, R8::E))), // 0x4B
            0b0100_1100 => Ok(Instruction::LoadR8R8((R8::C, R8::H))), // 0x4C
            0b0100_1101 => Ok(Instruction::LoadR8R8((R8::C, R8::L))), // 0x4D
            0b0100_1110 => Ok(Instruction::LoadR8R8((R8::C, R8::HL))), // 0x4E
            0b0100_1111 => Ok(Instruction::LoadR8R8((R8::C, R8::A))), // 0x4F
            0b0101_0000 => Ok(Instruction::LoadR8R8((R8::D, R8::B))), // 0x50
            0b0101_0001 => Ok(Instruction::LoadR8R8((R8::D, R8::C))), // 0x51
            0b0101_0010 => Ok(Instruction::LoadR8R8((R8::D, R8::D))), // 0x52
            0b0101_0011 => Ok(Instruction::LoadR8R8((R8::D, R8::E))), // 0x53
            0b0101_0100 => Ok(Instruction::LoadR8R8((R8::D, R8::H))), // 0x54
            0b0101_0101 => Ok(Instruction::LoadR8R8((R8::D, R8::L))), // 0x55
            0b0101_0110 => Ok(Instruction::LoadR8R8((R8::D, R8::HL))), // 0x56
            0b0101_0111 => Ok(Instruction::LoadR8R8((R8::D, R8::A))), // 0x57
            0b0101_1000 => Ok(Instruction::LoadR8R8((R8::E, R8::B))), // 0x58
            0b0101_1001 => Ok(Instruction::LoadR8R8((R8::E, R8::C))), // 0x59
            0b0101_1010 => Ok(Instruction::LoadR8R8((R8::E, R8::D))), // 0x5A
            0b0101_1011 => Ok(Instruction::LoadR8R8((R8::E, R8::E))), // 0x5B
            0b0101_1100 => Ok(Instruction::LoadR8R8((R8::E, R8::H))), // 0x5C
            0b0101_1101 => Ok(Instruction::LoadR8R8((R8::E, R8::L))), // 0x5D
            0b0101_1110 => Ok(Instruction::LoadR8R8((R8::E, R8::HL))), // 0x5E
            0b0101_1111 => Ok(Instruction::LoadR8R8((R8::E, R8::A))), // 0x5F
            0b0110_0000 => Ok(Instruction::LoadR8R8((R8::H, R8::B))), // 0x60
            0b0110_0001 => Ok(Instruction::LoadR8R8((R8::H, R8::C))), // 0x61
            0b0110_0010 => Ok(Instruction::LoadR8R8((R8::H, R8::D))), // 0x62
            0b0110_0011 => Ok(Instruction::LoadR8R8((R8::H, R8::E))), // 0x63
            0b0110_0100 => Ok(Instruction::LoadR8R8((R8::H, R8::H))), // 0x64
            0b0110_0101 => Ok(Instruction::LoadR8R8((R8::H, R8::L))), // 0x65
            0b0110_0110 => Ok(Instruction::LoadR8R8((R8::H, R8::HL))), // 0x66
            0b0110_0111 => Ok(Instruction::LoadR8R8((R8::H, R8::A))), // 0x67
            0b0110_1000 => Ok(Instruction::LoadR8R8((R8::L, R8::B))), // 0x68
            0b0110_1001 => Ok(Instruction::LoadR8R8((R8::L, R8::C))), // 0x69
            0b0110_1010 => Ok(Instruction::LoadR8R8((R8::L, R8::D))), // 0x6A
            0b0110_1011 => Ok(Instruction::LoadR8R8((R8::L, R8::E))), // 0x6B
            0b0110_1100 => Ok(Instruction::LoadR8R8((R8::L, R8::H))), // 0x6C
            0b0110_1101 => Ok(Instruction::LoadR8R8((R8::L, R8::L))), // 0x6D
            0b0110_1110 => Ok(Instruction::LoadR8R8((R8::L, R8::HL))), // 0x6E
            0b0110_1111 => Ok(Instruction::LoadR8R8((R8::L, R8::A))), // 0x6F
            0b0111_0000 => Ok(Instruction::LoadR8R8((R8::HL, R8::B))), // 0x70
            0b0111_0001 => Ok(Instruction::LoadR8R8((R8::HL, R8::C))), // 0x71
            0b0111_0010 => Ok(Instruction::LoadR8R8((R8::HL, R8::D))), // 0x72
            0b0111_0011 => Ok(Instruction::LoadR8R8((R8::HL, R8::E))), // 0x73
            0b0111_0100 => Ok(Instruction::LoadR8R8((R8::HL, R8::H))), // 0x74
            0b0111_0101 => Ok(Instruction::LoadR8R8((R8::HL, R8::L))), // 0x75
            0b0111_0110 => Ok(Instruction::Halt),                  // 0x76
            0b0111_0111 => Ok(Instruction::LoadR8R8((R8::HL, R8::A))), // 0x77
            0b0111_1000 => Ok(Instruction::LoadR8R8((R8::A, R8::B))), // 0x78
            0b0111_1001 => Ok(Instruction::LoadR8R8((R8::A, R8::C))), // 0x79
            0b0111_1010 => Ok(Instruction::LoadR8R8((R8::A, R8::D))), // 0x7A
            0b0111_1011 => Ok(Instruction::LoadR8R8((R8::A, R8::E))), // 0x7B
            0b0111_1100 => Ok(Instruction::LoadR8R8((R8::A, R8::H))), // 0x7C
            0b0111_1101 => Ok(Instruction::LoadR8R8((R8::A, R8::L))), // 0x7D
            0b0111_1110 => Ok(Instruction::LoadR8R8((R8::A, R8::HL))), // 0x7E
            0b0111_1111 => Ok(Instruction::LoadR8R8((R8::A, R8::A))), // 0x7F
            0b1000_0000 => Ok(Instruction::AddR8(R8::B)),          // 0x80
            0b1000_0001 => Ok(Instruction::AddR8(R8::C)),          // 0x81
            0b1000_0010 => Ok(Instruction::AddR8(R8::D)),          // 0x82
            0b1000_0011 => Ok(Instruction::AddR8(R8::E)),          // 0x83
            0b1000_0100 => Ok(Instruction::AddR8(R8::H)),          // 0x84
            0b1000_0101 => Ok(Instruction::AddR8(R8::L)),          // 0x85
            0b1000_0110 => Ok(Instruction::AddR8(R8::HL)),         // 0x86
            0b1000_0111 => Ok(Instruction::AddR8(R8::A)),          // 0x87
            0b1100_0000 => Ok(Instruction::ReturnCondition(JumpCondition::NotZero)), // 0xC0
            0b1100_0001 => Ok(Instruction::PopR16(R16Stack::BC)),  // 0xC1
            0b1100_0010 => Ok(Instruction::JpCondImm16(JumpCondition::NotZero)), // 0xC2
            0b1100_0011 => Ok(Instruction::JpImm16),               // 0xC3
            0b1100_0101 => Ok(Instruction::PushR16(R16Stack::BC)), // 0xC5
            0b1100_1000 => Ok(Instruction::ReturnCondition(JumpCondition::Zero)), // 0xC8
            0b1100_1001 => Ok(Instruction::Return),                // 0xC9
            0b1100_1010 => Ok(Instruction::JpCondImm16(JumpCondition::Zero)), // 0xCA
            0b1101_0000 => Ok(Instruction::ReturnCondition(JumpCondition::NotCarry)), // 0xD0
            0b1101_0001 => Ok(Instruction::PopR16(R16Stack::DE)),  // 0xD1
            0b1101_0010 => Ok(Instruction::JpCondImm16(JumpCondition::NotCarry)), // 0xD2
            0b1101_0101 => Ok(Instruction::PushR16(R16Stack::DE)), // 0xD5
            0b1101_1000 => Ok(Instruction::ReturnCondition(JumpCondition::Carry)), // 0xD8
            0b1101_1001 => Ok(Instruction::ReturnEnableInterrupts), // 0xD9
            0b1101_1010 => Ok(Instruction::JpCondImm16(JumpCondition::Carry)), // 0xDA
            0b1110_0001 => Ok(Instruction::PopR16(R16Stack::HL)),  // 0xE1
            0b1110_0101 => Ok(Instruction::PushR16(R16Stack::HL)), // 0xE5
            0b1110_1001 => Ok(Instruction::JpHL),                  // 0xE9
            0b1111_0001 => Ok(Instruction::PopR16(R16Stack::AF)),  // 0xF1
            0b1111_0011 => Ok(Instruction::DisableInterrupts),     // 0xF3
            0b1111_0101 => Ok(Instruction::PushR16(R16Stack::AF)), // 0xF5
            0b1111_1011 => Ok(Instruction::EnableInterrupts),      // 0xFB
            _ => Err(format!("Unknown unprefixed instruction byte: {:02X}", byte).into()),
        }
    }

    pub fn from_byte_prefixed(byte: u8) -> Result<Self, Box<dyn Error>> {
        match byte {
            _ => Err(format!("Unknown prefixed instruction byte: {:02X}", byte).into()),
        }
    }

    pub fn get_length(&self) -> usize {
        match self {
            Self::Nop
            | Self::AddR8(_)
            | Self::JpHL
            | Self::PopR16(_)
            | Self::PushR16(_)
            | Self::LoadR16A(_)
            | Self::LoadAR16(_)
            | Self::IncR16(_)
            | Self::DecR16(_)
            | Self::IncR8(_)
            | Self::DecR8(_)
            | Self::AddHLR16(_)
            | Self::ComplementA
            | Self::ComplementCarryFlag
            | Self::RotateLeftA
            | Self::RotateRightA
            | Self::RotateLeftCarryA
            | Self::RotateRightCarryA
            | Self::SetCarryFlag
            | Self::DAA
            | Self::DisableInterrupts
            | Self::EnableInterrupts
            | Self::Return
            | Self::ReturnCondition(_)
            | Self::ReturnEnableInterrupts
            | Self::Halt
            | Self::LoadR8R8(_) => 1,
            Self::LoadR8Imm8(_) | Self::JrImm8 | Self::JrCondImm8(_) => 2,
            Self::JpImm16 | Self::JpCondImm16(_) | Self::LoadR16Imm16(_) | Self::LoadImm16SP => 3,
        }
    }

    pub fn parse_clear_text_instructions_from_data(
        data: &[u8],
        detailed: bool,
    ) -> Result<Vec<String>, Box<dyn Error>> {
        let mut instructions = Vec::new();
        let mut i = 0;

        while i < data.len() {
            let prefixed = if data[i] == PREFIX_INSTRUCTION_BYTE {
                i += 1;
                if i == data.len() {
                    break;
                }
                true
            } else {
                false
            };

            let current_byte = data[i];
            let instruction = Instruction::from_byte(current_byte, prefixed)?;
            let lsb = data.get(i + 1).copied().unwrap_or(0);
            let msb = data.get(i + 2).copied().unwrap_or(0);

            let text = if !detailed {
                instruction.parse_clear_text(lsb, msb)
            } else {
                instruction.parse_description(lsb, msb)
            };

            instructions.push(format!("[0x{:02X}] {text}", current_byte));
            i += instruction.get_length();
        }

        Ok(instructions)
    }

    /// Takes in the 2 following bytes after the instruction
    pub fn parse_clear_text(&self, lsb: u8, msb: u8) -> String {
        match self {
            Self::Nop => "NOP".into(),
            Self::AddHLR16(r16) => format!("ADD HL, {r16}"),
            Self::AddR8(r8) => format!("ADD A, {r8}"),
            Self::ComplementA => "CPL".into(),
            Self::ComplementCarryFlag => "CCF".into(),
            Self::DAA => "DAA".into(),
            Self::DecR8(r8) => format!("DEC {r8}"),
            Self::DecR16(r16) => format!("DEC {r16}"),
            Self::DisableInterrupts => "DI".into(),
            Self::EnableInterrupts => "EI".into(),
            Self::Halt => "HALT".into(),
            Self::IncR8(r8) => format!("INC {r8}"),
            Self::IncR16(r16) => format!("INC {r16}"),
            Self::JpHL => "JP HL".into(),
            Self::JpImm16 => format!("JP 0x{:02X}{:02X}", msb, lsb),
            Self::JpCondImm16(cond) => format!("JP {cond}, 0x{:02X}{:02X}", msb, lsb),
            Self::JrImm8 => format!("JR 0x{:02X}", lsb),
            Self::JrCondImm8(cond) => format!("JR {cond}, 0x{:02X}", lsb),
            Self::LoadAR16(r16_mem) => format!("LD A, {r16_mem}"),
            Self::LoadR16A(r16_mem) => format!("LD {r16_mem}, A"),
            Self::LoadR16Imm16(r16) => format!("LD {r16}, 0x{:02X}{:02X}", msb, lsb),
            Self::LoadR8Imm8(r8) => format!("LD {r8}, 0x{:02X}", msb),
            Self::LoadR8R8((target, source)) => format!("LD {target}, {source}"),
            Self::LoadImm16SP => format!("LD 0x{:02X}{:02X}, SP", msb, lsb),
            Self::PopR16(r16_stack) => format!("POP {r16_stack}"),
            Self::PushR16(r16_stack) => format!("PUSH {r16_stack}"),
            Self::Return => "RET".into(),
            Self::ReturnCondition(cond) => format!("RET {cond}"),
            Self::ReturnEnableInterrupts => "RETI".into(),
            Self::RotateLeftA => "RLA".into(),
            Self::RotateRightA => "RRA".into(),
            Self::RotateLeftCarryA => "RLCA".into(),
            Self::RotateRightCarryA => "RRCA".into(),
            Self::SetCarryFlag => "SCF".into(),
        }
    }

    /// Takes in the 2 following bytes after the instruction
    pub fn parse_description(&self, lsb: u8, msb: u8) -> String {
        match self {
            Self::Nop => "No Operation".into(),
            Self::AddHLR16(r16) => format!("Add value from register {r16} to register HL"),
            Self::AddR8(r8) => format!("Add value from register {r8} to register A"),
            Self::ComplementA => "Negate register A bitwise".into(),
            Self::ComplementCarryFlag => "Complement the carry flag in the F register".into(),
            Self::DAA => "Decimal adjust value in register A to be valid BCD".into(),
            Self::DecR8(r8) => format!("Decrement register {r8}"),
            Self::DecR16(r16) => format!("Decrement register {r16}"),
            Self::DisableInterrupts => "Disable interrupts".into(),
            Self::EnableInterrupts => "Enable interrupts after the next instruction".into(),
            Self::Halt => "Halt".into(),
            Self::IncR8(r8) => format!("Increment register {r8}"),
            Self::IncR16(r16) => format!("Increment register {r16}"),
            Self::JpHL => "Jump to the address specified in register HL".into(),
            Self::JpImm16 => format!("Jump to address 0x{:02X}{:02X}", msb, lsb),
            Self::JpCondImm16(cond) => {
                format!(
                    "If {}, jump to address 0x{:02X}{:02X}",
                    cond.description(),
                    msb,
                    lsb
                )
            }
            Self::JrImm8 => format!(
                "Jump relative, incrementing the program counter by 0x{:02X}",
                lsb
            ),
            Self::JrCondImm8(cond) => {
                format!(
                    "If {}, jump relative, incrementing the program counter by 0x{:02X}",
                    cond.description(),
                    lsb,
                )
            }
            Self::LoadAR16(r16_mem) => {
                let extra = match r16_mem {
                    R16Mem::HLD => "; Decrement register HL",
                    R16Mem::HLI => "; Increment register HL",
                    _ => "",
                };
                format!("Load value from the address stored in register {r16_mem} into register A{extra}")
            }
            Self::LoadR16A(r16_mem) => {
                let extra = match r16_mem {
                    R16Mem::HLD => "; Decrement register HL",
                    R16Mem::HLI => "; Increment register HL",
                    _ => "",
                };
                format!(
                    "Load value from register A into address stored at register {r16_mem}{extra}"
                )
            }
            Self::LoadR16Imm16(r16) => {
                format!("Load 0x{:02X}{:02X} into register {r16}", msb, lsb)
            }
            Self::LoadR8Imm8(r8) => {
                format!("Load 0x{:02X} into register {r8}", msb)
            }
            Self::LoadR8R8((target, source)) => {
                format!("Load value in register {source} into register {target}")
            }
            Self::LoadImm16SP => {
                format!(
                    "Load value from the top of the stack into address 0x{:02X}{:02X}",
                    msb, lsb
                )
            }
            Self::PopR16(r16_stack) => format!("Pop value from stack into register {r16_stack}"),
            Self::PushR16(r16_stack) => format!("Push value in {r16_stack} onto the stack"),
            Self::Return => "Return from a called function".into(),
            Self::ReturnCondition(cond) => {
                format!("If {}, return from a called function", cond.description())
            }
            Self::ReturnEnableInterrupts => {
                "Return from a called function and enable interrupts".into()
            }
            Self::RotateLeftA => "Rotate register A left THROUGH the carry flag".into(),
            Self::RotateRightA => "Rotate register A right THROUGH the carry flag".into(),
            Self::RotateLeftCarryA => "Rotate register A left, update carry flag".into(),
            Self::RotateRightCarryA => "Rotate register A right, update carry flag".into(),
            Self::SetCarryFlag => "Set carry flag".into(),
        }
    }
}
