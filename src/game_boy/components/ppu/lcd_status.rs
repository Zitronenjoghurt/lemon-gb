use crate::game_boy::components::ppu::mode::PPUMode;

/// https://gbdev.io/pandocs/LCDC.html#lcd-status
#[derive(Debug, Clone, PartialEq)]
pub struct LCDStatus {
    pub lyc_interrupt: bool,
    pub mode2_interrupt: bool, // OAMSearch interrupt
    pub mode1_interrupt: bool, // VBlank interrupt
    pub mode0_interrupt: bool, // HBlank interrupt
    pub lyc_equals_ly: bool,
    pub ppu_mode: PPUMode,
}

impl From<u8> for LCDStatus {
    fn from(value: u8) -> Self {
        Self {
            lyc_interrupt: (value & 0b0100_0000) != 0,
            mode2_interrupt: (value & 0b0010_0000) != 0,
            mode1_interrupt: (value & 0b0001_0000) != 0,
            mode0_interrupt: (value & 0b0000_1000) != 0,
            lyc_equals_ly: (value & 0b0000_0100) != 0,
            ppu_mode: value.into(),
        }
    }
}

impl From<LCDStatus> for u8 {
    fn from(value: LCDStatus) -> Self {
        (
            if value.lyc_interrupt { 0b0100_0000 } else { 0 }
            | if value.mode2_interrupt { 0b0010_0000 } else { 0 }
            | if value.mode1_interrupt { 0b0001_0000 } else { 0 }
            | if value.mode0_interrupt { 0b0000_1000} else { 0 }
            | if value.lyc_equals_ly { 0b0000_0100 } else { 0 }
            | value.ppu_mode as u8
        )
    }
}
