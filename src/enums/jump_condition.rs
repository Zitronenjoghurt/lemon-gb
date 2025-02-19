#[derive(Debug, Copy, Clone, PartialEq)]
pub enum JumpCondition {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always,
}
