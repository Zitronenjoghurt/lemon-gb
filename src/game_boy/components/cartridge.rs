use crate::game_boy::components::cartridge::header::CartridgeHeader;
use crate::game_boy::components::mmu::ROM_BANK_SIZE;
use std::error::Error;
use std::path::PathBuf;

pub mod header;
pub mod types;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Cartridge {
    pub rom_banks: Vec<[u8; ROM_BANK_SIZE]>,
    pub header: CartridgeHeader,
}

impl Cartridge {
    pub fn load(path: PathBuf) -> Result<Cartridge, Box<dyn Error>> {
        let data = std::fs::read(path)?;
        let header = CartridgeHeader::parse(&data)?;

        let mut rom_banks = Vec::with_capacity(header.rom_size);
        for bank_index in 0..header.rom_size {
            let mut bank = [0u8; ROM_BANK_SIZE];
            let start = bank_index * ROM_BANK_SIZE;

            if start < data.len() {
                let end = (start + ROM_BANK_SIZE).min(data.len());
                bank[..(end - start)].copy_from_slice(&data[start..end]);
            }

            rom_banks.push(bank);
        }

        Ok(Cartridge { rom_banks, header })
    }
}
