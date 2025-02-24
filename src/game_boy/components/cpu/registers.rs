use crate::enums::parameter_groups::{JumpCondition, R16Mem, R16Stack, R16, R8};
use crate::game_boy::components::cpu::registers::builder::CPURegistersBuilder;
use crate::game_boy::components::cpu::registers::flags_register::CPUFlagsRegister;
use crate::game_boy::components::mmu::MMU;
use crate::helpers::bit_operations::{construct_u16, deconstruct_u16};
use serde::{Deserialize, Serialize};

pub mod builder;
pub mod flags_register;

// Initial CPU register values according to: https://gbdev.io/pandocs/Power_Up_Sequence.html?highlight=state#console-state-after-boot-rom-hand-off
// Model: DMG0
const INITIAL_A: u8 = 0x01;
const INITIAL_B: u8 = 0xFF;
const INITIAL_C: u8 = 0x13;
const INITIAL_D: u8 = 0x00;
const INITIAL_E: u8 = 0xC1;
const INITIAL_H: u8 = 0x84;
const INITIAL_L: u8 = 0x03;
const INITIAL_PC: u16 = 0x0100;
const INITIAL_SP: u16 = 0xFFFE;

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
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

    pub fn initialize() -> Self {
        Self {
            a: INITIAL_A,
            b: INITIAL_B,
            c: INITIAL_C,
            d: INITIAL_D,
            e: INITIAL_E,
            f: CPUFlagsRegister::initialize(),
            h: INITIAL_H,
            l: INITIAL_L,
            pc: INITIAL_PC,
            sp: INITIAL_SP,
        }
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

    fn get_r8(&self, register: R8, mmu: &MMU) -> u8 {
        match register {
            R8::B => self.get_b(),
            R8::C => self.get_c(),
            R8::D => self.get_d(),
            R8::E => self.get_e(),
            R8::H => self.get_h(),
            R8::L => self.get_l(),
            R8::HL => mmu.read(self.get_hl()),
            R8::A => self.get_a(),
        }
    }

    fn set_r8(&mut self, register: R8, value: u8, mmu: &mut MMU) {
        match register {
            R8::B => self.set_b(value),
            R8::C => self.set_c(value),
            R8::D => self.set_d(value),
            R8::E => self.set_e(value),
            R8::H => self.set_h(value),
            R8::L => self.set_l(value),
            R8::HL => mmu.write(self.get_hl(), value),
            R8::A => self.set_a(value),
        }
    }

    fn get_r16(&self, register: R16) -> u16 {
        match register {
            R16::BC => self.get_bc(),
            R16::DE => self.get_de(),
            R16::HL => self.get_hl(),
            R16::SP => self.get_sp(),
        }
    }

    fn set_r16(&mut self, register: R16, value: u16) {
        match register {
            R16::BC => self.set_bc(value),
            R16::DE => self.set_de(value),
            R16::HL => self.set_hl(value),
            R16::SP => self.set_sp(value),
        }
    }

    fn get_r16_stack(&self, register: R16Stack) -> u16 {
        match register {
            R16Stack::BC => self.get_bc(),
            R16Stack::DE => self.get_de(),
            R16Stack::HL => self.get_hl(),
            R16Stack::AF => self.get_af(),
        }
    }

    fn set_r16_stack(&mut self, register: R16Stack, value: u16) {
        match register {
            R16Stack::BC => self.set_bc(value),
            R16Stack::DE => self.set_de(value),
            R16Stack::HL => self.set_hl(value),
            R16Stack::AF => self.set_af(value),
        }
    }

    fn get_r16_mem(&self, register: R16Mem) -> u16 {
        match register {
            R16Mem::BC => self.get_bc(),
            R16Mem::DE => self.get_de(),
            R16Mem::HLI | R16Mem::HLD => self.get_hl(),
        }
    }

    fn check_jump_condition(&self, condition: JumpCondition) -> bool {
        match condition {
            JumpCondition::NotZero => !self.get_f_zero(),
            JumpCondition::Zero => self.get_f_zero(),
            JumpCondition::NotCarry => !self.get_f_carry(),
            JumpCondition::Carry => self.get_f_carry(),
        }
    }

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

    fn set_f(&mut self, value: u8) {
        self.get_registers_mut().f = value.into()
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

    fn increment_sp(&mut self) {
        self.set_sp(self.get_sp().wrapping_add(1));
    }

    fn decrement_sp(&mut self) {
        self.set_sp(self.get_sp().wrapping_sub(1));
    }

    fn get_af(&self) -> u16 {
        construct_u16(self.get_f(), self.get_a())
    }

    fn set_af(&mut self, value: u16) {
        let (f, a) = deconstruct_u16(value);
        self.set_f(f);
        self.set_a(a);
    }

    fn get_bc(&self) -> u16 {
        construct_u16(self.get_c(), self.get_b())
    }

    fn set_bc(&mut self, value: u16) {
        let (c, b) = deconstruct_u16(value);
        self.set_c(c);
        self.set_b(b);
    }

    fn get_de(&self) -> u16 {
        construct_u16(self.get_e(), self.get_d())
    }

    fn set_de(&mut self, value: u16) {
        let (e, d) = deconstruct_u16(value);
        self.set_e(e);
        self.set_d(d);
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
