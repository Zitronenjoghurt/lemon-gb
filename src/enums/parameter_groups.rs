//! These are parameter groups for CPU instructions as they are used in the Pan Docs
//! https://gbdev.io/pandocs/CPU_Instruction_Set.html

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum R8 {
    B = 0,
    C = 1,
    D = 2,
    E = 3,
    H = 4,
    L = 5,
    HL = 6,
    A = 7,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum R16 {
    BC = 0,
    DE = 1,
    HL = 2,
    SP = 3,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum JumpCondition {
    NotZero = 0,
    Zero = 1,
    NotCarry = 2,
    Carry = 3,
}
