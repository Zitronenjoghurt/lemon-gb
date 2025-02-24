use crate::game_boy::components::cartridge::types::MbcType;
use crate::game_boy::components::mmu::mbc::mbc1::Mbc1;
use serde::{Deserialize, Serialize};

pub mod mbc1;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Mbc {
    None,
    Mbc1(Mbc1),
}

impl Mbc {
    pub fn initialize(mbc_type: MbcType) -> Mbc {
        match mbc_type {
            MbcType::None => Mbc::None,
            MbcType::MBC1 => Mbc::Mbc1(Mbc1::initialize(false)),
            _ => panic!("Unsupported MBC type!"),
        }
    }

    pub fn handle_write(&mut self, address: u16, value: u8) {
        match self {
            Mbc::None => {}
            Mbc::Mbc1(mbc1) => mbc1.handle_write(address, value),
        }
    }

    pub fn get_lower_rom_index(&self) -> usize {
        match self {
            Mbc::None => 0,
            Mbc::Mbc1(mbc1) => mbc1.get_lower_rom_index(),
        }
    }

    pub fn get_upper_rom_index(&self) -> usize {
        match self {
            Mbc::None => 1,
            Mbc::Mbc1(mbc1) => mbc1.get_upper_rom_index(),
        }
    }

    pub fn get_ram_index(&self) -> usize {
        match self {
            Mbc::None => 0,
            Mbc::Mbc1(mbc1) => mbc1.get_ram_index(),
        }
    }

    pub fn ram_enabled(&self) -> bool {
        match self {
            Mbc::None => true,
            Mbc::Mbc1(mbc1) => mbc1.ram_enabled(),
        }
    }
}
