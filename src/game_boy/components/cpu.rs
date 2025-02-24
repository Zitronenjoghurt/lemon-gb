use crate::enums::parameter_groups::R16Stack;
use crate::enums::parameter_groups::{JumpCondition, R16Mem, R16, R8};
use crate::game_boy::components::cpu::builder::CpuBuilder;
use crate::game_boy::components::cpu::registers::CpuRegistersAccessTrait;
use crate::game_boy::components::mmu::{IF_ADDRESS, MMU};
use crate::helpers::bit_operations::*;
use crate::instructions::Instruction;
use log::info;
use registers::CPURegisters;
use serde::{Deserialize, Serialize};

mod builder;
pub mod registers;

/// This tells the CPU that the next instruction to be executed is a prefixed instruction
pub const PREFIX_INSTRUCTION_BYTE: u8 = 0xCB;

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct CPU {
    registers: CPURegisters,
    /// Interrupt Master Enable Flag
    /// When this flag is enabled, interrupts will be acknowledged, else they will be ignored
    ime: bool,
    /// True if the IME is to be set after handling the current instruction
    deferred_set_ime: bool,
    /// If the CPU is in low power mode
    eeping: bool,
    /// This is true when the program counter should not be incremented
    halting_bug_active: bool,
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
            Instruction::AddHLR16(r16) => self.add_hl_r16(r16),
            Instruction::AddR8(r8) => self.add_r8(r8, mmu),
            Instruction::AddCarryR8(r8) => self.add_carry_r8(r8, mmu),
            Instruction::ComplementA => self.complement_a(),
            Instruction::ComplementCarryFlag => self.complement_carry(),
            Instruction::DAA => self.decimal_adjust_accumulator(),
            Instruction::DecR8(r8) => self.decrement_r8(r8, mmu),
            Instruction::DecR16(r16) => self.decrement_r16(r16),
            Instruction::DisableInterrupts => self.disable_interrupts(),
            Instruction::EnableInterrupts => self.enable_interrupts(),
            Instruction::Halt | Instruction::LoadR8R8((R8::HL, R8::HL)) => self.halt(),
            Instruction::IncR8(r8) => self.increment_r8(r8, mmu),
            Instruction::IncR16(r16) => self.increment_r16(r16),
            Instruction::JpHL => self.jump_hl(),
            Instruction::JpImm16 => self.jump_imm16(mmu),
            Instruction::JpCondImm16(condition) => self.jump_condition_imm16(condition, mmu),
            Instruction::JrImm8 => self.jump_relative_imm8(mmu),
            Instruction::JrCondImm8(cond) => self.jump_relative_condition_imm8(cond, mmu),
            Instruction::LoadAR16(r16_mem) => self.load_a_r16m(r16_mem, mmu),
            Instruction::LoadR16A(r16_mem) => self.load_r16m_a(r16_mem, mmu),
            Instruction::LoadR16Imm16(r16) => self.load_r16_imm(r16, mmu),
            Instruction::LoadR8Imm8(r8) => self.load_r8_imm8(r8, mmu),
            Instruction::LoadR8R8((target_r8, source_r8)) => {
                self.load_r8_r8(target_r8, source_r8, mmu)
            }
            Instruction::LoadImm16SP => self.load_imm16_sp(mmu),
            Instruction::Nop => self.instruction_result(1, 1),
            Instruction::PopR16(r16_stack) => self.pop_r16(r16_stack, mmu),
            Instruction::PushR16(r16_stack) => self.push_r16(r16_stack, mmu),
            Instruction::Return => self.return_from_func(mmu),
            Instruction::ReturnCondition(cond) => self.return_from_func_cond(cond, mmu),
            Instruction::ReturnEnableInterrupts => self.return_from_func_enable_interrupts(mmu),
            Instruction::RotateLeftA => self.rotate_left_a(),
            Instruction::RotateRightA => self.rotate_right_a(),
            Instruction::RotateLeftCarryA => self.rotate_left_carry_a(),
            Instruction::RotateRightCarryA => self.rotate_right_carry_a(),
            Instruction::SetCarryFlag => self.set_carry_flag(),
        }
    }

    pub fn step(&mut self, mmu: &mut MMU) -> u8 {
        // This helps checking if the deferred set of the ime was already scheduled before the current instruction
        let initial_deferred_set_ime = self.get_deferred_set_ime();

        let has_interrupt = self.ime && self.handle_interrupts(mmu);
        if has_interrupt {
            self.eeping = false;
            return 5; // The interrupt handling takes 5 m-cycles
        }

        if self.eeping && !self.ime && self.is_interrupt_pending(mmu) {
            self.eeping = false;
        } else if self.eeping {
            return 1; // Just stall a cycle
        }

        let mut instruction_byte = mmu.read(self.get_pc());
        let prefixed = instruction_byte == PREFIX_INSTRUCTION_BYTE;
        if prefixed {
            instruction_byte = mmu.read(self.get_pc().wrapping_add(1));
        }

        let instruction = Instruction::from_byte(instruction_byte, prefixed).unwrap();
        if self.should_trigger_halting_bug(&instruction, mmu) {
            self.set_pc(self.get_pc().wrapping_add(1));
            self.halting_bug_active = true;
            return self.step(mmu);
        }

        self.log_instruction_execute(&instruction, instruction_byte, mmu);

        let (next_pc, m_cycles) = self.execute(instruction, mmu);
        if self.halting_bug_active {
            self.halting_bug_active = false;
        } else {
            self.set_pc(next_pc);
        }

        if initial_deferred_set_ime && self.deferred_set_ime {
            self.deferred_set_ime = false;
            self.ime = true;
        }

        m_cycles
    }

    fn is_interrupt_pending(&self, mmu: &MMU) -> bool {
        mmu.get_interrupt().is_some()
    }

    /// https://gbdev.io/pandocs/halt.html#halt
    fn should_trigger_halting_bug(&self, instruction: &Instruction, mmu: &MMU) -> bool {
        !self.ime
            && self.is_interrupt_pending(mmu)
            && matches!(
                instruction,
                Instruction::Halt | Instruction::LoadR8R8((R8::HL, R8::HL))
            )
    }

    fn handle_interrupts(&mut self, mmu: &mut MMU) -> bool {
        let Some(interrupt) = mmu.get_interrupt() else {
            return false;
        };

        let new_i_flag = set_bit_u8(mmu.read(IF_ADDRESS), interrupt.get_if_index(), false);
        mmu.write(IF_ADDRESS, new_i_flag);
        self.ime = false;

        self.push_u16(self.get_pc(), mmu);
        self.set_pc(interrupt.get_target_address());

        true
    }

    pub fn set_registers(&mut self, registers: CPURegisters) {
        self.registers = registers;
    }

    pub fn get_ime(&self) -> bool {
        self.ime
    }

    pub fn get_deferred_set_ime(&self) -> bool {
        self.deferred_set_ime
    }
}

/// Direct instruction interfaces
impl CPU {
    pub fn add_r8(&mut self, r8: R8, mmu: &MMU) -> (u16, u8) {
        let source_value = self.get_r8(r8, mmu);
        let (new_value, half_carry, carry) = add_u8(self.get_a(), source_value);

        self.set_a(new_value);
        self.set_f_zero(new_value == 0);
        self.set_f_subtract(false);
        self.set_f_half_carry(half_carry);
        self.set_f_carry(carry);

        let m = if r8 == R8::HL { 2 } else { 1 };
        self.instruction_result(1, m)
    }

    pub fn add_carry_r8(&mut self, r8: R8, mmu: &MMU) -> (u16, u8) {
        let source_value = self.get_r8(r8, mmu);
        let (new_value, half_carry, carry) =
            add_carry_u8(self.get_a(), source_value, self.get_f_carry());

        self.set_a(new_value);
        self.set_f_zero(new_value == 0);
        self.set_f_subtract(false);
        self.set_f_half_carry(half_carry);
        self.set_f_carry(carry);

        let m = if r8 == R8::HL { 2 } else { 1 };
        self.instruction_result(1, m)
    }

    pub fn add_hl_r16(&mut self, r16: R16) -> (u16, u8) {
        let source_value = self.get_r16(r16);
        let (new_value, half_carry, carry) = add_u16(self.get_hl(), source_value);

        self.set_hl(new_value);
        self.set_f_subtract(false);
        self.set_f_half_carry(half_carry);
        self.set_f_carry(carry);

        self.instruction_result(1, 2)
    }

    pub fn complement_a(&mut self) -> (u16, u8) {
        self.set_a(!self.get_a());
        self.set_f_subtract(true);
        self.set_f_half_carry(true);
        self.instruction_result(1, 1)
    }

    pub fn complement_carry(&mut self) -> (u16, u8) {
        self.set_f_carry(!self.get_f_carry());
        self.set_f_subtract(false);
        self.set_f_half_carry(false);
        self.instruction_result(1, 1)
    }

    pub fn decimal_adjust_accumulator(&mut self) -> (u16, u8) {
        let current_a = self.get_a();
        let mut new_carry = self.get_f_carry();
        let mut adjustment: u8 = 0;

        //let mut new_carry = false;
        let new_a = if self.get_f_subtract() {
            if self.get_f_half_carry() {
                adjustment += 0x06;
            }
            if self.get_f_carry() {
                adjustment += 0x60;
            }
            current_a.wrapping_sub(adjustment)
        } else {
            if self.get_f_half_carry() || current_a & 0xF > 0x9 {
                adjustment += 0x06;
            }
            if self.get_f_carry() || current_a > 0x99 {
                adjustment += 0x60;
                new_carry = true;
            }
            current_a.wrapping_add(adjustment)
        };

        self.set_a(new_a);
        self.set_f_carry(new_carry);
        self.set_f_half_carry(false);
        self.set_f_zero(new_a == 0);

        self.instruction_result(1, 1)
    }

    pub fn decrement_r8(&mut self, r8: R8, mmu: &mut MMU) -> (u16, u8) {
        self.set_r8(r8, self.get_r8(r8, mmu).wrapping_sub(1), mmu);
        let m = if r8 == R8::HL { 3 } else { 1 };
        self.instruction_result(1, m)
    }

    pub fn decrement_r16(&mut self, r16: R16) -> (u16, u8) {
        self.set_r16(r16, self.get_r16(r16).wrapping_sub(1));
        self.instruction_result(1, 2)
    }

    pub fn disable_interrupts(&mut self) -> (u16, u8) {
        self.ime = false;
        self.deferred_set_ime = false;
        self.instruction_result(1, 1)
    }

    pub fn enable_interrupts(&mut self) -> (u16, u8) {
        self.deferred_set_ime = true;
        self.instruction_result(1, 1)
    }

    pub fn halt(&mut self) -> (u16, u8) {
        self.eeping = true;
        self.instruction_result(1, 1)
    }

    pub fn increment_r8(&mut self, r8: R8, mmu: &mut MMU) -> (u16, u8) {
        self.set_r8(r8, self.get_r8(r8, mmu).wrapping_add(1), mmu);
        let m = if r8 == R8::HL { 3 } else { 1 };
        self.instruction_result(1, m)
    }

    pub fn increment_r16(&mut self, r16: R16) -> (u16, u8) {
        self.set_r16(r16, self.get_r16(r16).wrapping_add(1));
        self.instruction_result(1, 2)
    }

    pub fn load_r16_imm(&mut self, r16: R16, mmu: &MMU) -> (u16, u8) {
        let value = self.read_next_imm16(mmu);
        self.set_r16(r16, value);
        self.instruction_result(3, 3)
    }

    pub fn load_a_r16m(&mut self, r16_m: R16Mem, mmu: &mut MMU) -> (u16, u8) {
        let address = self.get_r16_mem(r16_m);
        let value = mmu.read(address);
        self.set_a(value);

        self.process_r16m_register_update(r16_m);
        self.instruction_result(1, 2)
    }

    pub fn load_r16m_a(&mut self, r16_m: R16Mem, mmu: &mut MMU) -> (u16, u8) {
        let address = self.get_r16_mem(r16_m);
        let value = self.get_a();
        mmu.write(address, value);

        self.process_r16m_register_update(r16_m);
        self.instruction_result(1, 2)
    }

    pub fn load_r8_imm8(&mut self, r8: R8, mmu: &mut MMU) -> (u16, u8) {
        let value = self.read_next_imm8(mmu);
        self.set_r8(r8, value, mmu);

        let m = if r8 == R8::HL { 3 } else { 2 };
        self.instruction_result(2, m)
    }

    pub fn load_r8_r8(&mut self, target_r8: R8, source_r8: R8, mmu: &mut MMU) -> (u16, u8) {
        if target_r8 == R8::HL && source_r8 == R8::HL {
            return self.halt();
        }

        let value = self.get_r8(source_r8, mmu);
        self.set_r8(target_r8, value, mmu);

        let m = if target_r8 == R8::HL || source_r8 == R8::HL {
            2
        } else {
            1
        };
        self.instruction_result(1, m)
    }

    pub fn load_imm16_sp(&mut self, mmu: &mut MMU) -> (u16, u8) {
        let address = self.read_next_imm16(mmu);
        let value_lsb = mmu.read(self.get_sp());
        let value_msb = mmu.read(self.get_sp().wrapping_add(1));
        mmu.write(address, value_lsb);
        mmu.write(address.wrapping_add(1), value_msb);

        self.instruction_result(3, 5)
    }

    pub fn pop_r16(&mut self, r16_stack: R16Stack, mmu: &MMU) -> (u16, u8) {
        let value = self.pop_u16(mmu);
        self.set_r16_stack(r16_stack, value);
        self.instruction_result(1, 3)
    }

    pub fn push_r16(&mut self, r16_stack: R16Stack, mmu: &mut MMU) -> (u16, u8) {
        let value = self.get_r16_stack(r16_stack);
        self.push_u16(value, mmu);
        self.instruction_result(1, 4)
    }

    pub fn jump_hl(&mut self) -> (u16, u8) {
        let new_pc = self.get_hl();
        (new_pc, 1)
    }

    pub fn jump_imm16(&self, mmu: &MMU) -> (u16, u8) {
        let new_pc = self.read_next_imm16(mmu);
        (new_pc, 4)
    }

    pub fn jump_condition_imm16(&self, condition: JumpCondition, mmu: &MMU) -> (u16, u8) {
        let should_jump = self.check_jump_condition(condition);

        if should_jump {
            self.jump_imm16(mmu)
        } else {
            self.instruction_result(3, 3)
        }
    }

    pub fn jump_relative_imm8(&self, mmu: &MMU) -> (u16, u8) {
        let value = self.read_next_imm8(mmu) as u16;
        self.instruction_result(value + 2, 3)
    }

    pub fn jump_relative_condition_imm8(&self, condition: JumpCondition, mmu: &MMU) -> (u16, u8) {
        let should_jump = self.check_jump_condition(condition);

        if should_jump {
            self.jump_relative_imm8(mmu)
        } else {
            self.instruction_result(2, 2)
        }
    }

    pub fn return_from_func(&mut self, mmu: &MMU) -> (u16, u8) {
        let return_to_pc = self.pop_u16(mmu);
        self.instruction_result(return_to_pc, 4)
    }

    pub fn return_from_func_cond(&mut self, condition: JumpCondition, mmu: &MMU) -> (u16, u8) {
        let should_jump = self.check_jump_condition(condition);
        if should_jump {
            let (new_pc, _) = self.return_from_func(mmu);
            self.instruction_result(new_pc, 5)
        } else {
            self.instruction_result(1, 2)
        }
    }

    pub fn return_from_func_enable_interrupts(&mut self, mmu: &MMU) -> (u16, u8) {
        self.ime = true;
        self.return_from_func(mmu)
    }

    pub fn rotate_left_a(&mut self) -> (u16, u8) {
        let (new_a, new_carry) = rotate_left_through_carry_u8(self.get_a(), self.get_f_carry());
        self.update_a_and_flags_after_rotation(new_a, new_carry);
        self.instruction_result(1, 1)
    }

    pub fn rotate_right_a(&mut self) -> (u16, u8) {
        let (new_a, new_carry) = rotate_right_through_carry_u8(self.get_a(), self.get_f_carry());
        self.update_a_and_flags_after_rotation(new_a, new_carry);
        self.instruction_result(1, 1)
    }

    pub fn rotate_left_carry_a(&mut self) -> (u16, u8) {
        let (new_a, new_carry) = rotate_left_get_carry_u8(self.get_a());
        self.update_a_and_flags_after_rotation(new_a, new_carry);
        self.instruction_result(1, 1)
    }

    pub fn rotate_right_carry_a(&mut self) -> (u16, u8) {
        let (new_a, new_carry) = rotate_right_get_carry_u8(self.get_a());
        self.update_a_and_flags_after_rotation(new_a, new_carry);
        self.instruction_result(1, 1)
    }

    pub fn set_carry_flag(&mut self) -> (u16, u8) {
        self.set_f_carry(true);
        self.set_f_subtract(false);
        self.set_f_half_carry(false);
        self.instruction_result(1, 1)
    }
}

/// Basic operations
impl CPU {
    pub fn pop_u8(&mut self, mmu: &MMU) -> u8 {
        let value = mmu.read(self.get_sp());
        self.increment_sp();
        value
    }

    pub fn pop_u16(&mut self, mmu: &MMU) -> u16 {
        let lsb = self.pop_u8(mmu);
        let msb = self.pop_u8(mmu);
        construct_u16(lsb, msb)
    }

    pub fn push_u8(&mut self, value: u8, mmu: &mut MMU) {
        self.decrement_sp();
        mmu.write(self.get_sp(), value);
    }

    pub fn push_u16(&mut self, value: u16, mmu: &mut MMU) {
        let (lsb, msb) = deconstruct_u16(value);
        self.push_u8(msb, mmu);
        self.push_u8(lsb, mmu);
    }
}

/// Helper functions
impl CPU {
    fn instruction_result(&self, pc_raise: u16, m_cycles: u8) -> (u16, u8) {
        (self.get_pc().wrapping_add(pc_raise), m_cycles)
    }

    fn read_next_imm8(&self, mmu: &MMU) -> u8 {
        mmu.read(self.get_pc().wrapping_add(1))
    }

    fn read_next_imm16(&self, mmu: &MMU) -> u16 {
        mmu.read_16(self.get_pc().wrapping_add(1))
    }

    fn process_r16m_register_update(&mut self, r16_m: R16Mem) {
        if r16_m == R16Mem::HLI {
            self.set_hl(self.get_hl().wrapping_add(1));
        } else if r16_m == R16Mem::HLD {
            self.set_hl(self.get_hl().wrapping_sub(1));
        }
    }

    fn update_a_and_flags_after_rotation(&mut self, a: u8, new_carry: bool) {
        self.set_a(a);
        self.set_f_carry(new_carry);
        self.set_f_zero(false);
        self.set_f_subtract(false);
        self.set_f_half_carry(false);
    }
}

/// Logging
impl CPU {
    fn log_instruction_execute(&self, instruction: &Instruction, instruction_byte: u8, mmu: &MMU) {
        if log::log_enabled!(log::Level::Info) {
            let (next_lsb, next_msb) = deconstruct_u16(self.read_next_imm16(mmu));
            info!(
                "PC(0x{:04X}) [0x{:02X}]: {}",
                self.get_pc(),
                instruction_byte,
                instruction.parse_description(next_lsb, next_msb)
            );
        }
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
