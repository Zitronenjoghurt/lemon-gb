use crate::helpers::bit_operations::get_bit_u8;

pub const INTERRUPT_VBLANK: u8 = 0b0000_0001;
pub const INTERRUPT_LCD: u8 = 0b0000_0010;
pub const INTERRUPT_TIMER: u8 = 0b0000_0100;
pub const INTERRUPT_SERIAL: u8 = 0b0000_1000;
pub const INTERRUPT_JOYPAD: u8 = 0b0001_0000;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Interrupt {
    Vblank = 0,
    Lcd = 1,
    Timer = 2,
    Serial = 3,
    Joypad = 4,
}

impl Interrupt {
    pub fn from_ie_if(ie_and_if: u8) -> Option<Interrupt> {
        if get_bit_u8(ie_and_if, 0) {
            Some(Interrupt::Vblank)
        } else if get_bit_u8(ie_and_if, 1) {
            Some(Interrupt::Lcd)
        } else if get_bit_u8(ie_and_if, 2) {
            Some(Interrupt::Timer)
        } else if get_bit_u8(ie_and_if, 3) {
            Some(Interrupt::Serial)
        } else if get_bit_u8(ie_and_if, 4) {
            Some(Interrupt::Joypad)
        } else {
            None
        }
    }

    pub fn get_if_index(&self) -> usize {
        match self {
            Interrupt::Vblank => 0,
            Interrupt::Lcd => 1,
            Interrupt::Timer => 2,
            Interrupt::Serial => 3,
            Interrupt::Joypad => 4,
        }
    }

    pub fn get_target_address(&self) -> u16 {
        match self {
            Interrupt::Vblank => 0x40,
            Interrupt::Lcd => 0x48,
            Interrupt::Timer => 0x50,
            Interrupt::Serial => 0x58,
            Interrupt::Joypad => 0x60,
        }
    }

    pub fn get_mask(&self) -> u8 {
        match self {
            Interrupt::Vblank => 0b0000_0001,
            Interrupt::Lcd => 0b0000_0010,
            Interrupt::Timer => 0b0000_0100,
            Interrupt::Serial => 0b0000_1000,
            Interrupt::Joypad => 0b0001_0000,
        }
    }
}
