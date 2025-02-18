use crate::enums::register_8::Register8;
use crate::hardware::cpu::memory_bus::MemoryBus;
use crate::hardware::registers::Registers;
use crate::instructions::Instruction;

mod memory_bus;

/// This tells the CPU that the next instruction to be executed is a prefixed instruction
const PREFIX_INSTRUCTION_BYTE: u8 = 0xCB;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Cpu {
    registers: Registers,
    pc: u16,
    memory: MemoryBus,
}

impl Cpu {
    pub fn execute(&mut self, instruction: Instruction) -> u16 {
        match instruction {
            Instruction::Add(source_r8) => {
                self.add(source_r8);
                self.pc.wrapping_add(1)
            }
        }
    }

    pub fn step(&mut self) {
        let mut instruction_byte = self.memory.read(self.pc);
        let prefixed = instruction_byte == PREFIX_INSTRUCTION_BYTE;
        if prefixed {
            instruction_byte = self.memory.read(self.pc + 1);
        }

        let instruction = Instruction::from_byte(instruction_byte, prefixed);
        let next_pc = self.execute(instruction);
        self.pc = next_pc;
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
}

/// Arithmetic operations
impl Cpu {
    pub fn add(&mut self, source_r8: Register8) {
        let source_value = self.get_value_r8(source_r8);
        let (new_value, did_overflow) = self.registers.get_a().overflowing_add(source_value);

        self.registers.set_f_zero(new_value == 0);
        self.registers.set_f_subtract(false);
        self.registers.set_f_carry(did_overflow);

        let half_carry = (self.registers.get_a() & 0xF) + (source_value & 0xF) > 0xF;
        self.registers.set_f_half_carry(half_carry);

        self.registers.set_a(new_value);
    }
}
