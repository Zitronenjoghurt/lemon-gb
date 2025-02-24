use serde::{Deserialize, Serialize};

// ToDo: Check if lower bit masking depending on ROM size is necessary
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mbc1 {
    bank1: u8,
    bank2: u8,
    ram_enabled: bool,
    banking_mode: bool,
    multicart: bool,
}

impl Mbc1 {
    pub fn initialize(multicart: bool) -> Self {
        Self {
            bank1: 0b0000_0001,
            bank2: 0b0000_0000,
            ram_enabled: false,
            banking_mode: false,
            multicart,
        }
    }

    pub fn handle_write(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => {
                self.ram_enabled = value & 0b0000_1111 == 0xA;
            }
            0x2000..=0x3FFF => {
                let masked_value = value & 0b0001_1111;
                self.bank1 = if masked_value == 0 { 1 } else { masked_value }
            }
            0x4000..=0x5FFF => {
                let masked_value = value & 0b0000_0011;
                self.bank2 = masked_value;
            }
            0x6000..=0x7FFF => {
                self.banking_mode = value & 0b1 == 0b1;
            }
            _ => (),
        }
    }

    pub fn ram_enabled(&self) -> bool {
        self.ram_enabled
    }

    pub fn get_lower_rom_index(&self) -> usize {
        if self.banking_mode {
            self.get_upper_bits() as usize
        } else {
            0b0000_0000
        }
    }

    pub fn get_upper_rom_index(&self) -> usize {
        (self.get_upper_bits() | self.get_lower_bits()) as usize
    }

    pub fn get_ram_index(&self) -> usize {
        if self.banking_mode {
            self.bank2 as usize
        } else {
            0b0000_0000
        }
    }

    fn get_lower_bits(&self) -> u8 {
        if self.multicart {
            self.bank1 & 0b0000_1111
        } else {
            self.bank1
        }
    }

    fn get_upper_bits(&self) -> u8 {
        if self.multicart {
            self.bank2 << 4
        } else {
            self.bank2 << 5
        }
    }
}
