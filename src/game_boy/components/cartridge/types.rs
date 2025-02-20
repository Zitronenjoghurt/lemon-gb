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

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CartridgeType {
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
