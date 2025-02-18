const ZERO_FLAG: u8 = 0b1000_0000;
const SUBTRACT_FLAG: u8 = 0b0100_0000;
const HALF_CARRY_FLAG: u8 = 0b0010_0000;
const CARRY_FLAG: u8 = 0b0001_0000;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct FlagsRegister {
    /// Set to true if the result of the operation is equal to 0
    zero: bool,
    /// Set to true if the operation was a subtraction
    subtract: bool,
    /// Set to true if the operation resulted in an overflow
    half_carry: bool,
    /// Set to true if there was an overflow from the lower 4 bits to the upper 4 bits
    carry: bool,
}

impl FlagsRegister {
    pub fn get_zero(&self) -> bool {
        self.zero
    }

    pub fn set_zero(&mut self, value: bool) {
        self.zero = value;
    }

    pub fn get_subtract(&self) -> bool {
        self.subtract
    }

    pub fn set_subtract(&mut self, value: bool) {
        self.subtract = value;
    }

    pub fn get_half_carry(&self) -> bool {
        self.half_carry
    }

    pub fn set_half_carry(&mut self, value: bool) {
        self.half_carry = value;
    }

    pub fn get_carry(&self) -> bool {
        self.carry
    }

    pub fn set_carry(&mut self, value: bool) {
        self.carry = value;
    }
}

impl From<FlagsRegister> for u8 {
    fn from(value: FlagsRegister) -> Self {
        (if value.zero { ZERO_FLAG } else { 0 })
            | (if value.subtract { SUBTRACT_FLAG } else { 0 })
            | (if value.half_carry { HALF_CARRY_FLAG } else { 0 })
            | (if value.carry { CARRY_FLAG } else { 0 })
    }
}

impl From<u8> for FlagsRegister {
    fn from(value: u8) -> Self {
        Self {
            zero: (value & ZERO_FLAG) != 0,
            subtract: (value & SUBTRACT_FLAG) != 0,
            half_carry: (value & HALF_CARRY_FLAG) != 0,
            carry: (value & CARRY_FLAG) != 0,
        }
    }
}
