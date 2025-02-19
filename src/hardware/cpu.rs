use crate::enums::jump_condition::JumpCondition;
use crate::enums::register_8::Register8;
use crate::hardware::cpu::builder::CpuBuilder;
use crate::hardware::cpu::memory_bus::MemoryBus;
use crate::hardware::registers::Registers;
use crate::helpers::bit_operations::unsigned_16;
use crate::instructions::Instruction;

mod builder;
pub mod memory_bus;

/// This tells the CPU that the next instruction to be executed is a prefixed instruction
const PREFIX_INSTRUCTION_BYTE: u8 = 0xCB;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Cpu {
    registers: Registers,
    pc: u16,
    memory: MemoryBus,
}

impl Cpu {
    pub fn builder() -> CpuBuilder {
        CpuBuilder::new()
    }

    pub fn execute(&mut self, instruction: Instruction) -> (u16, u8) {
        match instruction {
            Instruction::Add(source_r8) => self.add(source_r8),
            Instruction::JpCondImm(condition) => self.jump_condition_imm(condition),
        }
    }

    pub fn step(&mut self) -> u8 {
        let mut instruction_byte = self.memory.read(self.pc);
        let prefixed = instruction_byte == PREFIX_INSTRUCTION_BYTE;
        if prefixed {
            instruction_byte = self.memory.read(self.pc + 1);
        }

        let instruction = Instruction::from_byte(instruction_byte, prefixed);
        let (next_pc, m_cycles) = self.execute(instruction);
        self.pc = next_pc;

        m_cycles
    }

    pub fn get_registers(&self) -> &Registers {
        &self.registers
    }

    pub fn set_registers(&mut self, registers: Registers) {
        self.registers = registers;
    }

    pub fn get_pc(&self) -> u16 {
        self.pc
    }

    pub fn set_pc(&mut self, pc: u16) {
        self.pc = pc;
    }

    pub fn get_memory(&self) -> &MemoryBus {
        &self.memory
    }

    pub fn set_memory(&mut self, memory: MemoryBus) {
        self.memory = memory;
    }
}

/// Memory Operations
impl Cpu {
    pub fn get_value_r8(&self, register: Register8) -> u8 {
        match register {
            Register8::B => self.registers.get_b(),
            Register8::C => self.registers.get_c(),
            Register8::D => self.registers.get_d(),
            Register8::E => self.registers.get_e(),
            Register8::H => self.registers.get_h(),
            Register8::L => self.registers.get_l(),
            Register8::HL => self.memory.read(self.registers.get_hl()),
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

/// Arithmetic operations
impl Cpu {
    pub fn add(&mut self, source_r8: Register8) -> (u16, u8) {
        let source_value = self.get_value_r8(source_r8);
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

    pub fn jump_condition_imm(&self, condition: JumpCondition) -> (u16, u8) {
        let should_jump = self.check_jump_condition(condition);

        if should_jump {
            let least_significant_byte = self.memory.read(self.pc + 1);
            let most_significant_byte = self.memory.read(self.pc + 2);
            let new_pc = unsigned_16(least_significant_byte, most_significant_byte);
            (new_pc, 4)
        } else {
            self.instruction_result(3, 3)
        }
    }
}

/// Helper functions
impl Cpu {
    fn instruction_result(&self, pc_raise: u16, m_cycles: u8) -> (u16, u8) {
        (self.pc.wrapping_add(pc_raise), m_cycles)
    }
}
