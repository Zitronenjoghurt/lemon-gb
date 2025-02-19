use crate::hardware::registers::Registers;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct RegistersBuilder {
    registers: Registers,
}

impl RegistersBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Registers {
        self.registers
    }

    pub fn a(mut self, value: u8) -> Self {
        self.registers.set_a(value);
        self
    }

    pub fn b(mut self, value: u8) -> Self {
        self.registers.set_b(value);
        self
    }

    pub fn c(mut self, value: u8) -> Self {
        self.registers.set_c(value);
        self
    }

    pub fn d(mut self, value: u8) -> Self {
        self.registers.set_d(value);
        self
    }

    pub fn e(mut self, value: u8) -> Self {
        self.registers.set_e(value);
        self
    }

    pub fn f_zero(mut self, value: bool) -> Self {
        self.registers.set_f_zero(value);
        self
    }

    pub fn f_subtract(mut self, value: bool) -> Self {
        self.registers.set_f_subtract(value);
        self
    }

    pub fn f_half_carry(mut self, value: bool) -> Self {
        self.registers.set_f_half_carry(value);
        self
    }

    pub fn f_carry(mut self, value: bool) -> Self {
        self.registers.set_f_carry(value);
        self
    }

    pub fn h(mut self, value: u8) -> Self {
        self.registers.set_h(value);
        self
    }

    pub fn l(mut self, value: u8) -> Self {
        self.registers.set_l(value);
        self
    }

    pub fn hl(mut self, value: u16) -> Self {
        self.registers.set_hl(value);
        self
    }
}
