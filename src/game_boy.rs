use crate::game_boy::components::cpu::CPU;
use crate::game_boy::components::mmu::MMU;

pub mod components;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct GameBoy {
    cpu: CPU,
    mmu: MMU,
}
