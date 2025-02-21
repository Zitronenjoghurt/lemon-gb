use crate::enums::parameter_groups::{JumpCondition, R16};
use crate::enums::parameter_groups::{R16Stack, R8};
use crate::game_boy::components::cpu::builder::CpuBuilder;
use crate::game_boy::components::cpu::registers::CpuRegistersAccessTrait;
use crate::game_boy::components::mmu::MMU;
use crate::helpers::bit_operations::{construct_u16, deconstruct_u16};
use crate::instructions::Instruction;
use registers::CPURegisters;

mod builder;
pub mod registers;

/// This tells the CPU that the next instruction to be executed is a prefixed instruction
pub const PREFIX_INSTRUCTION_BYTE: u8 = 0xCB;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct CPU {
    registers: CPURegisters,
    /// Interrupt Master Enable Flag
    /// When this flag is enabled, interrupts will be acknowledged, else they will be ignored
    ime: bool,
}

impl CPU {
    pub fn builder() -> CpuBuilder {
        CpuBuilder::new()
    }

    pub fn initialize() -> Self {
        Self {
            registers: CPURegisters::initialize(),
            ..Default::default()
        }
    }

    /// Returns (New PC, M Cycles taken)
    pub fn execute(&mut self, instruction: Instruction, mmu: &mut MMU) -> (u16, u8) {
        match instruction {
            Instruction::Add(r8) => self.add(r8, mmu),
            Instruction::JpHL => self.jump_hl(),
            Instruction::JpImm16 => self.jump_imm(mmu),
            Instruction::JpCondImm16(condition) => self.jump_condition_imm(condition, mmu),
            Instruction::LoadR16Imm16(r16) => self.load_r16_imm(r16, mmu),
            Instruction::Nop => self.instruction_result(1, 1),
            Instruction::PopR16(r16_stack) => self.pop_r16(r16_stack, mmu),
            Instruction::PushR16(r16_stack) => self.push_r16(r16_stack, mmu),
        }
    }

    pub fn step(&mut self, mmu: &mut MMU) -> u8 {
        let mut instruction_byte = mmu.read(self.get_pc());
        let prefixed = instruction_byte == PREFIX_INSTRUCTION_BYTE;
        if prefixed {
            instruction_byte = mmu.read(self.get_pc().wrapping_add(1));
        }

        let instruction = Instruction::from_byte(instruction_byte, prefixed).unwrap();
        let (next_pc, m_cycles) = self.execute(instruction, mmu);
        self.set_pc(next_pc);

        m_cycles
    }

    pub fn set_registers(&mut self, registers: CPURegisters) {
        self.registers = registers;
    }
}

/// Direct instruction interfaces
impl CPU {
    pub fn load_r16_imm(&mut self, r16: R16, mmu: &MMU) -> (u16, u8) {
        let value = self.read_next_imm16(mmu);
        self.set_r16(r16, value);
        self.instruction_result(3, 3)
    }

    pub fn add(&mut self, r8: R8, mmu: &MMU) -> (u16, u8) {
        let source_value = self.get_r8(r8, mmu);
        let (new_value, did_overflow) = self.get_a().overflowing_add(source_value);

        self.set_f_zero(new_value == 0);
        self.set_f_subtract(false);
        self.set_f_carry(did_overflow);

        let half_carry = (self.get_a() & 0xF) + (source_value & 0xF) > 0xF;
        self.set_f_half_carry(half_carry);

        self.set_a(new_value);

        let m = if r8 == R8::HL { 3 } else { 2 };
        self.instruction_result(1, m)
    }

    pub fn pop_r16(&mut self, r16_stack: R16Stack, mmu: &MMU) -> (u16, u8) {
        let lsb = self.pop(mmu);
        let msb = self.pop(mmu);
        let value = construct_u16(lsb, msb);
        self.set_r16_stack(r16_stack, value);
        self.instruction_result(1, 3)
    }

    pub fn push_r16(&mut self, r16_stack: R16Stack, mmu: &mut MMU) -> (u16, u8) {
        let value = self.get_r16_stack(r16_stack);
        let (lsb, msb) = deconstruct_u16(value);
        self.push(msb, mmu);
        self.push(lsb, mmu);
        self.instruction_result(1, 4)
    }

    pub fn jump_hl(&mut self) -> (u16, u8) {
        let new_pc = self.get_hl();
        (new_pc, 1)
    }

    pub fn jump_imm(&self, mmu: &MMU) -> (u16, u8) {
        let new_pc = self.read_next_imm16(mmu);
        (new_pc, 4)
    }

    pub fn jump_condition_imm(&self, condition: JumpCondition, mmu: &MMU) -> (u16, u8) {
        let should_jump = self.check_jump_condition(condition);

        if should_jump {
            self.jump_imm(mmu)
        } else {
            self.instruction_result(3, 3)
        }
    }
}

/// Basic operations
impl CPU {
    pub fn pop(&mut self, mmu: &MMU) -> u8 {
        let value = mmu.read(self.get_sp());
        self.increment_sp();
        value
    }

    pub fn push(&mut self, value: u8, mmu: &mut MMU) {
        self.decrement_sp();
        mmu.write(self.get_sp(), value);
    }
}

/// Helper functions
impl CPU {
    fn instruction_result(&self, pc_raise: u16, m_cycles: u8) -> (u16, u8) {
        (self.get_pc().wrapping_add(pc_raise), m_cycles)
    }

    fn read_next_imm16(&self, mmu: &MMU) -> u16 {
        mmu.read_16(self.get_pc().wrapping_add(1))
    }
}

impl CpuRegistersAccessTrait for CPU {
    fn get_registers(&self) -> &CPURegisters {
        &self.registers
    }

    fn get_registers_mut(&mut self) -> &mut CPURegisters {
        &mut self.registers
    }
}
