use crate::game_boy::components::cpu::registers::builder::CPURegistersBuilderTrait;
use crate::game_boy::components::cpu::registers::{CPURegisters, CpuRegistersAccessTrait};
use crate::game_boy::components::cpu::CPU;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct CpuBuilder {
    cpu: CPU,
    registers: CPURegisters,
}

impl CpuBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(mut self) -> CPU {
        self.cpu.set_registers(self.registers.clone());
        self.cpu
    }
}

impl CpuRegistersAccessTrait for CpuBuilder {
    fn get_registers(&self) -> &CPURegisters {
        &self.registers
    }

    fn get_registers_mut(&mut self) -> &mut CPURegisters {
        &mut self.registers
    }
}

impl CPURegistersBuilderTrait for CpuBuilder {}
