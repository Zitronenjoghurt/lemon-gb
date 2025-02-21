use crate::game_boy::components::cartridge::Cartridge;
use crate::game_boy::components::cpu::CPU;
use crate::game_boy::components::mmu::MMU;
use crate::game_boy::components::timer::Timer;

pub mod components;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct GameBoy {
    /// Central Processing Unit
    cpu: CPU,
    /// Memory Management Unit
    /// Handles all memory storage and access
    mmu: MMU,
    timer: Timer,
}

impl GameBoy {
    pub fn initialize(cartridge: &Cartridge) -> Self {
        Self {
            cpu: CPU::initialize(),
            mmu: MMU::initialize(cartridge),
            timer: Timer::initialize(),
        }
    }

    pub fn step(&mut self) -> u8 {
        let m = self.cpu.step(&mut self.mmu);
        self.timer.step(m, &mut self.mmu);
        m
    }
}
