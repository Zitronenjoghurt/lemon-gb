//! These are parameter groups for CPU instructions as they are used in the Pan Docs
//! https://gbdev.io/pandocs/CPU_Instruction_Set.html

use std::fmt::Display;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum R8 {
    B = 0,
    C = 1,
    D = 2,
    E = 3,
    H = 4,
    L = 5,
    /// When this value is used as an 8-bit register, the byte HL points to is treated as the 8-Bit register
    HL = 6,
    A = 7,
}

impl Display for R8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            R8::B => write!(f, "B"),
            R8::C => write!(f, "C"),
            R8::D => write!(f, "D"),
            R8::E => write!(f, "E"),
            R8::H => write!(f, "H"),
            R8::L => write!(f, "L"),
            R8::HL => write!(f, "HL"),
            R8::A => write!(f, "A"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum R16 {
    BC = 0,
    DE = 1,
    HL = 2,
    SP = 3,
}

impl Display for R16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            R16::BC => write!(f, "BC"),
            R16::DE => write!(f, "DE"),
            R16::HL => write!(f, "HL"),
            R16::SP => write!(f, "SP"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum R16Stack {
    BC = 0,
    DE = 1,
    HL = 2,
    AF = 3,
}

impl Display for R16Stack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            R16Stack::BC => write!(f, "BC"),
            R16Stack::DE => write!(f, "DE"),
            R16Stack::HL => write!(f, "HL"),
            R16Stack::AF => write!(f, "AF"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum R16Mem {
    BC = 0,
    DE = 1,
    HLI = 2, // HL Increment (HL+)
    HLD = 3, // HL Decrement (HL-)
}

impl Display for R16Mem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            R16Mem::BC => write!(f, "BC"),
            R16Mem::DE => write!(f, "DE"),
            R16Mem::HLI => write!(f, "HL+"),
            R16Mem::HLD => write!(f, "HL-"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum JumpCondition {
    NotZero = 0,
    Zero = 1,
    NotCarry = 2,
    Carry = 3,
}

impl Display for JumpCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JumpCondition::NotZero => write!(f, "NZ"),
            JumpCondition::Zero => write!(f, "Z"),
            JumpCondition::NotCarry => write!(f, "NC"),
            JumpCondition::Carry => write!(f, "C"),
        }
    }
}

impl JumpCondition {
    pub fn description(&self) -> String {
        match self {
            JumpCondition::NotZero => "flag zero is not set".into(),
            JumpCondition::Zero => "flag zero is set".into(),
            JumpCondition::NotCarry => "flag carry is not set".into(),
            JumpCondition::Carry => "flag carry is set".into(),
        }
    }
}
