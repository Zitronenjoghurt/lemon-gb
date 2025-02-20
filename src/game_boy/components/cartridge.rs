pub mod types;

/// Returns the amount of rom banks the cartridge uses (if valid)
pub fn parse_rom_size(byte: u8) -> Option<usize> {
    match byte {
        0x00 => Some(2),
        0x01 => Some(4),
        0x02 => Some(8),
        0x03 => Some(16),
        0x04 => Some(32),
        0x05 => Some(64),
        0x06 => Some(128),
        0x07 => Some(256),
        0x08 => Some(512),
        _ => None,
    }
}

/// Returns the amount of ram banks the cartridge uses (if valid)
pub fn parse_ram_size(byte: u8) -> Option<usize> {
    match byte {
        0x00 => Some(0),
        0x02 => Some(1),
        0x03 => Some(4),
        0x04 => Some(16),
        0x05 => Some(8),
        _ => None,
    }
}
