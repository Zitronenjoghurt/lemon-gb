use crate::enums::jump_condition::JumpCondition;
use crate::enums::register_8::Register8;
use crate::game_boy::components::cpu::builder::CpuBuilder;
use crate::game_boy::components::cpu::registers::CpuRegistersAccessTrait;
use crate::game_boy::components::mmu::MMU;
use crate::helpers::bit_operations::construct_u16;
use crate::instructions::Instruction;
use registers::CPURegisters;

mod builder;
pub mod registers;

/// This tells the CPU that the next instruction to be executed is a prefixed instruction
const PREFIX_INSTRUCTION_BYTE: u8 = 0xCB;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct CPU {
    registers: CPURegisters,
}

impl CPU {
    pub fn builder() -> CpuBuilder {
        CpuBuilder::new()
    }

    pub fn execute(&mut self, instruction: Instruction, mut mmu: &MMU) -> (u16, u8) {
        match instruction {
            Instruction::Add(source_r8) => self.add(source_r8, &mmu),
            Instruction::JpHL => self.jump_hl(),
            Instruction::JpImm => self.jump_imm(&mmu),
            Instruction::JpCondImm(condition) => self.jump_condition_imm(condition, &mmu),
        }
    }

    pub fn step(&mut self, mut mmu: &MMU) -> u8 {
        let mut instruction_byte = mmu.read(self.get_pc());
        let prefixed = instruction_byte == PREFIX_INSTRUCTION_BYTE;
        if prefixed {
            instruction_byte = mmu.read(self.get_pc() + 1);
        }

        let instruction = Instruction::from_byte(instruction_byte, prefixed);
        let (next_pc, m_cycles) = self.execute(instruction, &mut mmu);
        self.set_pc(next_pc);

        m_cycles
    }

    pub fn get_registers(&self) -> &CPURegisters {
        &self.registers
    }

    pub fn set_registers(&mut self, registers: CPURegisters) {
        self.registers = registers;
    }
}

/// Register Operations
impl CPU {
    pub fn get_value_r8(&self, register: Register8, mmu: &MMU) -> u8 {
        match register {
            Register8::B => self.registers.get_b(),
            Register8::C => self.registers.get_c(),
            Register8::D => self.registers.get_d(),
            Register8::E => self.registers.get_e(),
            Register8::H => self.registers.get_h(),
            Register8::L => self.registers.get_l(),
            Register8::HL => mmu.read(self.registers.get_hl()),
            Register8::A => self.registers.get_a(),
        }
    }

    pub fn check_jump_condition(&self, condition: JumpCondition) -> bool {
        match condition {
            JumpCondition::Always => true,
            JumpCondition::NotZero => !self.registers.get_f_zero(),
            JumpCondition::Zero => self.registers.get_f_zero(),
            JumpCondition::NotCarry => !self.registers.get_f_carry(),
            JumpCondition::Carry => self.registers.get_f_carry(),
        }
    }
}

/// Direct instruction interfaces
impl CPU {
    pub fn add(&mut self, source_r8: Register8, mmu: &MMU) -> (u16, u8) {
        let source_value = self.get_value_r8(source_r8, mmu);
        let (new_value, did_overflow) = self.registers.get_a().overflowing_add(source_value);

        self.registers.set_f_zero(new_value == 0);
        self.registers.set_f_subtract(false);
        self.registers.set_f_carry(did_overflow);

        let half_carry = (self.registers.get_a() & 0xF) + (source_value & 0xF) > 0xF;
        self.registers.set_f_half_carry(half_carry);

        self.registers.set_a(new_value);

        let m = if source_r8 == Register8::HL { 3 } else { 2 };
        self.instruction_result(1, m)
    }

    pub fn jump_hl(&mut self) -> (u16, u8) {
        (self.registers.get_hl(), 1)
    }

    pub fn jump_imm(&self, mmu: &MMU) -> (u16, u8) {
        let least_significant_byte = mmu.read(self.get_pc() + 1);
        let most_significant_byte = mmu.read(self.get_pc() + 2);
        let new_pc = construct_u16(least_significant_byte, most_significant_byte);
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

/// Helper functions
impl CPU {
    fn instruction_result(&self, pc_raise: u16, m_cycles: u8) -> (u16, u8) {
        (self.get_pc().wrapping_add(pc_raise), m_cycles)
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
