use crate::enums::parameter_groups::{JumpCondition, R16Mem, R16Stack, R16, R8};
use crate::game_boy::components::cpu::PREFIX_INSTRUCTION_BYTE;
use std::error::Error;

#[derive(Debug, Default, Clone, PartialEq)]
pub enum Instruction {
    /// Add value from the specified register to the HL register
    AddHLR16(R16),
    /// Add the specified register to register A
    AddR8(R8),
    /// Add the next byte to register A
    AddImm8,
    /// Add the specified register WITH the current carry to register A
    AddCarryR8(R8),
    /// Add the next byte AND the current carry to register A
    AddCarryImm8,
    /// Adds the next byte (signed) to the stack pointer
    AddSpImm8,
    /// Register A will be set to the bitwise AND between the value register A and the value in the specified register
    AndR8(R8),
    /// Register A will be set to the bitwise AND between the value register A and the next byte
    AndImm8,
    /// Unconditional function call to the address specified by the next 2 bytes
    Call,
    /// Conditional function call to the address specified by the next 2 bytes
    CallCondition(JumpCondition),
    /// Set flag according to a subtraction between value in register A and the specified register while discarding the result
    /// This essentially sets the Carry if r8 > A
    CompareR8(R8),
    /// Set flag according to a subtraction between value in register A and the next byte while discarding the result
    /// This essentially sets the Carry if next byte > A
    CompareImm8,
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
    /// Unconditional relative jump adding the next byte (signed) to the program counter
    JrImm8,
    /// Conditional relative jump adding the next byte (signed) to the program counter
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
    /// Load the value from memory at address 0xFF00 + C (value in register C will be the least significant byte) into register A
    LoadHighAC,
    /// Load the value in register A into memory at address 0xFF00 + C (value in register C will be the least significant byte)
    LoadHighCA,
    /// Load the value from memory at address 0xFF00 + next byte (next byte will be the least significant byte) into register A
    LoadHighAImm8,
    /// Load the value in register A into memory at address 0xFF00 + next byte (next byte will be the least significant byte)
    LoadHighImm8A,
    /// Load the value from memory at address specified by the next 2 bytes into register A
    LoadAImm16,
    /// Load the value in register A into memory at address specified by the next 2 bytes
    LoadImm16A,
    /// Load the value at the top of the stack into the address specified by the following 2 bytes
    LoadImm16SP,
    /// Load the stack pointer added with the next byte (signed) into register HL
    LoadHlSpImm8,
    /// Load register HL into register SP
    LoadSpHl,
    /// Does nothing, will stall a cycle
    #[default]
    Nop,
    /// Register A will be set to the bitwise OR between the value register A and the value in the specified register
    OrR8(R8),
    /// Register A will be set to the bitwise OR between the value register A and the next byte
    OrImm8,
    /// Pop 2 bytes from the stack to the specified register
    PopR16(R16Stack),
    /// Push 2 bytes from the specified register to the stack
    PushR16(R16Stack),
    /// Unconditional quick function call to the address encoded within the instruction
    RestartVector(u8),
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
    RotateLeftCircularA,
    /// Rotate register A right by 1 bit
    /// ```
    ///   ┏━━━━━━━ A ━━━━━━━┓   ┏━ Flags ━┓
    /// ┌─╂→ b7 → ... → b0 ─╂─┬─╂→   C    ┃
    /// │ ┗━━━━━━━━━━━━━━━━━┛ │ ┗━━━━━━━━━┛
    /// └─────────────────────┘
    /// ```
    RotateRightCircularA,
    /// Sets the carry flag to 1
    SetCarryFlag,
    /// Subtract the value in the specified register from register A
    SubR8(R8),
    /// Subtract the next byte from register A
    SubImm8,
    /// Subtract the value in the specified register AND the current carry from register A
    SubCarryR8(R8),
    /// Subtract the next byte AND the current carry from register A
    SubCarryImm8,
    /// Register A will be set to the bitwise XOR between the value register A and the value in the specified register
    XorR8(R8),
    /// Register A will be set to the bitwise XOR between the value register A and the next byte
    XorImm8,
    // Prefixed instructions
    /// Test bit at specified index in the specified register, if its 0, set the zero flag
    BitCheckR8((usize, R8)),
    /// Reset bit at specified index in the specified register
    BitResetR8((usize, R8)),
    /// Set bit at specified index in the specified register
    BitSetR8((usize, R8)),
    /// Rotate the specified register left THROUGH the carry flag
    RotateLeftR8(R8),
    /// Rotate the specified register left
    RotateLeftCircularR8(R8),
    /// Rotate the specified register right THROUGH the carry flag
    RotateRightR8(R8),
    /// Rotate the specified register right
    RotateRightCircularR8(R8),
    /// Shift the specified register to the left (filling up with 0's)
    ShiftLeftR8(R8),
    /// Shift the specified register to the right (leftmost bit stays unchanged)
    ShiftRightR8(R8),
    /// Swap upper and lower 4 bits of the specified register
    SwapR8(R8),
    /// Shift the specified register to the right (filling up with 0's)
    ShiftRightLogicallyR8(R8),
}

impl Instruction {
    pub fn from_byte(byte: u8, prefixed: bool) -> Result<Self, Box<dyn Error>> {
        if prefixed {
            Ok(Self::from_byte_prefixed(byte))
        } else {
            Self::from_byte_unprefixed(byte)
        }
    }

    pub fn from_byte_unprefixed(byte: u8) -> Result<Self, Box<dyn Error>> {
        match byte {
            0b0000_0000 => Ok(Self::Nop),                                 // 0x00
            0b0000_0001 => Ok(Self::LoadR16Imm16(R16::BC)),               // 0x01
            0b0000_0010 => Ok(Self::LoadR16A(R16Mem::BC)),                // 0x02
            0b0000_0011 => Ok(Self::IncR16(R16::BC)),                     // 0x03
            0b0000_0100 => Ok(Self::IncR8(R8::B)),                        // 0x04
            0b0000_0101 => Ok(Self::DecR8(R8::B)),                        // 0x05
            0b0000_0110 => Ok(Self::LoadR8Imm8(R8::B)),                   // 0x06
            0b0000_0111 => Ok(Self::RotateLeftCircularA),                 // 0x07
            0b0000_1000 => Ok(Self::LoadImm16SP),                         // 0x08
            0b0000_1001 => Ok(Self::AddHLR16(R16::BC)),                   // 0x09
            0b0000_1010 => Ok(Self::LoadAR16(R16Mem::BC)),                // 0x0A
            0b0000_1011 => Ok(Self::DecR16(R16::BC)),                     // 0x0B
            0b0000_1100 => Ok(Self::IncR8(R8::C)),                        // 0x0C
            0b0000_1101 => Ok(Self::DecR8(R8::C)),                        // 0x0D
            0b0000_1110 => Ok(Self::LoadR8Imm8(R8::C)),                   // 0x0E
            0b0000_1111 => Ok(Self::RotateRightCircularA),                // 0x0F
            0b0001_0000 => Ok(Self::Nop),                                 // 0x10 ToDo: STOP
            0b0001_0001 => Ok(Self::LoadR16Imm16(R16::DE)),               // 0x11
            0b0001_0010 => Ok(Self::LoadR16A(R16Mem::DE)),                // 0x12
            0b0001_0011 => Ok(Self::IncR16(R16::DE)),                     // 0x13
            0b0001_0100 => Ok(Self::IncR8(R8::D)),                        // 0x14
            0b0001_0101 => Ok(Self::DecR8(R8::D)),                        // 0x15
            0b0001_0110 => Ok(Self::LoadR8Imm8(R8::D)),                   // 0x16
            0b0001_0111 => Ok(Self::RotateLeftA),                         // 0x17
            0b0001_1000 => Ok(Self::JrImm8),                              // 0x18
            0b0001_1001 => Ok(Self::AddHLR16(R16::DE)),                   // 0x19
            0b0001_1010 => Ok(Self::LoadAR16(R16Mem::DE)),                // 0x1A
            0b0001_1011 => Ok(Self::DecR16(R16::DE)),                     // 0x1B
            0b0001_1100 => Ok(Self::IncR8(R8::E)),                        // 0x1C
            0b0001_1101 => Ok(Self::DecR8(R8::E)),                        // 0x1D
            0b0001_1110 => Ok(Self::LoadR8Imm8(R8::E)),                   // 0x1E
            0b0001_1111 => Ok(Self::RotateRightA),                        // 0x1F
            0b0010_0000 => Ok(Self::JrCondImm8(JumpCondition::NotZero)),  // 0x20
            0b0010_0001 => Ok(Self::LoadR16Imm16(R16::HL)),               // 0x21
            0b0010_0010 => Ok(Self::LoadR16A(R16Mem::HLI)),               // 0x22
            0b0010_0011 => Ok(Self::IncR16(R16::HL)),                     // 0x23
            0b0010_0100 => Ok(Self::IncR8(R8::H)),                        // 0x24
            0b0010_0101 => Ok(Self::DecR8(R8::H)),                        // 0x25
            0b0010_0110 => Ok(Self::LoadR8Imm8(R8::H)),                   // 0x26
            0b0010_0111 => Ok(Self::DAA),                                 // 0x27
            0b0010_1000 => Ok(Self::JrCondImm8(JumpCondition::Zero)),     // 0x28
            0b0010_1001 => Ok(Self::AddHLR16(R16::HL)),                   // 0x29
            0b0010_1111 => Ok(Self::ComplementA),                         // 0x2F
            0b0011_0000 => Ok(Self::JrCondImm8(JumpCondition::NotCarry)), // 0x30
            0b0010_1010 => Ok(Self::LoadAR16(R16Mem::HLI)),               // 0x2A
            0b0010_1011 => Ok(Self::DecR16(R16::HL)),                     // 0x2B
            0b0010_1100 => Ok(Self::IncR8(R8::L)),                        // 0x2C
            0b0010_1101 => Ok(Self::DecR8(R8::L)),                        // 0x2D
            0b0010_1110 => Ok(Self::LoadR8Imm8(R8::L)),                   // 0x2E
            0b0011_0001 => Ok(Self::LoadR16Imm16(R16::SP)),               // 0x31
            0b0011_0010 => Ok(Self::LoadR16A(R16Mem::HLD)),               // 0x32
            0b0011_0011 => Ok(Self::IncR16(R16::SP)),                     // 0x33
            0b0011_0100 => Ok(Self::IncR8(R8::HL)),                       // 0x34
            0b0011_0101 => Ok(Self::DecR8(R8::HL)),                       // 0x35
            0b0011_0110 => Ok(Self::LoadR8Imm8(R8::HL)),                  // 0x36
            0b0011_0111 => Ok(Self::SetCarryFlag),                        // 0x37
            0b0011_1000 => Ok(Self::JrCondImm8(JumpCondition::Carry)),    // 0x38
            0b0011_1001 => Ok(Self::AddHLR16(R16::SP)),                   // 0x39
            0b0011_1010 => Ok(Self::LoadAR16(R16Mem::HLD)),               // 0x3A
            0b0011_1011 => Ok(Self::DecR16(R16::SP)),                     // 0x3B
            0b0011_1100 => Ok(Self::IncR8(R8::A)),                        // 0x3C
            0b0011_1101 => Ok(Self::DecR8(R8::A)),                        // 0x3D
            0b0011_1110 => Ok(Self::LoadR8Imm8(R8::A)),                   // 0x3E
            0b0011_1111 => Ok(Self::ComplementCarryFlag),                 // 0x3F
            0b0100_0000 => Ok(Self::LoadR8R8((R8::B, R8::B))),            // 0x40
            0b0100_0001 => Ok(Self::LoadR8R8((R8::B, R8::C))),            // 0x41
            0b0100_0010 => Ok(Self::LoadR8R8((R8::B, R8::D))),            // 0x42
            0b0100_0011 => Ok(Self::LoadR8R8((R8::B, R8::E))),            // 0x43
            0b0100_0100 => Ok(Self::LoadR8R8((R8::B, R8::H))),            // 0x44
            0b0100_0101 => Ok(Self::LoadR8R8((R8::B, R8::L))),            // 0x45
            0b0100_0110 => Ok(Self::LoadR8R8((R8::B, R8::HL))),           // 0x46
            0b0100_0111 => Ok(Self::LoadR8R8((R8::B, R8::A))),            // 0x47
            0b0100_1000 => Ok(Self::LoadR8R8((R8::C, R8::B))),            // 0x48
            0b0100_1001 => Ok(Self::LoadR8R8((R8::C, R8::C))),            // 0x49
            0b0100_1010 => Ok(Self::LoadR8R8((R8::C, R8::D))),            // 0x4A
            0b0100_1011 => Ok(Self::LoadR8R8((R8::C, R8::E))),            // 0x4B
            0b0100_1100 => Ok(Self::LoadR8R8((R8::C, R8::H))),            // 0x4C
            0b0100_1101 => Ok(Self::LoadR8R8((R8::C, R8::L))),            // 0x4D
            0b0100_1110 => Ok(Self::LoadR8R8((R8::C, R8::HL))),           // 0x4E
            0b0100_1111 => Ok(Self::LoadR8R8((R8::C, R8::A))),            // 0x4F
            0b0101_0000 => Ok(Self::LoadR8R8((R8::D, R8::B))),            // 0x50
            0b0101_0001 => Ok(Self::LoadR8R8((R8::D, R8::C))),            // 0x51
            0b0101_0010 => Ok(Self::LoadR8R8((R8::D, R8::D))),            // 0x52
            0b0101_0011 => Ok(Self::LoadR8R8((R8::D, R8::E))),            // 0x53
            0b0101_0100 => Ok(Self::LoadR8R8((R8::D, R8::H))),            // 0x54
            0b0101_0101 => Ok(Self::LoadR8R8((R8::D, R8::L))),            // 0x55
            0b0101_0110 => Ok(Self::LoadR8R8((R8::D, R8::HL))),           // 0x56
            0b0101_0111 => Ok(Self::LoadR8R8((R8::D, R8::A))),            // 0x57
            0b0101_1000 => Ok(Self::LoadR8R8((R8::E, R8::B))),            // 0x58
            0b0101_1001 => Ok(Self::LoadR8R8((R8::E, R8::C))),            // 0x59
            0b0101_1010 => Ok(Self::LoadR8R8((R8::E, R8::D))),            // 0x5A
            0b0101_1011 => Ok(Self::LoadR8R8((R8::E, R8::E))),            // 0x5B
            0b0101_1100 => Ok(Self::LoadR8R8((R8::E, R8::H))),            // 0x5C
            0b0101_1101 => Ok(Self::LoadR8R8((R8::E, R8::L))),            // 0x5D
            0b0101_1110 => Ok(Self::LoadR8R8((R8::E, R8::HL))),           // 0x5E
            0b0101_1111 => Ok(Self::LoadR8R8((R8::E, R8::A))),            // 0x5F
            0b0110_0000 => Ok(Self::LoadR8R8((R8::H, R8::B))),            // 0x60
            0b0110_0001 => Ok(Self::LoadR8R8((R8::H, R8::C))),            // 0x61
            0b0110_0010 => Ok(Self::LoadR8R8((R8::H, R8::D))),            // 0x62
            0b0110_0011 => Ok(Self::LoadR8R8((R8::H, R8::E))),            // 0x63
            0b0110_0100 => Ok(Self::LoadR8R8((R8::H, R8::H))),            // 0x64
            0b0110_0101 => Ok(Self::LoadR8R8((R8::H, R8::L))),            // 0x65
            0b0110_0110 => Ok(Self::LoadR8R8((R8::H, R8::HL))),           // 0x66
            0b0110_0111 => Ok(Self::LoadR8R8((R8::H, R8::A))),            // 0x67
            0b0110_1000 => Ok(Self::LoadR8R8((R8::L, R8::B))),            // 0x68
            0b0110_1001 => Ok(Self::LoadR8R8((R8::L, R8::C))),            // 0x69
            0b0110_1010 => Ok(Self::LoadR8R8((R8::L, R8::D))),            // 0x6A
            0b0110_1011 => Ok(Self::LoadR8R8((R8::L, R8::E))),            // 0x6B
            0b0110_1100 => Ok(Self::LoadR8R8((R8::L, R8::H))),            // 0x6C
            0b0110_1101 => Ok(Self::LoadR8R8((R8::L, R8::L))),            // 0x6D
            0b0110_1110 => Ok(Self::LoadR8R8((R8::L, R8::HL))),           // 0x6E
            0b0110_1111 => Ok(Self::LoadR8R8((R8::L, R8::A))),            // 0x6F
            0b0111_0000 => Ok(Self::LoadR8R8((R8::HL, R8::B))),           // 0x70
            0b0111_0001 => Ok(Self::LoadR8R8((R8::HL, R8::C))),           // 0x71
            0b0111_0010 => Ok(Self::LoadR8R8((R8::HL, R8::D))),           // 0x72
            0b0111_0011 => Ok(Self::LoadR8R8((R8::HL, R8::E))),           // 0x73
            0b0111_0100 => Ok(Self::LoadR8R8((R8::HL, R8::H))),           // 0x74
            0b0111_0101 => Ok(Self::LoadR8R8((R8::HL, R8::L))),           // 0x75
            0b0111_0110 => Ok(Self::Halt),                                // 0x76
            0b0111_0111 => Ok(Self::LoadR8R8((R8::HL, R8::A))),           // 0x77
            0b0111_1000 => Ok(Self::LoadR8R8((R8::A, R8::B))),            // 0x78
            0b0111_1001 => Ok(Self::LoadR8R8((R8::A, R8::C))),            // 0x79
            0b0111_1010 => Ok(Self::LoadR8R8((R8::A, R8::D))),            // 0x7A
            0b0111_1011 => Ok(Self::LoadR8R8((R8::A, R8::E))),            // 0x7B
            0b0111_1100 => Ok(Self::LoadR8R8((R8::A, R8::H))),            // 0x7C
            0b0111_1101 => Ok(Self::LoadR8R8((R8::A, R8::L))),            // 0x7D
            0b0111_1110 => Ok(Self::LoadR8R8((R8::A, R8::HL))),           // 0x7E
            0b0111_1111 => Ok(Self::LoadR8R8((R8::A, R8::A))),            // 0x7F
            0b1000_0000 => Ok(Self::AddR8(R8::B)),                        // 0x80
            0b1000_0001 => Ok(Self::AddR8(R8::C)),                        // 0x81
            0b1000_0010 => Ok(Self::AddR8(R8::D)),                        // 0x82
            0b1000_0011 => Ok(Self::AddR8(R8::E)),                        // 0x83
            0b1000_0100 => Ok(Self::AddR8(R8::H)),                        // 0x84
            0b1000_0101 => Ok(Self::AddR8(R8::L)),                        // 0x85
            0b1000_0110 => Ok(Self::AddR8(R8::HL)),                       // 0x86
            0b1000_0111 => Ok(Self::AddR8(R8::A)),                        // 0x87
            0b1000_1000 => Ok(Self::AddCarryR8(R8::B)),                   // 0x88
            0b1000_1001 => Ok(Self::AddCarryR8(R8::C)),                   // 0x89
            0b1000_1010 => Ok(Self::AddCarryR8(R8::D)),                   // 0x8A
            0b1000_1011 => Ok(Self::AddCarryR8(R8::E)),                   // 0x8B
            0b1000_1100 => Ok(Self::AddCarryR8(R8::H)),                   // 0x8C
            0b1000_1101 => Ok(Self::AddCarryR8(R8::L)),                   // 0x8D
            0b1000_1110 => Ok(Self::AddCarryR8(R8::HL)),                  // 0x8E
            0b1000_1111 => Ok(Self::AddCarryR8(R8::A)),                   // 0x8F
            0b1001_0000 => Ok(Self::SubR8(R8::B)),                        // 0x90
            0b1001_0001 => Ok(Self::SubR8(R8::C)),                        // 0x91
            0b1001_0010 => Ok(Self::SubR8(R8::D)),                        // 0x92
            0b1001_0011 => Ok(Self::SubR8(R8::E)),                        // 0x93
            0b1001_0100 => Ok(Self::SubR8(R8::H)),                        // 0x94
            0b1001_0101 => Ok(Self::SubR8(R8::L)),                        // 0x95
            0b1001_0110 => Ok(Self::SubR8(R8::HL)),                       // 0x96
            0b1001_0111 => Ok(Self::SubR8(R8::A)),                        // 0x97
            0b1001_1000 => Ok(Self::SubCarryR8(R8::B)),                   // 0x98
            0b1001_1001 => Ok(Self::SubCarryR8(R8::C)),                   // 0x99
            0b1001_1010 => Ok(Self::SubCarryR8(R8::D)),                   // 0x9A
            0b1001_1011 => Ok(Self::SubCarryR8(R8::E)),                   // 0x9B
            0b1001_1100 => Ok(Self::SubCarryR8(R8::H)),                   // 0x9C
            0b1001_1101 => Ok(Self::SubCarryR8(R8::L)),                   // 0x9D
            0b1001_1110 => Ok(Self::SubCarryR8(R8::HL)),                  // 0x9E
            0b1001_1111 => Ok(Self::SubCarryR8(R8::A)),                   // 0x9F
            0b1010_0000 => Ok(Self::AndR8(R8::B)),                        // 0xA0
            0b1010_0001 => Ok(Self::AndR8(R8::C)),                        // 0xA1
            0b1010_0010 => Ok(Self::AndR8(R8::D)),                        // 0xA2
            0b1010_0011 => Ok(Self::AndR8(R8::E)),                        // 0xA3
            0b1010_0100 => Ok(Self::AndR8(R8::H)),                        // 0xA4
            0b1010_0101 => Ok(Self::AndR8(R8::L)),                        // 0xA5
            0b1010_0110 => Ok(Self::AndR8(R8::HL)),                       // 0xA6
            0b1010_0111 => Ok(Self::AndR8(R8::A)),                        // 0xA7
            0b1010_1000 => Ok(Self::XorR8(R8::B)),                        // 0xA8
            0b1010_1001 => Ok(Self::XorR8(R8::C)),                        // 0xA9
            0b1010_1010 => Ok(Self::XorR8(R8::D)),                        // 0xAA
            0b1010_1011 => Ok(Self::XorR8(R8::E)),                        // 0xAB
            0b1010_1100 => Ok(Self::XorR8(R8::H)),                        // 0xAC
            0b1010_1101 => Ok(Self::XorR8(R8::L)),                        // 0xAD
            0b1010_1110 => Ok(Self::XorR8(R8::HL)),                       // 0xAE
            0b1010_1111 => Ok(Self::XorR8(R8::A)),                        // 0xAF
            0b1011_0000 => Ok(Self::OrR8(R8::B)),                         // 0xB0
            0b1011_0001 => Ok(Self::OrR8(R8::C)),                         // 0xB1
            0b1011_0010 => Ok(Self::OrR8(R8::D)),                         // 0xB2
            0b1011_0011 => Ok(Self::OrR8(R8::E)),                         // 0xB3
            0b1011_0100 => Ok(Self::OrR8(R8::H)),                         // 0xB4
            0b1011_0101 => Ok(Self::OrR8(R8::L)),                         // 0xB5
            0b1011_0110 => Ok(Self::OrR8(R8::HL)),                        // 0xB6
            0b1011_0111 => Ok(Self::OrR8(R8::A)),                         // 0xB7
            0b1011_1000 => Ok(Self::CompareR8(R8::B)),                    // 0xB8
            0b1011_1001 => Ok(Self::CompareR8(R8::C)),                    // 0xB9
            0b1011_1010 => Ok(Self::CompareR8(R8::D)),                    // 0xBA
            0b1011_1011 => Ok(Self::CompareR8(R8::E)),                    // 0xBB
            0b1011_1100 => Ok(Self::CompareR8(R8::H)),                    // 0xBC
            0b1011_1101 => Ok(Self::CompareR8(R8::L)),                    // 0xBD
            0b1011_1110 => Ok(Self::CompareR8(R8::HL)),                   // 0xBE
            0b1011_1111 => Ok(Self::CompareR8(R8::A)),                    // 0xBF
            0b1100_0000 => Ok(Self::ReturnCondition(JumpCondition::NotZero)), // 0xC0
            0b1100_0001 => Ok(Self::PopR16(R16Stack::BC)),                // 0xC1
            0b1100_0010 => Ok(Self::JpCondImm16(JumpCondition::NotZero)), // 0xC2
            0b1100_0011 => Ok(Self::JpImm16),                             // 0xC3
            0b1100_0100 => Ok(Self::CallCondition(JumpCondition::NotZero)), // 0xC4
            0b1100_0101 => Ok(Self::PushR16(R16Stack::BC)),               // 0xC5
            0b1100_0110 => Ok(Self::AddImm8),                             // 0xC6
            0b1100_0111 => Ok(Self::RestartVector(0x00)),                 // 0xC7
            0b1100_1000 => Ok(Self::ReturnCondition(JumpCondition::Zero)), // 0xC8
            0b1100_1001 => Ok(Self::Return),                              // 0xC9
            0b1100_1010 => Ok(Self::JpCondImm16(JumpCondition::Zero)),    // 0xCA
            0b1100_1100 => Ok(Self::CallCondition(JumpCondition::Zero)),  // 0xCC
            0b1100_1101 => Ok(Self::Call),                                // 0xCD
            0b1100_1110 => Ok(Self::AddCarryImm8),                        // 0xCE
            0b1100_1111 => Ok(Self::RestartVector(0x08)),                 // 0xCF
            0b1101_0000 => Ok(Self::ReturnCondition(JumpCondition::NotCarry)), // 0xD0
            0b1101_0001 => Ok(Self::PopR16(R16Stack::DE)),                // 0xD1
            0b1101_0010 => Ok(Self::JpCondImm16(JumpCondition::NotCarry)), // 0xD2
            0b1101_0100 => Ok(Self::CallCondition(JumpCondition::NotCarry)), // 0xD4
            0b1101_0101 => Ok(Self::PushR16(R16Stack::DE)),               // 0xD5
            0b1101_0110 => Ok(Self::SubImm8),                             // 0xD6
            0b1101_0111 => Ok(Self::RestartVector(0x10)),                 // 0xD7
            0b1101_1000 => Ok(Self::ReturnCondition(JumpCondition::Carry)), // 0xD8
            0b1101_1001 => Ok(Self::ReturnEnableInterrupts),              // 0xD9
            0b1101_1010 => Ok(Self::JpCondImm16(JumpCondition::Carry)),   // 0xDA,
            0b1101_1100 => Ok(Self::CallCondition(JumpCondition::Carry)), // 0xDC
            0b1101_1110 => Ok(Self::SubCarryImm8),                        // 0xDE
            0b1101_1111 => Ok(Self::RestartVector(0x18)),                 // 0xDF
            0b1110_0000 => Ok(Self::LoadHighImm8A),                       // 0xE0
            0b1110_0001 => Ok(Self::PopR16(R16Stack::HL)),                // 0xE1
            0b1110_0010 => Ok(Self::LoadHighCA),                          // 0xE2
            0b1110_0101 => Ok(Self::PushR16(R16Stack::HL)),               // 0xE5
            0b1110_0110 => Ok(Self::AndImm8),                             // 0xE6
            0b1110_0111 => Ok(Self::RestartVector(0x0020)),               // 0xE7
            0b1110_1000 => Ok(Self::AddSpImm8),                           // 0xE8
            0b1110_1001 => Ok(Self::JpHL),                                // 0xE9
            0b1110_1010 => Ok(Self::LoadImm16A),                          // 0xEA
            0b1110_1110 => Ok(Self::XorImm8),                             // 0xEE
            0b1110_1111 => Ok(Self::RestartVector(0x28)),                 // 0xEF
            0b1111_0000 => Ok(Self::LoadHighAImm8),                       // 0xF0
            0b1111_0001 => Ok(Self::PopR16(R16Stack::AF)),                // 0xF1
            0b1111_0010 => Ok(Self::LoadHighAC),                          // 0xF2
            0b1111_0011 => Ok(Self::DisableInterrupts),                   // 0xF3
            0b1111_0101 => Ok(Self::PushR16(R16Stack::AF)),               // 0xF5
            0b1111_0110 => Ok(Self::OrImm8),                              // 0xF6
            0b1111_0111 => Ok(Self::RestartVector(0x30)),                 // 0xF7
            0b1111_1000 => Ok(Self::LoadHlSpImm8),                        // 0xF8
            0b1111_1001 => Ok(Self::LoadSpHl),                            // 0xF9
            0b1111_1010 => Ok(Self::LoadAImm16),                          // 0xFA
            0b1111_1011 => Ok(Self::EnableInterrupts),                    // 0xFB
            0b1111_1110 => Ok(Self::CompareImm8),                         // 0xFE
            0b1111_1111 => Ok(Self::RestartVector(0x38)),                 // 0xFF
            _ => Err(format!("Illegal unprefixed instruction byte: {:02X}", byte).into()),
        }
    }

    #[allow(unreachable_patterns)]
    pub fn from_byte_prefixed(byte: u8) -> Self {
        match byte {
            0b0000_0000 => Self::RotateLeftCircularR8(R8::B), // 0x00
            0b0000_0001 => Self::RotateLeftCircularR8(R8::C), // 0x01
            0b0000_0010 => Self::RotateLeftCircularR8(R8::D), // 0x02
            0b0000_0011 => Self::RotateLeftCircularR8(R8::E), // 0x03
            0b0000_0100 => Self::RotateLeftCircularR8(R8::H), // 0x04
            0b0000_0101 => Self::RotateLeftCircularR8(R8::L), // 0x05
            0b0000_0110 => Self::RotateLeftCircularR8(R8::HL), // 0x06
            0b0000_0111 => Self::RotateLeftCircularR8(R8::A), // 0x07
            0b0000_1000 => Self::RotateRightCircularR8(R8::B), // 0x08
            0b0000_1001 => Self::RotateRightCircularR8(R8::C), // 0x09
            0b0000_1010 => Self::RotateRightCircularR8(R8::D), // 0x0A
            0b0000_1011 => Self::RotateRightCircularR8(R8::E), // 0x0B
            0b0000_1100 => Self::RotateRightCircularR8(R8::H), // 0x0C
            0b0000_1101 => Self::RotateRightCircularR8(R8::L), // 0x0D
            0b0000_1110 => Self::RotateRightCircularR8(R8::HL), // 0x0E
            0b0000_1111 => Self::RotateRightCircularR8(R8::A), // 0x0F
            0b0001_0000 => Self::RotateLeftR8(R8::B),         // 0x10
            0b0001_0001 => Self::RotateLeftR8(R8::C),         // 0x11
            0b0001_0010 => Self::RotateLeftR8(R8::D),         // 0x12
            0b0001_0011 => Self::RotateLeftR8(R8::E),         // 0x13
            0b0001_0100 => Self::RotateLeftR8(R8::H),         // 0x14
            0b0001_0101 => Self::RotateLeftR8(R8::L),         // 0x15
            0b0001_0110 => Self::RotateLeftR8(R8::HL),        // 0x16
            0b0001_0111 => Self::RotateLeftR8(R8::A),         // 0x17
            0b0001_1000 => Self::RotateRightR8(R8::B),        // 0x18
            0b0001_1001 => Self::RotateRightR8(R8::C),        // 0x19
            0b0001_1010 => Self::RotateRightR8(R8::D),        // 0x1A
            0b0001_1011 => Self::RotateRightR8(R8::E),        // 0x1B
            0b0001_1100 => Self::RotateRightR8(R8::H),        // 0x1C
            0b0001_1101 => Self::RotateRightR8(R8::L),        // 0x1D
            0b0001_1110 => Self::RotateRightR8(R8::HL),       // 0x1E
            0b0001_1111 => Self::RotateRightR8(R8::A),        // 0x1F
            0b0010_0000 => Self::ShiftLeftR8(R8::B),          // 0x20
            0b0010_0001 => Self::ShiftLeftR8(R8::C),          // 0x21
            0b0010_0010 => Self::ShiftLeftR8(R8::D),          // 0x22
            0b0010_0011 => Self::ShiftLeftR8(R8::E),          // 0x23
            0b0010_0100 => Self::ShiftLeftR8(R8::H),          // 0x24
            0b0010_0101 => Self::ShiftLeftR8(R8::L),          // 0x25
            0b0010_0110 => Self::ShiftLeftR8(R8::HL),         // 0x26
            0b0010_0111 => Self::ShiftLeftR8(R8::A),          // 0x27
            0b0010_1000 => Self::ShiftRightR8(R8::B),         // 0x28
            0b0010_1001 => Self::ShiftRightR8(R8::C),         // 0x29
            0b0010_1010 => Self::ShiftRightR8(R8::D),         // 0x2A
            0b0010_1011 => Self::ShiftRightR8(R8::E),         // 0x2B
            0b0010_1100 => Self::ShiftRightR8(R8::H),         // 0x2C
            0b0010_1101 => Self::ShiftRightR8(R8::L),         // 0x2D
            0b0010_1110 => Self::ShiftRightR8(R8::HL),        // 0x2E
            0b0010_1111 => Self::ShiftRightR8(R8::A),         // 0x2F
            0b0011_0000 => Self::SwapR8(R8::B),               // 0x30
            0b0011_0001 => Self::SwapR8(R8::C),               // 0x31
            0b0011_0010 => Self::SwapR8(R8::D),               // 0x32
            0b0011_0011 => Self::SwapR8(R8::E),               // 0x33
            0b0011_0100 => Self::SwapR8(R8::H),               // 0x34
            0b0011_0101 => Self::SwapR8(R8::L),               // 0x35
            0b0011_0110 => Self::SwapR8(R8::HL),              // 0x36
            0b0011_0111 => Self::SwapR8(R8::A),               // 0x37
            0b0011_1000 => Self::ShiftRightLogicallyR8(R8::B), // 0x38
            0b0011_1001 => Self::ShiftRightLogicallyR8(R8::C), // 0x39
            0b0011_1010 => Self::ShiftRightLogicallyR8(R8::D), // 0x3A
            0b0011_1011 => Self::ShiftRightLogicallyR8(R8::E), // 0x3B
            0b0011_1100 => Self::ShiftRightLogicallyR8(R8::H), // 0x3C
            0b0011_1101 => Self::ShiftRightLogicallyR8(R8::L), // 0x3D
            0b0011_1110 => Self::ShiftRightLogicallyR8(R8::HL), // 0x3E
            0b0011_1111 => Self::ShiftRightLogicallyR8(R8::A), // 0x3F
            0b0100_0000 => Self::BitCheckR8((0, R8::B)),      // 0x40
            0b0100_0001 => Self::BitCheckR8((0, R8::C)),      // 0x41
            0b0100_0010 => Self::BitCheckR8((0, R8::D)),      // 0x42
            0b0100_0011 => Self::BitCheckR8((0, R8::E)),      // 0x43
            0b0100_0100 => Self::BitCheckR8((0, R8::H)),      // 0x44
            0b0100_0101 => Self::BitCheckR8((0, R8::L)),      // 0x45
            0b0100_0110 => Self::BitCheckR8((0, R8::HL)),     // 0x46
            0b0100_0111 => Self::BitCheckR8((0, R8::A)),      // 0x47
            0b0100_1000 => Self::BitCheckR8((1, R8::B)),      // 0x48
            0b0100_1001 => Self::BitCheckR8((1, R8::C)),      // 0x49
            0b0100_1010 => Self::BitCheckR8((1, R8::D)),      // 0x4A
            0b0100_1011 => Self::BitCheckR8((1, R8::E)),      // 0x4B
            0b0100_1100 => Self::BitCheckR8((1, R8::H)),      // 0x4C
            0b0100_1101 => Self::BitCheckR8((1, R8::L)),      // 0x4D
            0b0100_1110 => Self::BitCheckR8((1, R8::HL)),     // 0x4E
            0b0100_1111 => Self::BitCheckR8((1, R8::A)),      // 0x4F
            0b0101_0000 => Self::BitCheckR8((2, R8::B)),      // 0x50
            0b0101_0001 => Self::BitCheckR8((2, R8::C)),      // 0x51
            0b0101_0010 => Self::BitCheckR8((2, R8::D)),      // 0x52
            0b0101_0011 => Self::BitCheckR8((2, R8::E)),      // 0x53
            0b0101_0100 => Self::BitCheckR8((2, R8::H)),      // 0x54
            0b0101_0101 => Self::BitCheckR8((2, R8::L)),      // 0x55
            0b0101_0110 => Self::BitCheckR8((2, R8::HL)),     // 0x56
            0b0101_0111 => Self::BitCheckR8((2, R8::A)),      // 0x57
            0b0101_1000 => Self::BitCheckR8((3, R8::B)),      // 0x58
            0b0101_1001 => Self::BitCheckR8((3, R8::C)),      // 0x59
            0b0101_1010 => Self::BitCheckR8((3, R8::D)),      // 0x5A
            0b0101_1011 => Self::BitCheckR8((3, R8::E)),      // 0x5B
            0b0101_1100 => Self::BitCheckR8((3, R8::H)),      // 0x5C
            0b0101_1101 => Self::BitCheckR8((3, R8::L)),      // 0x5D
            0b0101_1110 => Self::BitCheckR8((3, R8::HL)),     // 0x5E
            0b0101_1111 => Self::BitCheckR8((3, R8::A)),      // 0x5F
            0b0110_0000 => Self::BitCheckR8((4, R8::B)),      // 0x60
            0b0110_0001 => Self::BitCheckR8((4, R8::C)),      // 0x61
            0b0110_0010 => Self::BitCheckR8((4, R8::D)),      // 0x62
            0b0110_0011 => Self::BitCheckR8((4, R8::E)),      // 0x63
            0b0110_0100 => Self::BitCheckR8((4, R8::H)),      // 0x64
            0b0110_0101 => Self::BitCheckR8((4, R8::L)),      // 0x65
            0b0110_0110 => Self::BitCheckR8((4, R8::HL)),     // 0x66
            0b0110_0111 => Self::BitCheckR8((4, R8::A)),      // 0x67
            0b0110_1000 => Self::BitCheckR8((5, R8::B)),      // 0x68
            0b0110_1001 => Self::BitCheckR8((5, R8::C)),      // 0x69
            0b0110_1010 => Self::BitCheckR8((5, R8::D)),      // 0x6A
            0b0110_1011 => Self::BitCheckR8((5, R8::E)),      // 0x6B
            0b0110_1100 => Self::BitCheckR8((5, R8::H)),      // 0x6C
            0b0110_1101 => Self::BitCheckR8((5, R8::L)),      // 0x6D
            0b0110_1110 => Self::BitCheckR8((5, R8::HL)),     // 0x6E
            0b0110_1111 => Self::BitCheckR8((5, R8::A)),      // 0x6F
            0b0111_0000 => Self::BitCheckR8((6, R8::B)),      // 0x70
            0b0111_0001 => Self::BitCheckR8((6, R8::C)),      // 0x71
            0b0111_0010 => Self::BitCheckR8((6, R8::D)),      // 0x72
            0b0111_0011 => Self::BitCheckR8((6, R8::E)),      // 0x73
            0b0111_0100 => Self::BitCheckR8((6, R8::H)),      // 0x74
            0b0111_0101 => Self::BitCheckR8((6, R8::L)),      // 0x75
            0b0111_0110 => Self::BitCheckR8((6, R8::HL)),     // 0x76
            0b0111_0111 => Self::BitCheckR8((6, R8::A)),      // 0x77
            0b0111_1000 => Self::BitCheckR8((7, R8::B)),      // 0x78
            0b0111_1001 => Self::BitCheckR8((7, R8::C)),      // 0x79
            0b0111_1010 => Self::BitCheckR8((7, R8::D)),      // 0x7A
            0b0111_1011 => Self::BitCheckR8((7, R8::E)),      // 0x7B
            0b0111_1100 => Self::BitCheckR8((7, R8::H)),      // 0x7C
            0b0111_1101 => Self::BitCheckR8((7, R8::L)),      // 0x7D
            0b0111_1110 => Self::BitCheckR8((7, R8::HL)),     // 0x7E
            0b0111_1111 => Self::BitCheckR8((7, R8::A)),      // 0x7F
            0b1000_0000 => Self::BitResetR8((0, R8::B)),      // 0x80
            0b1000_0001 => Self::BitResetR8((0, R8::C)),      // 0x81
            0b1000_0010 => Self::BitResetR8((0, R8::D)),      // 0x82
            0b1000_0011 => Self::BitResetR8((0, R8::E)),      // 0x83
            0b1000_0100 => Self::BitResetR8((0, R8::H)),      // 0x84
            0b1000_0101 => Self::BitResetR8((0, R8::L)),      // 0x85
            0b1000_0110 => Self::BitResetR8((0, R8::HL)),     // 0x86
            0b1000_0111 => Self::BitResetR8((0, R8::A)),      // 0x87
            0b1000_1000 => Self::BitResetR8((1, R8::B)),      // 0x88
            0b1000_1001 => Self::BitResetR8((1, R8::C)),      // 0x89
            0b1000_1010 => Self::BitResetR8((1, R8::D)),      // 0x8A
            0b1000_1011 => Self::BitResetR8((1, R8::E)),      // 0x8B
            0b1000_1100 => Self::BitResetR8((1, R8::H)),      // 0x8C
            0b1000_1101 => Self::BitResetR8((1, R8::L)),      // 0x8D
            0b1000_1110 => Self::BitResetR8((1, R8::HL)),     // 0x8E
            0b1000_1111 => Self::BitResetR8((1, R8::A)),      // 0x8F
            0b1001_0000 => Self::BitResetR8((2, R8::B)),      // 0x90
            0b1001_0001 => Self::BitResetR8((2, R8::C)),      // 0x91
            0b1001_0010 => Self::BitResetR8((2, R8::D)),      // 0x92
            0b1001_0011 => Self::BitResetR8((2, R8::E)),      // 0x93
            0b1001_0100 => Self::BitResetR8((2, R8::H)),      // 0x94
            0b1001_0101 => Self::BitResetR8((2, R8::L)),      // 0x95
            0b1001_0110 => Self::BitResetR8((2, R8::HL)),     // 0x96
            0b1001_0111 => Self::BitResetR8((2, R8::A)),      // 0x97
            0b1001_1000 => Self::BitResetR8((3, R8::B)),      // 0x98
            0b1001_1001 => Self::BitResetR8((3, R8::C)),      // 0x99
            0b1001_1010 => Self::BitResetR8((3, R8::D)),      // 0x9A
            0b1001_1011 => Self::BitResetR8((3, R8::E)),      // 0x9B
            0b1001_1100 => Self::BitResetR8((3, R8::H)),      // 0x9C
            0b1001_1101 => Self::BitResetR8((3, R8::L)),      // 0x9D
            0b1001_1110 => Self::BitResetR8((3, R8::HL)),     // 0x9E
            0b1001_1111 => Self::BitResetR8((3, R8::A)),      // 0x9F
            0b1010_0000 => Self::BitResetR8((4, R8::B)),      // 0xA0
            0b1010_0001 => Self::BitResetR8((4, R8::C)),      // 0xA1
            0b1010_0010 => Self::BitResetR8((4, R8::D)),      // 0xA2
            0b1010_0011 => Self::BitResetR8((4, R8::E)),      // 0xA3
            0b1010_0100 => Self::BitResetR8((4, R8::H)),      // 0xA4
            0b1010_0101 => Self::BitResetR8((4, R8::L)),      // 0xA5
            0b1010_0110 => Self::BitResetR8((4, R8::HL)),     // 0xA6
            0b1010_0111 => Self::BitResetR8((4, R8::A)),      // 0xA7
            0b1010_1000 => Self::BitResetR8((5, R8::B)),      // 0xA8
            0b1010_1001 => Self::BitResetR8((5, R8::C)),      // 0xA9
            0b1010_1010 => Self::BitResetR8((5, R8::D)),      // 0xAA
            0b1010_1011 => Self::BitResetR8((5, R8::E)),      // 0xAB
            0b1010_1100 => Self::BitResetR8((5, R8::H)),      // 0xAC
            0b1010_1101 => Self::BitResetR8((5, R8::L)),      // 0xAD
            0b1010_1110 => Self::BitResetR8((5, R8::HL)),     // 0xAE
            0b1010_1111 => Self::BitResetR8((5, R8::A)),      // 0xAF
            0b1011_0000 => Self::BitResetR8((6, R8::B)),      // 0xB0
            0b1011_0001 => Self::BitResetR8((6, R8::C)),      // 0xB1
            0b1011_0010 => Self::BitResetR8((6, R8::D)),      // 0xB2
            0b1011_0011 => Self::BitResetR8((6, R8::E)),      // 0xB3
            0b1011_0100 => Self::BitResetR8((6, R8::H)),      // 0xB4
            0b1011_0101 => Self::BitResetR8((6, R8::L)),      // 0xB5
            0b1011_0110 => Self::BitResetR8((6, R8::HL)),     // 0xB6
            0b1011_0111 => Self::BitResetR8((6, R8::A)),      // 0xB7
            0b1011_1000 => Self::BitResetR8((7, R8::B)),      // 0xB8
            0b1011_1001 => Self::BitResetR8((7, R8::C)),      // 0xB9
            0b1011_1010 => Self::BitResetR8((7, R8::D)),      // 0xBA
            0b1011_1011 => Self::BitResetR8((7, R8::E)),      // 0xBB
            0b1011_1100 => Self::BitResetR8((7, R8::H)),      // 0xBC
            0b1011_1101 => Self::BitResetR8((7, R8::L)),      // 0xBD
            0b1011_1110 => Self::BitResetR8((7, R8::HL)),     // 0xBE
            0b1011_1111 => Self::BitResetR8((7, R8::A)),      // 0xBF
            0b1100_0000 => Self::BitSetR8((0, R8::B)),        // 0xC0
            0b1100_0001 => Self::BitSetR8((0, R8::C)),        // 0xC1
            0b1100_0010 => Self::BitSetR8((0, R8::D)),        // 0xC2
            0b1100_0011 => Self::BitSetR8((0, R8::E)),        // 0xC3
            0b1100_0100 => Self::BitSetR8((0, R8::H)),        // 0xC4
            0b1100_0101 => Self::BitSetR8((0, R8::L)),        // 0xC5
            0b1100_0110 => Self::BitSetR8((0, R8::HL)),       // 0xC6
            0b1100_0111 => Self::BitSetR8((0, R8::A)),        // 0xC7
            0b1100_1000 => Self::BitSetR8((1, R8::B)),        // 0xC8
            0b1100_1001 => Self::BitSetR8((1, R8::C)),        // 0xC9
            0b1100_1010 => Self::BitSetR8((1, R8::D)),        // 0xCA
            0b1100_1011 => Self::BitSetR8((1, R8::E)),        // 0xCB
            0b1100_1100 => Self::BitSetR8((1, R8::H)),        // 0xCC
            0b1100_1101 => Self::BitSetR8((1, R8::L)),        // 0xCD
            0b1100_1110 => Self::BitSetR8((1, R8::HL)),       // 0xCE
            0b1100_1111 => Self::BitSetR8((1, R8::A)),        // 0xCF
            0b1101_0000 => Self::BitSetR8((2, R8::B)),        // 0xD0
            0b1101_0001 => Self::BitSetR8((2, R8::C)),        // 0xD1
            0b1101_0010 => Self::BitSetR8((2, R8::D)),        // 0xD2
            0b1101_0011 => Self::BitSetR8((2, R8::E)),        // 0xD3
            0b1101_0100 => Self::BitSetR8((2, R8::H)),        // 0xD4
            0b1101_0101 => Self::BitSetR8((2, R8::L)),        // 0xD5
            0b1101_0110 => Self::BitSetR8((2, R8::HL)),       // 0xD6
            0b1101_0111 => Self::BitSetR8((2, R8::A)),        // 0xD7
            0b1101_1000 => Self::BitSetR8((3, R8::B)),        // 0xD8
            0b1101_1001 => Self::BitSetR8((3, R8::C)),        // 0xD9
            0b1101_1010 => Self::BitSetR8((3, R8::D)),        // 0xDA
            0b1101_1011 => Self::BitSetR8((3, R8::E)),        // 0xDB
            0b1101_1100 => Self::BitSetR8((3, R8::H)),        // 0xDC
            0b1101_1101 => Self::BitSetR8((3, R8::L)),        // 0xDD
            0b1101_1110 => Self::BitSetR8((3, R8::HL)),       // 0xDE
            0b1101_1111 => Self::BitSetR8((3, R8::A)),        // 0xDF
            0b1110_0000 => Self::BitSetR8((4, R8::B)),        // 0xE0
            0b1110_0001 => Self::BitSetR8((4, R8::C)),        // 0xE1
            0b1110_0010 => Self::BitSetR8((4, R8::D)),        // 0xE2
            0b1110_0011 => Self::BitSetR8((4, R8::E)),        // 0xE3
            0b1110_0100 => Self::BitSetR8((4, R8::H)),        // 0xE4
            0b1110_0101 => Self::BitSetR8((4, R8::L)),        // 0xE5
            0b1110_0110 => Self::BitSetR8((4, R8::HL)),       // 0xE6
            0b1110_0111 => Self::BitSetR8((4, R8::A)),        // 0xE7
            0b1110_1000 => Self::BitSetR8((5, R8::B)),        // 0xE8
            0b1110_1001 => Self::BitSetR8((5, R8::C)),        // 0xE9
            0b1110_1010 => Self::BitSetR8((5, R8::D)),        // 0xEA
            0b1110_1011 => Self::BitSetR8((5, R8::E)),        // 0xEB
            0b1110_1100 => Self::BitSetR8((5, R8::H)),        // 0xEC
            0b1110_1101 => Self::BitSetR8((5, R8::L)),        // 0xED
            0b1110_1110 => Self::BitSetR8((5, R8::HL)),       // 0xEE
            0b1110_1111 => Self::BitSetR8((5, R8::A)),        // 0xEF
            0b1111_0000 => Self::BitSetR8((6, R8::B)),        // 0xF0
            0b1111_0001 => Self::BitSetR8((6, R8::C)),        // 0xF1
            0b1111_0010 => Self::BitSetR8((6, R8::D)),        // 0xF2
            0b1111_0011 => Self::BitSetR8((6, R8::E)),        // 0xF3
            0b1111_0100 => Self::BitSetR8((6, R8::H)),        // 0xF4
            0b1111_0101 => Self::BitSetR8((6, R8::L)),        // 0xF5
            0b1111_0110 => Self::BitSetR8((6, R8::HL)),       // 0xF6
            0b1111_0111 => Self::BitSetR8((6, R8::A)),        // 0xF7
            0b1111_1000 => Self::BitSetR8((7, R8::B)),        // 0xF8
            0b1111_1001 => Self::BitSetR8((7, R8::C)),        // 0xF9
            0b1111_1010 => Self::BitSetR8((7, R8::D)),        // 0xFA
            0b1111_1011 => Self::BitSetR8((7, R8::E)),        // 0xFB
            0b1111_1100 => Self::BitSetR8((7, R8::H)),        // 0xFC
            0b1111_1101 => Self::BitSetR8((7, R8::L)),        // 0xFD
            0b1111_1110 => Self::BitSetR8((7, R8::HL)),       // 0xFE
            0b1111_1111 => Self::BitSetR8((7, R8::A)),        // 0xFF
            _ => panic!("Illegal prefixed instruction byte: {:02X}", byte),
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
            | Self::RotateLeftCircularA
            | Self::RotateRightCircularA
            | Self::SetCarryFlag
            | Self::DAA
            | Self::DisableInterrupts
            | Self::EnableInterrupts
            | Self::Return
            | Self::ReturnCondition(_)
            | Self::ReturnEnableInterrupts
            | Self::Halt
            | Self::LoadR8R8(_)
            | Self::AddCarryR8(_)
            | Self::SubR8(_)
            | Self::SubCarryR8(_)
            | Self::AndR8(_)
            | Self::XorR8(_)
            | Self::OrR8(_)
            | Self::CompareR8(_)
            | Self::LoadHighAC
            | Self::LoadHighCA
            | Self::LoadSpHl
            | Self::RestartVector(_) => 1,
            Self::LoadR8Imm8(_)
            | Self::JrImm8
            | Self::JrCondImm8(_)
            | Self::AddImm8
            | Self::AddCarryImm8
            | Self::AndImm8
            | Self::CompareImm8
            | Self::OrImm8
            | Self::SubImm8
            | Self::SubCarryImm8
            | Self::XorImm8
            | Self::LoadHighAImm8
            | Self::LoadHighImm8A
            | Self::AddSpImm8
            | Self::LoadHlSpImm8
            | Self::BitCheckR8(_)
            | Self::BitResetR8(_)
            | Self::BitSetR8(_)
            | Self::RotateLeftR8(_)
            | Self::RotateLeftCircularR8(_)
            | Self::RotateRightR8(_)
            | Self::RotateRightCircularR8(_)
            | Self::ShiftLeftR8(_)
            | Self::ShiftRightR8(_)
            | Self::SwapR8(_)
            | Self::ShiftRightLogicallyR8(_) => 2,
            Self::JpImm16
            | Self::JpCondImm16(_)
            | Self::LoadR16Imm16(_)
            | Self::LoadImm16SP
            | Self::LoadAImm16
            | Self::LoadImm16A
            | Self::Call
            | Self::CallCondition(_) => 3,
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
            Self::AddImm8 => format!("ADD A, 0x{:02X}", lsb),
            Self::AddCarryR8(r8) => format!("ADC A, {r8}"),
            Self::AddCarryImm8 => format!("ADC A, 0x{:02X}", lsb),
            Self::AddSpImm8 => format!("ADD SP, 0x{:02X}", lsb as i8),
            Self::AndR8(r8) => format!("AND A, {r8}"),
            Self::AndImm8 => format!("AND A, 0x{:02X}", lsb),
            Self::Call => format!("CALL 0x{:02X}{:02X}", msb, lsb),
            Self::CallCondition(cond) => format!("CALL {cond}, 0x{:02X}{:02X}", msb, lsb),
            Self::CompareR8(r8) => format!("CP A, {r8}"),
            Self::CompareImm8 => format!("CP A, 0x{:02X}", lsb),
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
            Self::JrImm8 => format!("JR 0x{:02X}", lsb as i8),
            Self::JrCondImm8(cond) => format!("JR {cond}, 0x{:02X}", lsb as i8),
            Self::LoadAR16(r16_mem) => format!("LD A, {r16_mem}"),
            Self::LoadR16A(r16_mem) => format!("LD {r16_mem}, A"),
            Self::LoadR16Imm16(r16) => format!("LD {r16}, 0x{:02X}{:02X}", msb, lsb),
            Self::LoadR8Imm8(r8) => format!("LD {r8}, 0x{:02X}", lsb),
            Self::LoadR8R8((target, source)) => format!("LD {target}, {source}"),
            Self::LoadHighAC => "LDH A, [0xFF00+C]".into(),
            Self::LoadHighCA => "LDH [0xFF00+C], A".into(),
            Self::LoadHighAImm8 => format!("LDH A, [0xFF00+{:02X}]", lsb),
            Self::LoadHighImm8A => format!("LDH [0xFF00+{:02X}], A", lsb),
            Self::LoadAImm16 => format!("LD A, 0x{:02X}{:02X}", msb, lsb),
            Self::LoadImm16A => format!("LD 0x{:02X}{:02X}, A", msb, lsb),
            Self::LoadImm16SP => format!("LD 0x{:02X}{:02X}, SP", msb, lsb),
            Self::LoadHlSpImm8 => format!("LD HL, SP+0x{:02X}", lsb as i8),
            Self::LoadSpHl => "LD SP, HL".into(),
            Self::OrR8(r8) => format!("OR A, {r8}"),
            Self::OrImm8 => format!("OR A, 0x{:02X}", lsb),
            Self::PopR16(r16_stack) => format!("POP {r16_stack}"),
            Self::PushR16(r16_stack) => format!("PUSH {r16_stack}"),
            Self::RestartVector(address) => format!("RST 0x{:02X}", address),
            Self::Return => "RET".into(),
            Self::ReturnCondition(cond) => format!("RET {cond}"),
            Self::ReturnEnableInterrupts => "RETI".into(),
            Self::RotateLeftA => "RLA".into(),
            Self::RotateRightA => "RRA".into(),
            Self::RotateLeftCircularA => "RLCA".into(),
            Self::RotateRightCircularA => "RRCA".into(),
            Self::SetCarryFlag => "SCF".into(),
            Self::SubR8(r8) => format!("SUB A, {r8}"),
            Self::SubImm8 => format!("SUB A, 0x{:02X}", lsb),
            Self::SubCarryR8(r8) => format!("SBC A, {r8}"),
            Self::SubCarryImm8 => format!("SBC A, 0x{:02X}", lsb),
            Self::XorR8(r8) => format!("XOR A, {r8}"),
            Self::XorImm8 => format!("XOR A, 0x{:02X}", lsb),
            Self::BitCheckR8((u8, r8)) => format!("BIT {u8}, {r8}"),
            Self::BitResetR8((u8, r8)) => format!("RES {u8}, {r8}"),
            Self::BitSetR8((u8, r8)) => format!("SET {u8}, {r8}"),
            Self::RotateLeftR8(r8) => format!("RL {r8}"),
            Self::RotateLeftCircularR8(r8) => format!("RLC {r8}"),
            Self::RotateRightR8(r8) => format!("RR {r8}"),
            Self::RotateRightCircularR8(r8) => format!("RRC {r8}"),
            Self::ShiftLeftR8(r8) => format!("SLA {r8}"),
            Self::ShiftRightR8(r8) => format!("SRA {r8}"),
            Self::SwapR8(r8) => format!("SWAP {r8}"),
            Self::ShiftRightLogicallyR8(r8) => format!("SRL {r8}"),
        }
    }

    /// Takes in the 2 following bytes after the instruction
    pub fn parse_description(&self, lsb: u8, msb: u8) -> String {
        match self {
            Self::Nop => "No Operation".into(),
            Self::AddHLR16(r16) => format!("Add value from register {r16} to register HL"),
            Self::AddR8(r8) => format!("Add value from register {r8} to register A"),
            Self::AddImm8 => format!("Add 0x{:02X} to register A", lsb),
            Self::AddCarryR8(r8) => {
                format!("Add value from register {r8} (and the current carry) to register A")
            }
            Self::AddCarryImm8 => format!("Add 0x{:02X} (and the current carry) to register A", lsb),
            Self::AddSpImm8 => format!("Add 0x{:02X} to the stack pointer", lsb as i8),
            Self::AndR8(r8) => format!("Bitwise AND value in register {r8} to register A"),
            Self::AndImm8 => format!("Bitwise AND 0x{:02X} to register A", lsb),
            Self::Call => format!("Call function at address 0x{:02X}{:02X}", lsb, msb),
            Self::CallCondition(cond) => format!("If {}, call function at address 0x{:02X}{:02X}", cond.description(), lsb, msb),
            Self::CompareR8(r8) => format!("Subtract value in register {r8} from register A but discard the result (comparison)"),
            Self::CompareImm8 => format!("Subtract 0x{:02X} from register A but discard the result (comparison)", lsb),
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
                "Jump relative, adding 0x{:02X} to the program counter",
                lsb as i8
            ),
            Self::JrCondImm8(cond) => {
                format!(
                    "If {}, jump relative, adding 0x{:02X} to the program counter",
                    cond.description(),
                    lsb as i8,
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
                format!("Load 0x{:02X} into register {r8}", lsb)
            }
            Self::LoadR8R8((target, source)) => {
                format!("Load value in register {source} into register {target}")
            }
            Self::LoadHighAC => {
                "Load value from address 0xFF00+C into register A".into()
            }
            Self::LoadHighCA => {
                "Load value from register A into address 0xFF00+C".into()
            }
            Self::LoadHighAImm8 => {
                format!("Load value from address 0xFF{:02X} into register A", lsb)
            }
            Self::LoadHighImm8A => {
                format!("Load value from register A into address 0xFF{:02X}", lsb)
            }
            Self::LoadAImm16 => {
                format!("Load value from address 0x{:02X}{:02X} into register A", msb, lsb)
            }
            Self::LoadImm16A => {
                format!("Load value from register A into address 0x{:02X}{:02X}", msb, lsb)
            }
            Self::LoadImm16SP => {
                format!(
                    "Load value from the top of the stack into address 0x{:02X}{:02X}",
                    msb, lsb
                )
            }
            Self::LoadHlSpImm8 => {
                format!("Load SP+0x{:02X} into register HL", lsb as i8)
            }
            Self::LoadSpHl => {
                "Load value from register HL into register SP".into()
            }
            Self::OrR8(r8) => format!("Bitwise OR value in register {r8} to register A"),
            Self::OrImm8 => format!("Bitwise OR 0x{:02X} to register A", lsb),
            Self::PopR16(r16_stack) => format!("Pop value from stack into register {r16_stack}"),
            Self::PushR16(r16_stack) => format!("Push value in {r16_stack} onto the stack"),
            Self::RestartVector(address) => format!("Restart vector to address 0x{:02X}", address),
            Self::Return => "Return from a called function".into(),
            Self::ReturnCondition(cond) => {
                format!("If {}, return from a called function", cond.description())
            }
            Self::ReturnEnableInterrupts => {
                "Return from a called function and enable interrupts".into()
            }
            Self::RotateLeftA => "Rotate register A left THROUGH the carry flag".into(),
            Self::RotateRightA => "Rotate register A right THROUGH the carry flag".into(),
            Self::RotateLeftCircularA => "Rotate register A left, update carry flag".into(),
            Self::RotateRightCircularA => "Rotate register A right, update carry flag".into(),
            Self::SetCarryFlag => "Set carry flag".into(),
            Self::SubR8(r8) => format!("Subtract value in register {r8} from register A"),
            Self::SubImm8 => format!("Subtract 0x{:02X} from register A", lsb),
            Self::SubCarryR8(r8) => {
                format!("Subtract value in register {r8} (and the current carry) from register A")
            }
            Self::SubCarryImm8 => format!("Subtract 0x{:02X} (and the current carry) from register A", lsb),
            Self::XorR8(r8) => format!("Bitwise XOR value in register {r8} to register A"),
            Self::XorImm8 => format!("Bitwise XOR 0x{:02X} to register A", lsb),
            Self::BitCheckR8((u8, r8)) => format!("Check bit at index {u8} of register {r8} and set zero flag if its 0"),
            Self::BitResetR8((u8, r8)) => format!("Reset bit at index {u8} of register {r8}"),
            Self::BitSetR8((u8, r8)) => format!("Set bit at index {u8} of register {r8}"),
            Self::RotateLeftR8(r8) => format!("Rotate register {r8} left THROUGH the carry flag"),
            Self::RotateLeftCircularR8(r8) => format!("Rotate register {r8} left carry, update carry flag"),
            Self::RotateRightR8(r8) => format!("Rotate register {r8} right THROUGH the carry flag"),
            Self::RotateRightCircularR8(r8) => format!("Rotate register {r8} right, update carry flag"),
            Self::ShiftLeftR8(r8) => format!("Shift register {r8} left (fill up with 0)"),
            Self::ShiftRightR8(r8) => format!("Shift register {r8} right (persist leftmost bit)"),
            Self::SwapR8(r8) => format!("Swap upper and lower 4 bits in register {r8}"),
            Self::ShiftRightLogicallyR8(r8) => format!("Shift register {r8} right (fill up with 0)"),
        }
    }
}
