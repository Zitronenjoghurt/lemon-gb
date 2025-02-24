use crate::game_boy::components::mmu::mbc::Mbc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MMUSaveState {
    pub mbc: Mbc,
    pub ram: Vec<Vec<u8>>,
    pub vram: Vec<u8>,
    pub wram: Vec<u8>,
    pub oam: Vec<u8>,
    pub io_registers: Vec<u8>,
    pub hram: Vec<u8>,
    pub ie_register: u8,
}
