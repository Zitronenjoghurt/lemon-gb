use crate::hardware::cpu::memory_bus::MemoryBus;
use crate::hardware::cpu::Cpu;
use crate::hardware::registers::Registers;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct CpuBuilder {
    cpu: Cpu,
}

impl CpuBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Cpu {
        self.cpu
    }

    pub fn registers(mut self, registers: Registers) -> Self {
        self.cpu.set_registers(registers);
        self
    }

    pub fn pc(mut self, pc: u16) -> Self {
        self.cpu.set_pc(pc);
        self
    }

    pub fn memory(mut self, memory: MemoryBus) -> Self {
        self.cpu.set_memory(memory);
        self
    }
}
