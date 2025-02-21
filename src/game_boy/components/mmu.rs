use crate::game_boy::components::cartridge::types::MbcType;
use crate::game_boy::components::mmu::builder::MMUBuilder;
use crate::helpers::bit_operations::construct_u16;

mod builder;

pub const ROM_BANK_SIZE: usize = 0x4000; // 16KB
const RAM_BANK_SIZE: usize = 0x2000; // 8KB
const VRAM_SIZE: usize = 0x2000; // 8KB
const WRAM_SIZE: usize = 0x2000; // 8KB
const OAM_SIZE: usize = 160; // Bytes
const HRAM_SIZE: usize = 127; // Bytes
const IO_REGISTERS_SIZE: usize = 160; // Bytes

#[derive(Debug, Clone, PartialEq)]
pub struct MMU {
    rom_banks: Vec<[u8; ROM_BANK_SIZE]>,
    current_rom_bank: usize,

    ram_banks: Vec<[u8; RAM_BANK_SIZE]>,
    current_ram_bank: usize,
    ram_enabled: bool,

    vram: [u8; VRAM_SIZE],
    wram: [u8; WRAM_SIZE],

    oam: [u8; OAM_SIZE],
    io_registers: [u8; IO_REGISTERS_SIZE],
    hram: [u8; HRAM_SIZE],
    ie_register: u8,

    mbc_type: MbcType,
    banking_mode: u8,
}

impl MMU {
    pub fn builder() -> MMUBuilder {
        MMUBuilder::new()
    }

    pub fn initialize(rom_banks: usize, ram_banks: usize, mbc_type: MbcType) -> Self {
        Self {
            rom_banks: vec![[0; ROM_BANK_SIZE]; rom_banks],
            current_rom_bank: 1,
            ram_banks: vec![[0; RAM_BANK_SIZE]; ram_banks],
            current_ram_bank: 0,
            ram_enabled: false,
            vram: [0; VRAM_SIZE],
            wram: [0; WRAM_SIZE],
            oam: [0; OAM_SIZE],
            io_registers: [0; IO_REGISTERS_SIZE],
            hram: [0; HRAM_SIZE],
            ie_register: 0,
            mbc_type,
            banking_mode: 0,
        }
    }

    #[allow(unreachable_patterns)]
    pub fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => self.get_rom(0, address),
            0x4000..=0x7FFF => self.get_rom(self.current_rom_bank, address - 0x4000),
            0x8000..=0x9FFF => self.get_vram(address - 0x8000),
            0xA000..=0xBFFF => self.get_ram(address - 0xA000),
            0xC000..=0xDFFF => self.get_wram(address - 0xC000),
            0xE000..=0xFDFF => self.get_wram(address - 0xE000),
            0xFE00..=0xFE9F => self.get_oam(address - 0xFE00),
            0xFEA0..=0xFEFF => self.get_unusable(),
            0xFF00..=0xFF7F => self.get_io_register(address - 0xFF00),
            0xFF80..=0xFFFE => self.get_hram(address - 0xFF80),
            0xFFFF => self.get_ie_register(),
            _ => unreachable!(),
        }
    }

    #[allow(unreachable_patterns)]
    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x3FFF => self.set_rom(0, address, value),
            0x4000..=0x7FFF => self.set_rom(self.current_rom_bank, address - 0x4000, value),
            0x8000..=0x9FFF => self.set_vram(address - 0x8000, value),
            0xA000..=0xBFFF => self.set_ram(address - 0xA000, value),
            0xC000..=0xDFFF => self.set_wram(address - 0xC000, value),
            0xE000..=0xFDFF => self.set_wram(address - 0xE000, value),
            0xFE00..=0xFE9F => self.set_oam(address - 0xFE00, value),
            0xFEA0..=0xFEFF => self.set_unusable(value),
            0xFF00..=0xFF7F => self.set_io_register(address - 0xFF00, value),
            0xFF80..=0xFFFE => self.set_hram(address - 0xFF80, value),
            0xFFFF => self.set_ie_register(value),
            _ => unreachable!(),
        }
    }

    pub fn read_16(&self, address: u16) -> u16 {
        let lsb = self.read(address);
        let msb = self.read(address.wrapping_add(1));
        construct_u16(lsb, msb)
    }
}

/// Memory access functions
/// ToDo: Proper MBC Type Behavior
impl MMU {
    fn get_rom(&self, bank: usize, index: u16) -> u8 {
        self.rom_banks[bank][index as usize]
    }

    fn set_rom(&mut self, bank: usize, index: u16, value: u8) {
        self.rom_banks[bank][index as usize] = value;
    }

    fn get_vram(&self, index: u16) -> u8 {
        self.vram[index as usize]
    }

    fn set_vram(&mut self, index: u16, value: u8) {
        self.vram[index as usize] = value;
    }

    fn get_ram(&self, index: u16) -> u8 {
        if !self.ram_banks.is_empty() && self.ram_enabled {
            self.ram_banks[self.current_ram_bank][index as usize]
        } else {
            0xFF
        }
    }

    fn set_ram(&mut self, index: u16, value: u8) {
        if !self.ram_banks.is_empty() && self.ram_enabled {
            self.ram_banks[self.current_ram_bank][index as usize] = value;
        }
    }

    fn get_wram(&self, index: u16) -> u8 {
        self.wram[index as usize]
    }

    fn set_wram(&mut self, index: u16, value: u8) {
        self.wram[index as usize] = value;
    }

    fn get_oam(&self, index: u16) -> u8 {
        self.oam[index as usize]
    }

    fn set_oam(&mut self, index: u16, value: u8) {
        self.oam[index as usize] = value;
    }

    fn get_unusable(&self) -> u8 {
        // ToDo: On OAM Block this should trigger OAM corruption
        0x00
    }

    fn set_unusable(&mut self, _value: u8) {
        // ToDo: Potentially do something?
    }

    fn get_io_register(&self, index: u16) -> u8 {
        self.io_registers[index as usize]
    }

    fn set_io_register(&mut self, index: u16, value: u8) {
        self.io_registers[index as usize] = value;
    }

    fn get_hram(&self, index: u16) -> u8 {
        self.hram[index as usize]
    }

    fn set_hram(&mut self, index: u16, value: u8) {
        self.hram[index as usize] = value;
    }

    fn get_ie_register(&self) -> u8 {
        self.ie_register
    }

    fn set_ie_register(&mut self, value: u8) {
        self.ie_register = value;
    }
}

impl Default for MMU {
    fn default() -> Self {
        Self {
            rom_banks: vec![[0; ROM_BANK_SIZE]; 2],
            current_rom_bank: 1,
            ram_banks: vec![[0; RAM_BANK_SIZE]; 1],
            current_ram_bank: 0,
            ram_enabled: false,
            vram: [0; VRAM_SIZE],
            wram: [0; WRAM_SIZE],
            oam: [0; OAM_SIZE],
            io_registers: [0; IO_REGISTERS_SIZE],
            hram: [0; HRAM_SIZE],
            ie_register: 0,
            mbc_type: MbcType::None,
            banking_mode: 0,
        }
    }
}
