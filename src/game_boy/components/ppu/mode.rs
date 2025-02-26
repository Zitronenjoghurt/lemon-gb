use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum PPUMode {
    HBlank = 0,
    VBlank = 1,
    #[default]
    OAMSearch = 2,
    PixelTransfer = 3,
}

impl PPUMode {
    pub fn get_mode_bits(&self) -> u8 {
        match self {
            PPUMode::HBlank => 0b00,
            PPUMode::VBlank => 0b01,
            PPUMode::OAMSearch => 0b10,
            PPUMode::PixelTransfer => 0b11,
        }
    }
}

impl From<u8> for PPUMode {
    fn from(value: u8) -> Self {
        match value & 0b0000_0011 {
            0b00 => PPUMode::HBlank,
            0b01 => PPUMode::VBlank,
            0b10 => PPUMode::OAMSearch,
            0b11 => PPUMode::PixelTransfer,
            _ => unreachable!(),
        }
    }
}
