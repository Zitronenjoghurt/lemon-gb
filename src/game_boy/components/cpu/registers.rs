use crate::game_boy::components::cpu::registers::builder::CPURegistersBuilder;
use crate::game_boy::components::cpu::registers::flags_register::CPUFlagsRegister;
use crate::helpers::bit_operations::{construct_u16, deconstruct_u16};

pub mod builder;
pub mod flags_register;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct CPURegisters {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: CPUFlagsRegister,
    h: u8,
    l: u8,
    /// Program counter - points to the memory address storing the next instruction to execute
    pc: u16,
    /// Stack pointer - points to the memory address storing the current top of the stack
    sp: u16,
}

impl CPURegisters {
    pub fn builder() -> CPURegistersBuilder {
        CPURegistersBuilder::new()
    }
}

impl CpuRegistersAccessTrait for CPURegisters {
    fn get_registers(&self) -> &CPURegisters {
        self
    }

    fn get_registers_mut(&mut self) -> &mut CPURegisters {
        self
    }
}

pub trait CpuRegistersAccessTrait {
    fn get_registers(&self) -> &CPURegisters;
    fn get_registers_mut(&mut self) -> &mut CPURegisters;

    fn get_a(&self) -> u8 {
        self.get_registers().a
    }

    fn set_a(&mut self, value: u8) {
        self.get_registers_mut().a = value;
    }

    fn get_b(&self) -> u8 {
        self.get_registers().b
    }

    fn set_b(&mut self, value: u8) {
        self.get_registers_mut().b = value;
    }

    fn get_c(&self) -> u8 {
        self.get_registers().c
    }

    fn set_c(&mut self, value: u8) {
        self.get_registers_mut().c = value;
    }

    fn get_d(&self) -> u8 {
        self.get_registers().d
    }

    fn set_d(&mut self, value: u8) {
        self.get_registers_mut().d = value;
    }

    fn get_e(&self) -> u8 {
        self.get_registers().e
    }

    fn set_e(&mut self, value: u8) {
        self.get_registers_mut().e = value;
    }

    fn get_h(&self) -> u8 {
        self.get_registers().h
    }

    fn set_h(&mut self, value: u8) {
        self.get_registers_mut().h = value;
    }

    fn get_l(&self) -> u8 {
        self.get_registers().l
    }

    fn set_l(&mut self, value: u8) {
        self.get_registers_mut().l = value;
    }

    fn get_f(&self) -> u8 {
        self.get_registers().f.into()
    }

    fn get_f_zero(&self) -> bool {
        self.get_registers().f.get_zero()
    }

    fn set_f_zero(&mut self, value: bool) {
        self.get_registers_mut().f.set_zero(value);
    }

    fn get_f_subtract(&self) -> bool {
        self.get_registers().f.get_subtract()
    }

    fn set_f_subtract(&mut self, value: bool) {
        self.get_registers_mut().f.set_subtract(value);
    }

    fn get_f_half_carry(&self) -> bool {
        self.get_registers().f.get_half_carry()
    }

    fn set_f_half_carry(&mut self, value: bool) {
        self.get_registers_mut().f.set_half_carry(value);
    }

    fn get_f_carry(&self) -> bool {
        self.get_registers().f.get_carry()
    }

    fn set_f_carry(&mut self, value: bool) {
        self.get_registers_mut().f.set_carry(value);
    }

    fn get_pc(&self) -> u16 {
        self.get_registers().pc
    }

    fn set_pc(&mut self, value: u16) {
        self.get_registers_mut().pc = value;
    }

    fn get_sp(&self) -> u16 {
        self.get_registers().sp
    }

    fn set_sp(&mut self, value: u16) {
        self.get_registers_mut().sp = value;
    }

    fn get_bc(&self) -> u16 {
        construct_u16(self.get_c(), self.get_b())
    }

    fn set_bc(&mut self, value: u16) {
        let (c, b) = deconstruct_u16(value);
        self.set_c(c);
        self.set_b(b);
    }

    fn get_hl(&self) -> u16 {
        construct_u16(self.get_l(), self.get_h())
    }

    fn set_hl(&mut self, value: u16) {
        let (l, h) = deconstruct_u16(value);
        self.set_l(l);
        self.set_h(h);
    }
}
