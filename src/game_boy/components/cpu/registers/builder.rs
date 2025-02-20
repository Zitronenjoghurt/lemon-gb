use crate::game_boy::components::cpu::registers::{CPURegisters, CpuRegistersAccessTrait};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct CPURegistersBuilder {
    registers: CPURegisters,
}

impl CPURegistersBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> CPURegisters {
        self.registers
    }
}

pub trait CPURegistersBuilderTrait: CpuRegistersAccessTrait + Sized {
    fn a(mut self, value: u8) -> Self {
        self.get_registers_mut().set_a(value);
        self
    }

    fn b(mut self, value: u8) -> Self {
        self.get_registers_mut().set_b(value);
        self
    }

    fn c(mut self, value: u8) -> Self {
        self.get_registers_mut().set_c(value);
        self
    }

    fn d(mut self, value: u8) -> Self {
        self.get_registers_mut().set_d(value);
        self
    }

    fn e(mut self, value: u8) -> Self {
        self.get_registers_mut().set_e(value);
        self
    }

    fn f_zero(mut self, value: bool) -> Self {
        self.get_registers_mut().set_f_zero(value);
        self
    }

    fn f_subtract(mut self, value: bool) -> Self {
        self.get_registers_mut().set_f_subtract(value);
        self
    }

    fn f_half_carry(mut self, value: bool) -> Self {
        self.get_registers_mut().set_f_half_carry(value);
        self
    }

    fn f_carry(mut self, value: bool) -> Self {
        self.get_registers_mut().set_f_carry(value);
        self
    }

    fn h(mut self, value: u8) -> Self {
        self.get_registers_mut().set_h(value);
        self
    }

    fn l(mut self, value: u8) -> Self {
        self.get_registers_mut().set_l(value);
        self
    }

    fn pc(mut self, value: u16) -> Self {
        self.get_registers_mut().set_pc(value);
        self
    }

    fn sp(mut self, value: u16) -> Self {
        self.get_registers_mut().set_sp(value);
        self
    }

    fn af(mut self, value: u16) -> Self {
        self.get_registers_mut().set_af(value);
        self
    }

    fn bc(mut self, value: u16) -> Self {
        self.get_registers_mut().set_bc(value);
        self
    }

    fn de(mut self, value: u16) -> Self {
        self.get_registers_mut().set_de(value);
        self
    }

    fn hl(mut self, value: u16) -> Self {
        self.get_registers_mut().set_hl(value);
        self
    }
}
