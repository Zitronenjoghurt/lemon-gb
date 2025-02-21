use std::error::Error;

/// This will tell the MMU how to behave during memory access
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MbcType {
    // 32 KiB ROM, optionally 8 KiB of RAM
    None,
    MBC1,
    MBC2,
    MBC3,
    MBC5,
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub enum CartridgeType {
    #[default]
    RomOnly = 0x00,
    MBC1 = 0x01,
    MBC1Ram = 0x02,
    MBC1RamBattery = 0x03,
    MBC2 = 0x05,
    MBC2Battery = 0x06,
    // No licensed cartridge makes use of this option. The exact behavior is unknown.
    RomRam = 0x08,
    // No licensed cartridge makes use of this option. The exact behavior is unknown.
    RomRamBattery = 0x09,
    MMM01 = 0x0B,
    MMM01Ram = 0x0C,
    MMM01RamBattery = 0x0D,
    MBC3TimerBattery = 0x0F,
    // MBC3 with 64 KiB of SRAM refers to MBC30, used only in Pocket Monsters: Crystal Version (the Japanese version of Pokémon Crystal Version).
    MBC3TimerRamBattery = 0x10,
    MBC3 = 0x11,
    // MBC3 with 64 KiB of SRAM refers to MBC30, used only in Pocket Monsters: Crystal Version (the Japanese version of Pokémon Crystal Version).
    MBC3Ram = 0x12,
    // MBC3 with 64 KiB of SRAM refers to MBC30, used only in Pocket Monsters: Crystal Version (the Japanese version of Pokémon Crystal Version).
    MBC3RamBattery = 0x13,
    MBC5 = 0x19,
    MBC5Ram = 0x1A,
    MBC5RamBattery = 0x1B,
    MBC5Rumble = 0x1C,
    MBC5RumbleRam = 0x1D,
    MBC5RumbleRamBattery = 0x1E,
    MBC6 = 0x20,
    MBC7SensorRumbleRamBattery = 0x22,
    PocketCamera = 0xFC,
    BandaiTama5 = 0xFD,
    HuC3 = 0xFE,
    HuC1RamBattery = 0xFF,
}

impl TryFrom<u8> for CartridgeType {
    type Error = Box<dyn Error>;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Self::RomOnly),
            0x01 => Ok(Self::MBC1),
            0x02 => Ok(Self::MBC1Ram),
            0x03 => Ok(Self::MBC1RamBattery),
            0x05 => Ok(Self::MBC2),
            0x06 => Ok(Self::MBC2Battery),
            0x08 => Ok(Self::RomRam),
            0x09 => Ok(Self::RomRamBattery),
            0x0B => Ok(Self::MMM01),
            0x0C => Ok(Self::MMM01Ram),
            0x0D => Ok(Self::MMM01RamBattery),
            0x0F => Ok(Self::MBC3TimerBattery),
            0x10 => Ok(Self::MBC3TimerRamBattery),
            0x11 => Ok(Self::MBC3),
            0x12 => Ok(Self::MBC3Ram),
            0x13 => Ok(Self::MBC3RamBattery),
            0x19 => Ok(Self::MBC5),
            0x1A => Ok(Self::MBC5Ram),
            0x1B => Ok(Self::MBC5RamBattery),
            0x1C => Ok(Self::MBC5Rumble),
            0x1D => Ok(Self::MBC5RumbleRam),
            0x1E => Ok(Self::MBC5RumbleRamBattery),
            0x20 => Ok(Self::MBC6),
            0x22 => Ok(Self::MBC7SensorRumbleRamBattery),
            0xFC => Ok(Self::PocketCamera),
            0xFD => Ok(Self::BandaiTama5),
            0xFE => Ok(Self::HuC3),
            0xFF => Ok(Self::HuC1RamBattery),
            _ => Err(format!("Invalid cartridge type: {}", value).into()),
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub enum CartridgeCGBFlag {
    #[default]
    None,
    GBCompatible = 0x80,
    CGBOnly = 0xC0,
}

impl From<u8> for CartridgeCGBFlag {
    fn from(value: u8) -> CartridgeCGBFlag {
        match value {
            0x80 => CartridgeCGBFlag::GBCompatible,
            0xC0 => CartridgeCGBFlag::CGBOnly,
            _ => CartridgeCGBFlag::None,
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub enum CartridgeDestinationCode {
    #[default]
    None = 0xFF,
    JapanAndPossiblyOverseas = 0x00,
    OverseasOnly = 0x01,
}

impl From<u8> for CartridgeDestinationCode {
    fn from(value: u8) -> CartridgeDestinationCode {
        match value {
            0x00 => CartridgeDestinationCode::JapanAndPossiblyOverseas,
            0x01 => CartridgeDestinationCode::OverseasOnly,
            _ => CartridgeDestinationCode::None,
        }
    }
}
