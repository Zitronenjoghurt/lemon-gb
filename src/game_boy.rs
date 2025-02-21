use crate::game_boy::components::cartridge::Cartridge;
use crate::game_boy::components::cpu::CPU;
use crate::game_boy::components::mmu::MMU;

pub mod components;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct GameBoy {
    /// Central Processing Unit
    cpu: CPU,
    /// Memory Management Unit
    /// Handles all memory storage and access
    mmu: MMU,
}

impl GameBoy {
    pub fn initialize(cartridge: &Cartridge) -> Self {
        Self {
            cpu: CPU::initialize(),
            mmu: MMU::initialize(cartridge),
        }
    }
}
