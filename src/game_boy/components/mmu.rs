use crate::enums::interrupts::Interrupt;
use crate::game_boy::components::cartridge::header::CartridgeHeader;
use crate::game_boy::components::cartridge::Cartridge;
use crate::game_boy::components::mmu::builder::MMUBuilder;
use crate::game_boy::components::mmu::mbc::Mbc;
use crate::game_boy::components::mmu::save_state::MMUSaveState;
use crate::helpers::bit_operations::construct_u16;
use std::error::Error;

mod builder;
pub mod mbc;
pub mod save_state;

pub const ROM_BANK_SIZE: usize = 0x4000; // 16KB
const RAM_BANK_SIZE: usize = 0x2000; // 8KB
const VRAM_SIZE: usize = 0x2000; // 8KB
const WRAM_SIZE: usize = 0x2000; // 8KB
const OAM_SIZE: usize = 160; // Bytes
const HRAM_SIZE: usize = 127; // Bytes
const IO_REGISTERS_SIZE: usize = 160; // Bytes

// Initial hardware registers using the DMG0 Model
// https://gbdev.io/pandocs/Power_Up_Sequence.html?highlight=state#console-state-after-boot-rom-hand-off
const INITIAL_P1: u8 = 0xCF;
const INITIAL_SB: u8 = 0x00;
const INITIAL_SC: u8 = 0x7E;
pub const INITIAL_DIV: u8 = 0x18;
const INITIAL_TIMA: u8 = 0x00;
const INITIAL_TMA: u8 = 0x00;
const INITIAL_TAC: u8 = 0xF8;
const INITIAL_IF: u8 = 0xE1;
const INITIAL_NR10: u8 = 0x80;
const INITIAL_NR11: u8 = 0xBF;
const INITIAL_NR12: u8 = 0xF3;
const INITIAL_NR13: u8 = 0xFF;
const INITIAL_NR14: u8 = 0xBF;
const INITIAL_NR21: u8 = 0x3F;
const INITIAL_NR22: u8 = 0x00;
const INITIAL_NR23: u8 = 0xFF;
const INITIAL_NR24: u8 = 0xBF;
const INITIAL_NR30: u8 = 0x7F;
const INITIAL_NR31: u8 = 0xFF;
const INITIAL_NR32: u8 = 0x9F;
const INITIAL_NR33: u8 = 0xFF;
const INITIAL_NR34: u8 = 0xBF;
const INITIAL_NR41: u8 = 0xFF;
const INITIAL_NR42: u8 = 0x00;
const INITIAL_NR43: u8 = 0x00;
const INITIAL_NR44: u8 = 0xBF;
const INITIAL_NR50: u8 = 0x77;
const INITIAL_NR51: u8 = 0xF3;
const INITIAL_NR52: u8 = 0xF1;
const INITIAL_LCDC: u8 = 0x91;
const INITIAL_STAT: u8 = 0x81;
const INITIAL_SCY: u8 = 0x00;
const INITIAL_SCX: u8 = 0x00;
const INITIAL_LY: u8 = 0x91;
const INITIAL_LYC: u8 = 0x00;
const INITIAL_DMA: u8 = 0xFF;
const INITIAL_BGP: u8 = 0xFC;
const INITIAL_WY: u8 = 0x00;
const INITIAL_WX: u8 = 0x00;
const INITIAL_IE: u8 = 0x00;

// IMPORTANT ADDRESSES
// Timer
pub const DIV_ADDRESS: u16 = 0xFF04;
pub const TIMA_ADDRESS: u16 = 0xFF05;
pub const TMA_ADDRESS: u16 = 0xFF06;
pub const TAC_ADDRESS: u16 = 0xFF07;

// Interrupts
pub const IF_ADDRESS: u16 = 0xFF0F;
pub const IE_ADDRESS: u16 = 0xFFFF;

// Graphics
pub const LCDC_ADDRESS: u16 = 0xFF40;
pub const STAT_ADDRESS: u16 = 0xFF41;
pub const SCY_ADDRESS: u16 = 0xFF42;
pub const SCX_ADDRESS: u16 = 0xFF43;
pub const LY_ADDRESS: u16 = 0xFF44;
pub const LYC_ADDRESS: u16 = 0xFF45;
pub const DMA_ADDRESS: u16 = 0xFF46;
pub const BGP_ADDRESS: u16 = 0xFF47; // Background color palette

#[derive(Debug, Clone, PartialEq)]
pub struct MMU {
    pub cartridge_header: CartridgeHeader,

    mbc: Mbc,
    rom_banks: Vec<[u8; ROM_BANK_SIZE]>,
    ram_banks: Vec<[u8; RAM_BANK_SIZE]>,

    vram: [u8; VRAM_SIZE],
    wram: [u8; WRAM_SIZE],

    oam: [u8; OAM_SIZE],
    io_registers: [u8; IO_REGISTERS_SIZE],
    hram: [u8; HRAM_SIZE],
    ie_register: u8,
}

impl MMU {
    pub fn builder() -> MMUBuilder {
        MMUBuilder::new()
    }

    pub fn initialize(cartridge: &Cartridge) -> Self {
        Self {
            cartridge_header: cartridge.header.clone(),
            mbc: Mbc::initialize(cartridge.header.cartridge_type.into()),
            rom_banks: cartridge.rom_banks.clone(),
            ram_banks: vec![[0; RAM_BANK_SIZE]; cartridge.header.ram_size],
            vram: [0; VRAM_SIZE],
            wram: [0; WRAM_SIZE],
            oam: [0; OAM_SIZE],
            io_registers: Self::initialize_io_registers(),
            hram: [0; HRAM_SIZE],
            ie_register: INITIAL_IE,
        }
    }

    // Using the DMG0 model
    pub fn initialize_io_registers() -> [u8; IO_REGISTERS_SIZE] {
        let absolute_address: usize = 0xFF00;
        let mut io_registers = [0u8; IO_REGISTERS_SIZE];
        io_registers[0xFF00 - absolute_address] = INITIAL_P1;
        io_registers[0xFF01 - absolute_address] = INITIAL_SB;
        io_registers[0xFF02 - absolute_address] = INITIAL_SC;
        io_registers[0xFF04 - absolute_address] = INITIAL_DIV;
        io_registers[0xFF05 - absolute_address] = INITIAL_TIMA;
        io_registers[0xFF06 - absolute_address] = INITIAL_TMA;
        io_registers[0xFF07 - absolute_address] = INITIAL_TAC;
        io_registers[0xFF0F - absolute_address] = INITIAL_IF;
        io_registers[0xFF10 - absolute_address] = INITIAL_NR10;
        io_registers[0xFF11 - absolute_address] = INITIAL_NR11;
        io_registers[0xFF12 - absolute_address] = INITIAL_NR12;
        io_registers[0xFF13 - absolute_address] = INITIAL_NR13;
        io_registers[0xFF14 - absolute_address] = INITIAL_NR14;
        io_registers[0xFF16 - absolute_address] = INITIAL_NR21;
        io_registers[0xFF17 - absolute_address] = INITIAL_NR22;
        io_registers[0xFF18 - absolute_address] = INITIAL_NR23;
        io_registers[0xFF19 - absolute_address] = INITIAL_NR24;
        io_registers[0xFF1A - absolute_address] = INITIAL_NR30;
        io_registers[0xFF1B - absolute_address] = INITIAL_NR31;
        io_registers[0xFF1C - absolute_address] = INITIAL_NR32;
        io_registers[0xFF1D - absolute_address] = INITIAL_NR33;
        io_registers[0xFF1E - absolute_address] = INITIAL_NR34;
        io_registers[0xFF20 - absolute_address] = INITIAL_NR41;
        io_registers[0xFF21 - absolute_address] = INITIAL_NR42;
        io_registers[0xFF22 - absolute_address] = INITIAL_NR43;
        io_registers[0xFF23 - absolute_address] = INITIAL_NR44;
        io_registers[0xFF24 - absolute_address] = INITIAL_NR50;
        io_registers[0xFF25 - absolute_address] = INITIAL_NR51;
        io_registers[0xFF26 - absolute_address] = INITIAL_NR52;
        io_registers[0xFF40 - absolute_address] = INITIAL_LCDC;
        io_registers[0xFF41 - absolute_address] = INITIAL_STAT;
        io_registers[0xFF42 - absolute_address] = INITIAL_SCY;
        io_registers[0xFF43 - absolute_address] = INITIAL_SCX;
        io_registers[0xFF44 - absolute_address] = INITIAL_LY;
        io_registers[0xFF45 - absolute_address] = INITIAL_LYC;
        io_registers[0xFF46 - absolute_address] = INITIAL_DMA;
        io_registers[0xFF47 - absolute_address] = INITIAL_BGP;
        io_registers[0xFF4A - absolute_address] = INITIAL_WY;
        io_registers[0xFF4B - absolute_address] = INITIAL_WX;
        io_registers
    }

    #[allow(unreachable_patterns)]
    pub fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => self.get_rom(self.mbc.get_lower_rom_index(), address),
            0x4000..=0x7FFF => self.get_rom(self.mbc.get_upper_rom_index(), address - 0x4000),
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
            0x0000..=0x3FFF => self.set_rom(self.mbc.get_lower_rom_index(), address, value),
            0x4000..=0x7FFF => {
                self.set_rom(self.mbc.get_upper_rom_index(), address - 0x4000, value)
            }
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

    pub fn timer_update_div(&mut self, value: u8) {
        let div_index = DIV_ADDRESS - 0xFF00;
        self.io_registers[div_index as usize] = value;
    }

    pub fn force_write_rom(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x3FFF => {
                self.rom_banks[self.mbc.get_lower_rom_index()][address as usize] = value
            }
            0x4000..=0x7FFF => {
                let index = address - 0x4000;
                self.rom_banks[self.mbc.get_upper_rom_index()][index as usize] = value
            }
            _ => {}
        }
    }

    /// Fetches an interrupt by the provided priority and resets the IF flag
    pub fn get_interrupt(&self) -> Option<Interrupt> {
        let i_enable = self.get_ie_register();
        let i_flag = self.read(IF_ADDRESS);
        Interrupt::from_ie_if(i_enable & i_flag)
    }

    pub fn save(&self) -> MMUSaveState {
        MMUSaveState {
            mbc: self.mbc.clone(),
            ram: self.ram_banks.iter().map(|bank| bank.to_vec()).collect(),
            vram: self.vram.to_vec(),
            wram: self.wram.to_vec(),
            oam: self.oam.to_vec(),
            io_registers: self.io_registers.to_vec(),
            hram: self.hram.to_vec(),
            ie_register: self.ie_register,
        }
    }

    pub fn load(state: MMUSaveState, cartridge: &Cartridge) -> Result<Self, Box<dyn Error>> {
        let ram_banks = state
            .ram
            .into_iter()
            .map(|bank| bank.try_into().map_err(|_| "Failed to load RAM banks"))
            .collect::<Result<Vec<[u8; RAM_BANK_SIZE]>, &str>>()?;

        Ok(Self {
            cartridge_header: cartridge.header.clone(),
            mbc: state.mbc,
            rom_banks: cartridge.rom_banks.clone(),
            ram_banks,
            vram: state.vram.try_into().map_err(|_| "Failed to load VRAM")?,
            wram: state.wram.try_into().map_err(|_| "Failed to load WRAM")?,
            oam: state.oam.try_into().map_err(|_| "Failed to load OAM")?,
            io_registers: state
                .io_registers
                .try_into()
                .map_err(|_| "Failed to load IO registers")?,
            hram: state.hram.try_into().map_err(|_| "Failed to load HRAM")?,
            ie_register: state.ie_register,
        })
    }
}

/// Memory access functions
/// ToDo: Proper MBC Type Behavior
impl MMU {
    fn get_rom(&self, bank: usize, index: u16) -> u8 {
        self.rom_banks[bank][index as usize]
    }

    fn set_rom(&mut self, _bank: usize, index: u16, value: u8) {
        self.mbc.handle_write(index, value);
        // We won't write to ROM anymore
        // self.rom_banks[bank][index as usize] = value;
    }

    fn get_vram(&self, index: u16) -> u8 {
        self.vram[index as usize]
    }

    fn set_vram(&mut self, index: u16, value: u8) {
        self.vram[index as usize] = value;
    }

    fn get_ram(&self, index: u16) -> u8 {
        if !self.ram_banks.is_empty() && self.mbc.ram_enabled() {
            self.ram_banks[self.mbc.get_ram_index()][index as usize]
        } else {
            // Pan Docs say this is not guaranteed, but often the case
            0xFF
        }
    }

    fn set_ram(&mut self, index: u16, value: u8) {
        if !self.ram_banks.is_empty() && self.mbc.ram_enabled() {
            self.ram_banks[self.mbc.get_ram_index()][index as usize] = value;
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
        let div_index: u16 = 0xFF04 - 0xFF00;
        if index == div_index {
            // Write to DIV, reset it
            self.io_registers[div_index as usize] = 0;
        } else {
            self.io_registers[index as usize] = value;
        }
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
            cartridge_header: CartridgeHeader::default(),
            mbc: Mbc::None,
            rom_banks: vec![[0; ROM_BANK_SIZE]; 2],
            ram_banks: vec![[0; RAM_BANK_SIZE]; 1],
            vram: [0; VRAM_SIZE],
            wram: [0; WRAM_SIZE],
            oam: [0; OAM_SIZE],
            io_registers: [0; IO_REGISTERS_SIZE],
            hram: [0; HRAM_SIZE],
            ie_register: 0,
        }
    }
}
