use crate::enums::parameter_groups::R16Stack;
use crate::enums::parameter_groups::{JumpCondition, R16Mem, R16, R8};
use crate::game_boy::components::cpu::builder::CpuBuilder;
use crate::game_boy::components::cpu::registers::CpuRegistersAccessTrait;
use crate::game_boy::components::mmu::{IF_ADDRESS, MMU};
use crate::helpers::bit_operations::*;
use crate::instructions::Instruction;
use log::debug;
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
            Instruction::AddImm8 => self.add_imm8(mmu),
            Instruction::AddCarryR8(r8) => self.add_carry_r8(r8, mmu),
            Instruction::AddCarryImm8 => self.add_carry_imm8(mmu),
            Instruction::AddSpImm8 => self.add_sp_imm8(mmu),
            Instruction::AndR8(r8) => self.and_r8(r8, mmu),
            Instruction::AndImm8 => self.and_imm8(mmu),
            Instruction::Call => self.call(mmu),
            Instruction::CallCondition(cond) => self.call_conditional(cond, mmu),
            Instruction::CompareR8(r8) => self.compare_r8(r8, mmu),
            Instruction::CompareImm8 => self.compare_imm8(mmu),
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
            Instruction::LoadHighAC => self.load_high_a_c(mmu),
            Instruction::LoadHighCA => self.load_high_c_a(mmu),
            Instruction::LoadHighAImm8 => self.load_high_a_imm8(mmu),
            Instruction::LoadHighImm8A => self.load_high_imm8_a(mmu),
            Instruction::LoadAImm16 => self.load_a_imm16(mmu),
            Instruction::LoadImm16A => self.load_imm16_a(mmu),
            Instruction::LoadImm16SP => self.load_imm16_sp(mmu),
            Instruction::LoadHlSpImm8 => self.load_hl_sp_imm8(mmu),
            Instruction::LoadSpHl => self.load_sp_hl(),
            Instruction::Nop => self.instruction_result(1, 1),
            Instruction::OrR8(r8) => self.or_r8(r8, mmu),
            Instruction::OrImm8 => self.or_imm8(mmu),
            Instruction::PopR16(r16_stack) => self.pop_r16(r16_stack, mmu),
            Instruction::PushR16(r16_stack) => self.push_r16(r16_stack, mmu),
            Instruction::RestartVector(address) => self.restart_vector(address, mmu),
            Instruction::Return => self.return_from_func(mmu),
            Instruction::ReturnCondition(cond) => self.return_from_func_cond(cond, mmu),
            Instruction::ReturnEnableInterrupts => self.return_from_func_enable_interrupts(mmu),
            Instruction::RotateLeftA => self.rotate_left_a(),
            Instruction::RotateRightA => self.rotate_right_a(),
            Instruction::RotateLeftCircularA => self.rotate_left_circular_a(),
            Instruction::RotateRightCircularA => self.rotate_right_circular_a(),
            Instruction::SetCarryFlag => self.set_carry_flag(),
            Instruction::SubR8(r8) => self.sub_r8(r8, mmu),
            Instruction::SubImm8 => self.sub_imm8(mmu),
            Instruction::SubCarryR8(r8) => self.sub_carry_r8(r8, mmu),
            Instruction::SubCarryImm8 => self.sub_carry_imm8(mmu),
            Instruction::XorR8(r8) => self.xor_r8(r8, mmu),
            Instruction::XorImm8 => self.xor_imm8(mmu),
            Instruction::BitCheckR8((index, r8)) => self.bit_check_r8(index, r8, mmu),
            Instruction::BitResetR8((index, r8)) => self.bit_reset_r8(index, r8, mmu),
            Instruction::BitSetR8((index, r8)) => self.bit_set_r8(index, r8, mmu),
            Instruction::RotateLeftR8(r8) => self.rotate_left_r8(r8, mmu),
            Instruction::RotateLeftCircularR8(r8) => self.rotate_left_circular_r8(r8, mmu),
            Instruction::RotateRightR8(r8) => self.rotate_right_r8(r8, mmu),
            Instruction::RotateRightCircularR8(r8) => self.rotate_right_circular_r8(r8, mmu),
            Instruction::ShiftLeftR8(r8) => self.shift_left_r8(r8, mmu),
            Instruction::ShiftRightR8(r8) => self.shift_right_arithmetical_r8(r8, mmu),
            Instruction::ShiftRightLogicallyR8(r8) => self.shift_right_logical_r8(r8, mmu),
            Instruction::SwapR8(r8) => self.swap_r8(r8, mmu),
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

    pub fn add_imm8(&mut self, mmu: &MMU) -> (u16, u8) {
        let source_value = self.read_next_imm8(mmu);
        let (new_value, half_carry, carry) = add_u8(self.get_a(), source_value);

        self.set_a(new_value);
        self.set_f_zero(new_value == 0);
        self.set_f_subtract(false);
        self.set_f_half_carry(half_carry);
        self.set_f_carry(carry);

        self.instruction_result(2, 2)
    }

    pub fn add_carry_imm8(&mut self, mmu: &MMU) -> (u16, u8) {
        let source_value = self.read_next_imm8(mmu);
        let (new_value, half_carry, carry) =
            add_carry_u8(self.get_a(), source_value, self.get_f_carry());

        self.set_a(new_value);
        self.set_f_zero(new_value == 0);
        self.set_f_subtract(false);
        self.set_f_half_carry(half_carry);
        self.set_f_carry(carry);

        self.instruction_result(2, 2)
    }

    pub fn and_r8(&mut self, r8: R8, mmu: &MMU) -> (u16, u8) {
        let source_value = self.get_r8(r8, mmu);
        let new_value = self.get_a() & source_value;

        self.set_a(new_value);
        self.set_f_zero(new_value == 0);
        self.set_f_subtract(false);
        self.set_f_half_carry(true);
        self.set_f_carry(false);

        let m = if r8 == R8::HL { 2 } else { 1 };
        self.instruction_result(1, m)
    }

    pub fn and_imm8(&mut self, mmu: &MMU) -> (u16, u8) {
        let source_value = self.read_next_imm8(mmu);
        let new_value = self.get_a() & source_value;

        self.set_a(new_value);
        self.set_f_zero(new_value == 0);
        self.set_f_subtract(false);
        self.set_f_half_carry(true);
        self.set_f_carry(false);

        self.instruction_result(2, 2)
    }

    pub fn add_sp_imm8(&mut self, mmu: &MMU) -> (u16, u8) {
        let value = self.read_next_imm8_signed(mmu);
        let (result, half_carry, carry) = add_u16_i8(self.get_sp(), value);
        self.set_sp(result);
        self.set_f_zero(false);
        self.set_f_subtract(false);
        self.set_f_half_carry(half_carry);
        self.set_f_carry(carry);

        self.instruction_result(2, 4)
    }

    pub fn bit_check_r8(&mut self, bit_index: usize, register: R8, mmu: &mut MMU) -> (u16, u8) {
        let value = self.get_r8(register, mmu);
        self.set_f_zero(!get_bit_u8(value, bit_index));
        self.set_f_subtract(false);
        self.set_f_half_carry(true);

        let m = if register == R8::HL { 3 } else { 2 };
        self.instruction_result(2, m)
    }

    pub fn bit_reset_r8(&mut self, bit_index: usize, register: R8, mmu: &mut MMU) -> (u16, u8) {
        let value = self.get_r8(register, mmu);
        let new_value = set_bit_u8(value, bit_index, false);
        self.set_r8(register, new_value, mmu);
        let m = if register == R8::HL { 4 } else { 2 };
        self.instruction_result(2, m)
    }

    pub fn bit_set_r8(&mut self, bit_index: usize, register: R8, mmu: &mut MMU) -> (u16, u8) {
        let value = self.get_r8(register, mmu);
        let new_value = set_bit_u8(value, bit_index, true);
        self.set_r8(register, new_value, mmu);
        let m = if register == R8::HL { 4 } else { 2 };
        self.instruction_result(2, m)
    }

    pub fn call(&mut self, mmu: &mut MMU) -> (u16, u8) {
        let func_address = self.read_next_imm16(mmu);
        self.push_u16(self.get_pc().wrapping_add(3), mmu);
        (func_address, 6)
    }

    pub fn call_conditional(&mut self, jump_condition: JumpCondition, mmu: &mut MMU) -> (u16, u8) {
        let should_jump = self.check_jump_condition(jump_condition);

        if should_jump {
            self.call(mmu)
        } else {
            self.instruction_result(3, 3)
        }
    }

    pub fn compare_r8(&mut self, r8: R8, mmu: &MMU) -> (u16, u8) {
        let source_value = self.get_r8(r8, mmu);
        let (ignored_result, half_carry, carry) = sub_u8(self.get_a(), source_value);

        self.set_f_zero(ignored_result == 0);
        self.set_f_subtract(true);
        self.set_f_half_carry(half_carry);
        self.set_f_carry(carry);

        let m = if r8 == R8::HL { 2 } else { 1 };
        self.instruction_result(1, m)
    }

    pub fn compare_imm8(&mut self, mmu: &MMU) -> (u16, u8) {
        let source_value = self.read_next_imm8(mmu);
        let (ignored_result, half_carry, carry) = sub_u8(self.get_a(), source_value);

        self.set_f_zero(ignored_result == 0);
        self.set_f_subtract(true);
        self.set_f_half_carry(half_carry);
        self.set_f_carry(carry);

        self.instruction_result(2, 2)
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
        let value = self.get_r8(r8, mmu);
        let (new_value, half_carry, _) = sub_u8(value, 1);

        self.set_r8(r8, new_value, mmu);
        self.set_f_zero(new_value == 0);
        self.set_f_subtract(true);
        self.set_f_half_carry(half_carry);

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
        let value = self.get_r8(r8, mmu);
        let (new_value, half_carry, _) = add_u8(value, 1);

        self.set_r8(r8, new_value, mmu);
        self.set_f_zero(new_value == 0);
        self.set_f_subtract(false);
        self.set_f_half_carry(half_carry);

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

    pub fn load_high_a_c(&mut self, mmu: &MMU) -> (u16, u8) {
        let address = construct_u16(self.get_c(), 0xFF);
        self.set_a(mmu.read(address));
        self.instruction_result(1, 2)
    }

    pub fn load_high_c_a(&mut self, mmu: &mut MMU) -> (u16, u8) {
        let address = construct_u16(self.get_c(), 0xFF);
        mmu.write(address, self.get_a());
        self.instruction_result(1, 2)
    }

    pub fn load_high_a_imm8(&mut self, mmu: &MMU) -> (u16, u8) {
        let lsb = self.read_next_imm8(mmu);
        let address = construct_u16(lsb, 0xFF);
        self.set_a(mmu.read(address));
        self.instruction_result(2, 3)
    }

    pub fn load_high_imm8_a(&mut self, mmu: &mut MMU) -> (u16, u8) {
        let lsb = self.read_next_imm8(mmu);
        let address = construct_u16(lsb, 0xFF);
        mmu.write(address, self.get_a());
        self.instruction_result(2, 3)
    }

    pub fn load_a_imm16(&mut self, mmu: &MMU) -> (u16, u8) {
        let address = self.read_next_imm16(mmu);
        self.set_a(mmu.read(address));
        self.instruction_result(3, 4)
    }

    pub fn load_imm16_a(&mut self, mmu: &mut MMU) -> (u16, u8) {
        let address = self.read_next_imm16(mmu);
        mmu.write(address, self.get_a());
        self.instruction_result(3, 4)
    }

    pub fn load_imm16_sp(&mut self, mmu: &mut MMU) -> (u16, u8) {
        let address = self.read_next_imm16(mmu);
        let (sp_lsb, sp_msb) = deconstruct_u16(self.get_sp());
        mmu.write(address, sp_lsb);
        mmu.write(address.wrapping_add(1), sp_msb);

        self.instruction_result(3, 5)
    }

    pub fn load_hl_sp_imm8(&mut self, mmu: &MMU) -> (u16, u8) {
        let value = self.read_next_imm8_signed(mmu);
        let (new_hl, half_carry, carry) = add_u16_i8(self.get_sp(), value);

        self.set_hl(new_hl);
        self.set_f_zero(false);
        self.set_f_subtract(false);
        self.set_f_half_carry(half_carry);
        self.set_f_carry(carry);

        self.instruction_result(2, 3)
    }

    pub fn load_sp_hl(&mut self) -> (u16, u8) {
        self.set_sp(self.get_hl());
        self.instruction_result(1, 2)
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
        let value = self.read_next_imm8_signed(mmu);
        let (new_pc, _, _) = add_u16_i8(self.get_pc(), value);
        let new_pc = new_pc.wrapping_add(2); // The pc increments that occurred due to this instruction
        (new_pc, 3)
    }

    pub fn jump_relative_condition_imm8(&self, condition: JumpCondition, mmu: &MMU) -> (u16, u8) {
        let should_jump = self.check_jump_condition(condition);

        if should_jump {
            self.jump_relative_imm8(mmu)
        } else {
            self.instruction_result(2, 2)
        }
    }

    pub fn or_r8(&mut self, r8: R8, mmu: &mut MMU) -> (u16, u8) {
        let source_value = self.get_r8(r8, mmu);
        let new_value = self.get_a() | source_value;

        self.set_a(new_value);
        self.set_f_zero(new_value == 0);
        self.set_f_subtract(false);
        self.set_f_half_carry(false);
        self.set_f_carry(false);

        let m = if r8 == R8::HL { 2 } else { 1 };
        self.instruction_result(1, m)
    }

    pub fn or_imm8(&mut self, mmu: &MMU) -> (u16, u8) {
        let source_value = self.read_next_imm8(mmu);
        let new_value = self.get_a() | source_value;

        self.set_a(new_value);
        self.set_f_zero(new_value == 0);
        self.set_f_subtract(false);
        self.set_f_half_carry(false);
        self.set_f_carry(false);

        self.instruction_result(2, 2)
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

    pub fn restart_vector(&mut self, address_lsb: u8, mmu: &mut MMU) -> (u16, u8) {
        let address = construct_u16(address_lsb, 0x00);
        self.push_u16(self.get_pc().wrapping_add(1), mmu);
        (address, 4)
    }

    pub fn return_from_func(&mut self, mmu: &MMU) -> (u16, u8) {
        let return_to_pc = self.pop_u16(mmu);
        (return_to_pc, 4)
    }

    pub fn return_from_func_cond(&mut self, condition: JumpCondition, mmu: &MMU) -> (u16, u8) {
        let should_jump = self.check_jump_condition(condition);
        if should_jump {
            let (new_pc, _) = self.return_from_func(mmu);
            (new_pc, 5)
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

    pub fn rotate_left_r8(&mut self, register: R8, mmu: &mut MMU) -> (u16, u8) {
        let value = self.get_r8(register, mmu);
        let (new_value, new_carry) = rotate_left_through_carry_u8(value, self.get_f_carry());

        self.set_r8(register, new_value, mmu);
        self.set_f_zero(new_value == 0);
        self.set_f_subtract(false);
        self.set_f_half_carry(false);
        self.set_f_carry(new_carry);

        let m = if register == R8::HL { 4 } else { 2 };
        self.instruction_result(2, m)
    }

    pub fn rotate_right_a(&mut self) -> (u16, u8) {
        let (new_a, new_carry) = rotate_right_through_carry_u8(self.get_a(), self.get_f_carry());
        self.update_a_and_flags_after_rotation(new_a, new_carry);
        self.instruction_result(1, 1)
    }

    pub fn rotate_right_r8(&mut self, register: R8, mmu: &mut MMU) -> (u16, u8) {
        let value = self.get_r8(register, mmu);
        let (new_value, new_carry) = rotate_right_through_carry_u8(value, self.get_f_carry());

        self.set_r8(register, new_value, mmu);
        self.set_f_zero(new_value == 0);
        self.set_f_subtract(false);
        self.set_f_half_carry(false);
        self.set_f_carry(new_carry);

        let m = if register == R8::HL { 4 } else { 2 };
        self.instruction_result(2, m)
    }

    pub fn rotate_left_circular_a(&mut self) -> (u16, u8) {
        let (new_a, new_carry) = rotate_left_get_carry_u8(self.get_a());
        self.update_a_and_flags_after_rotation(new_a, new_carry);
        self.instruction_result(1, 1)
    }

    pub fn rotate_left_circular_r8(&mut self, register: R8, mmu: &mut MMU) -> (u16, u8) {
        let value = self.get_r8(register, mmu);
        let (new_value, new_carry) = rotate_left_get_carry_u8(value);

        self.set_r8(register, new_value, mmu);
        self.set_f_zero(new_value == 0);
        self.set_f_subtract(false);
        self.set_f_half_carry(false);
        self.set_f_carry(new_carry);

        let m = if register == R8::HL { 4 } else { 2 };
        self.instruction_result(2, m)
    }

    pub fn rotate_right_circular_a(&mut self) -> (u16, u8) {
        let (new_a, new_carry) = rotate_right_get_carry_u8(self.get_a());
        self.update_a_and_flags_after_rotation(new_a, new_carry);
        self.instruction_result(1, 1)
    }

    pub fn rotate_right_circular_r8(&mut self, register: R8, mmu: &mut MMU) -> (u16, u8) {
        let value = self.get_r8(register, mmu);
        let (new_value, new_carry) = rotate_right_get_carry_u8(value);

        self.set_r8(register, new_value, mmu);
        self.set_f_zero(new_value == 0);
        self.set_f_subtract(false);
        self.set_f_half_carry(false);
        self.set_f_carry(new_carry);

        let m = if register == R8::HL { 4 } else { 2 };
        self.instruction_result(2, m)
    }

    pub fn set_carry_flag(&mut self) -> (u16, u8) {
        self.set_f_carry(true);
        self.set_f_subtract(false);
        self.set_f_half_carry(false);
        self.instruction_result(1, 1)
    }

    pub fn shift_left_r8(&mut self, register: R8, mmu: &mut MMU) -> (u16, u8) {
        let value = self.get_r8(register, mmu);
        let new_carry = get_bit_u8(value, 7);
        let new_value = value << 1;

        self.set_r8(register, new_value, mmu);
        self.set_f_zero(new_value == 0);
        self.set_f_subtract(false);
        self.set_f_half_carry(false);
        self.set_f_carry(new_carry);

        let m = if register == R8::HL { 4 } else { 2 };
        self.instruction_result(2, m)
    }

    pub fn shift_right_arithmetical_r8(&mut self, register: R8, mmu: &mut MMU) -> (u16, u8) {
        let value = self.get_r8(register, mmu);
        let new_carry = get_bit_u8(value, 0);
        // Shift right while persisting the leftmost bit, this is important for signed values
        let new_value = set_bit_u8(value >> 1, 7, get_bit_u8(value, 7));

        self.set_r8(register, new_value, mmu);
        self.set_f_zero(new_value == 0);
        self.set_f_subtract(false);
        self.set_f_half_carry(false);
        self.set_f_carry(new_carry);

        let m = if register == R8::HL { 4 } else { 2 };
        self.instruction_result(2, m)
    }

    pub fn shift_right_logical_r8(&mut self, register: R8, mmu: &mut MMU) -> (u16, u8) {
        let value = self.get_r8(register, mmu);
        let new_carry = get_bit_u8(value, 0);
        let new_value = value >> 1; // Shift right while filling up with 0's

        self.set_r8(register, new_value, mmu);
        self.set_f_zero(new_value == 0);
        self.set_f_subtract(false);
        self.set_f_half_carry(false);
        self.set_f_carry(new_carry);

        let m = if register == R8::HL { 4 } else { 2 };
        self.instruction_result(2, m)
    }

    pub fn sub_r8(&mut self, r8: R8, mmu: &mut MMU) -> (u16, u8) {
        let source_value = self.get_r8(r8, mmu);
        let (new_value, half_carry, carry) = sub_u8(self.get_a(), source_value);

        self.set_a(new_value);
        self.set_f_zero(new_value == 0);
        self.set_f_subtract(true);
        self.set_f_half_carry(half_carry);
        self.set_f_carry(carry);

        let m = if r8 == R8::HL { 2 } else { 1 };
        self.instruction_result(1, m)
    }

    pub fn sub_imm8(&mut self, mmu: &MMU) -> (u16, u8) {
        let source_value = self.read_next_imm8(mmu);
        let (new_value, half_carry, carry) = sub_u8(self.get_a(), source_value);

        self.set_a(new_value);
        self.set_f_zero(new_value == 0);
        self.set_f_subtract(true);
        self.set_f_half_carry(half_carry);
        self.set_f_carry(carry);

        self.instruction_result(2, 2)
    }

    pub fn sub_carry_r8(&mut self, r8: R8, mmu: &mut MMU) -> (u16, u8) {
        let source_value = self.get_r8(r8, mmu);
        let (new_value, half_carry, carry) =
            sub_carry_u8(self.get_a(), source_value, self.get_f_carry());

        self.set_a(new_value);
        self.set_f_zero(new_value == 0);
        self.set_f_subtract(true);
        self.set_f_half_carry(half_carry);
        self.set_f_carry(carry);

        let m = if r8 == R8::HL { 2 } else { 1 };
        self.instruction_result(1, m)
    }

    pub fn sub_carry_imm8(&mut self, mmu: &MMU) -> (u16, u8) {
        let source_value = self.read_next_imm8(mmu);
        let (new_value, half_carry, carry) =
            sub_carry_u8(self.get_a(), source_value, self.get_f_carry());

        self.set_a(new_value);
        self.set_f_zero(new_value == 0);
        self.set_f_subtract(true);
        self.set_f_half_carry(half_carry);
        self.set_f_carry(carry);

        self.instruction_result(2, 2)
    }

    pub fn swap_r8(&mut self, register: R8, mmu: &mut MMU) -> (u16, u8) {
        let value = self.get_r8(register, mmu);
        let new_value = (value >> 4) | (value << 4);

        self.set_r8(register, new_value, mmu);
        self.set_f_zero(new_value == 0);
        self.set_f_subtract(false);
        self.set_f_half_carry(false);
        self.set_f_carry(false);

        let m = if register == R8::HL { 4 } else { 2 };
        self.instruction_result(2, m)
    }

    pub fn xor_r8(&mut self, r8: R8, mmu: &mut MMU) -> (u16, u8) {
        let source_value = self.get_r8(r8, mmu);
        let new_value = self.get_a() ^ source_value;

        self.set_a(new_value);
        self.set_f_zero(new_value == 0);
        self.set_f_subtract(false);
        self.set_f_half_carry(false);
        self.set_f_carry(false);

        let m = if r8 == R8::HL { 2 } else { 1 };
        self.instruction_result(1, m)
    }

    pub fn xor_imm8(&mut self, mmu: &MMU) -> (u16, u8) {
        let source_value = self.read_next_imm8(mmu);
        let new_value = self.get_a() ^ source_value;

        self.set_a(new_value);
        self.set_f_zero(new_value == 0);
        self.set_f_subtract(false);
        self.set_f_half_carry(false);
        self.set_f_carry(false);

        self.instruction_result(2, 2)
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

    fn read_next_imm8_signed(&self, mmu: &MMU) -> i8 {
        mmu.read(self.get_pc().wrapping_add(1)) as i8
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
            debug!(
                "PC(0x{:04X}) [0x{:02X}]: {}",
                self.get_pc(),
                instruction_byte,
                instruction.parse_clear_text(next_lsb, next_msb)
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
