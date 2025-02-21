use crate::game_boy::components::cartridge::types::{
    CartridgeCGBFlag, CartridgeDestinationCode, CartridgeType,
};
use crate::helpers::bit_operations::construct_u16;
use crate::instructions::Instruction;
use std::error::Error;
use std::fmt::Debug;

const NINTENDO_LOGO: [u8; 48] = [
    0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D,
    0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99,
    0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
];

#[derive(Debug, Default, Clone, PartialEq)]
pub struct CartridgeHeader {
    /// The first 2-4 instructions of the ROM, usually that's where they jump to the actual ROM entry point
    pub entry_point: Vec<String>,
    pub valid_nintendo_logo: bool,
    pub title: String,
    /// In older cartridges these bytes were part of the Title (see above). In newer cartridges they contain a 4-character manufacturer code (in uppercase ASCII). The purpose of the manufacturer code is unknown.
    pub manufacturer_code: String,
    pub cgb_flag: CartridgeCGBFlag,
    pub licensee: String,
    pub cartridge_type: CartridgeType,
    /// The amount of ROM banks this cartridge uses
    pub rom_size: usize,
    /// The amount of RAM banks this cartridge uses
    pub ram_size: usize,
    pub destination_code: CartridgeDestinationCode,
    pub mask_rom_version: u8,
    pub header_checksum: u8,
    pub global_checksum: u16,
}

impl CartridgeHeader {
    pub fn parse(rom: &[u8]) -> Result<Self, Box<dyn Error>> {
        if rom.len() < 0x150 {
            return Err("ROM is too small, there is no header to read".into());
        }

        let header = Self {
            entry_point: Self::parse_entry_point(rom[0x100..=0x103].try_into()?)?,
            valid_nintendo_logo: Self::parse_nintendo_logo(rom[0x104..=0x133].try_into()?),
            title: Self::parse_ascii(&rom[0x134..=0x143]),
            manufacturer_code: Self::parse_ascii(&rom[0x13F..=0x142]),
            cgb_flag: rom[0x143].into(),
            licensee: Self::parse_licensee(rom[0x14B], rom[0x144..=0x145].try_into()?),
            cartridge_type: CartridgeType::try_from(rom[0x147])?,
            rom_size: Self::parse_rom_size(rom[0x148])?,
            ram_size: Self::parse_ram_size(rom[0x149])?,
            destination_code: rom[0x14A].into(),
            mask_rom_version: rom[0x14C],
            header_checksum: rom[0x14D],
            global_checksum: Self::parse_global_checksum(rom[0x14E..=0x14F].try_into()?),
        };

        Ok(header)
    }

    fn parse_entry_point(entry_point: &[u8; 4]) -> Result<Vec<String>, Box<dyn Error>> {
        Instruction::parse_clear_text_instructions_from_data(entry_point, true)
            .map_err(|e| format!("Unable to parse cartridge entry point: {}", e).into())
    }

    fn parse_nintendo_logo(data: &[u8; 48]) -> bool {
        *data == NINTENDO_LOGO
    }

    fn parse_ascii(data: &[u8]) -> String {
        data.iter()
            .take_while(|&&byte| byte != 0)
            .filter(|&&byte| byte.is_ascii())
            .map(|&byte| byte as char)
            .collect::<String>()
            .trim()
            .to_string()
    }

    fn parse_rom_size(data: u8) -> Result<usize, Box<dyn Error>> {
        match data {
            0x00 => Ok(2),
            0x01 => Ok(4),
            0x02 => Ok(8),
            0x03 => Ok(16),
            0x04 => Ok(32),
            0x05 => Ok(64),
            0x06 => Ok(128),
            0x07 => Ok(256),
            0x08 => Ok(512),
            _ => Err(format!("Invalid ROM size: 0x{:02X}", data).into()),
        }
    }

    fn parse_ram_size(data: u8) -> Result<usize, Box<dyn Error>> {
        match data {
            0x00 => Ok(0),
            0x02 => Ok(1),
            0x03 => Ok(4),
            0x04 => Ok(16),
            0x05 => Ok(8),
            _ => Err(format!("Invalid RAM size: 0x{:02X}", data).into()),
        }
    }

    fn parse_licensee(old_licensee: u8, new_licensee: [u8; 2]) -> String {
        if old_licensee == 0x33 {
            Self::parse_new_licensee(new_licensee)
        } else {
            Self::parse_old_licensee(old_licensee)
        }
    }

    fn parse_old_licensee(old_licensee: u8) -> String {
        match old_licensee {
            0x00 => "None".to_string(),
            0x01 => "Nintendo".to_string(),
            0x08 => "Capcom".to_string(),
            0x09 => "HOT-B".to_string(),
            0x0A => "Jaleco".to_string(),
            0x0B => "Coconuts Japan".to_string(),
            0x0C => "Elite Systems".to_string(),
            0x13 => "EA (Electronic Arts)".to_string(),
            0x18 => "Hudson Soft".to_string(),
            0x19 => "ITC Entertainment".to_string(),
            0x1A => "Yanoman".to_string(),
            0x1D => "Japan Clary".to_string(),
            0x1F => "Virgin Games Ltd.".to_string(),
            0x24 => "PCM Complete".to_string(),
            0x25 => "San-X".to_string(),
            0x28 => "Kemco".to_string(),
            0x29 => "SETA Corporation".to_string(),
            0x30 => "Infogrames".to_string(),
            0x31 => "Nintendo".to_string(),
            0x32 => "Bandai".to_string(),
            0x33 => "Use New Licensee Code".to_string(),
            0x34 => "Konami".to_string(),
            0x35 => "HectorSoft".to_string(),
            0x38 => "Capcom".to_string(),
            0x39 => "Banpresto".to_string(),
            0x3C => "Entertainment Interactive".to_string(),
            0x3E => "Gremlin".to_string(),
            0x41 => "Ubi Soft".to_string(),
            0x42 => "Atlus".to_string(),
            0x44 => "Malibu Interactive".to_string(),
            0x46 => "Angel".to_string(),
            0x47 => "Spectrum HoloByte".to_string(),
            0x49 => "Irem".to_string(),
            0x4A => "Virgin Games Ltd.".to_string(),
            0x4D => "Malibu Interactive".to_string(),
            0x4F => "U.S. Gold".to_string(),
            0x50 => "Absolute".to_string(),
            0x51 => "Acclaim Entertainment".to_string(),
            0x52 => "Activision".to_string(),
            0x53 => "Sammy USA Corporation".to_string(),
            0x54 => "GameTek".to_string(),
            0x55 => "Park Place".to_string(),
            0x56 => "LJN".to_string(),
            0x57 => "Matchbox".to_string(),
            0x59 => "Milton Bradley Company".to_string(),
            0x5A => "Mindscape".to_string(),
            0x5B => "Romstar".to_string(),
            0x5C => "Naxat Soft".to_string(),
            0x5D => "Tradewest".to_string(),
            0x60 => "Titus Interactive".to_string(),
            0x61 => "Virgin Games Ltd.".to_string(),
            0x67 => "Ocean Software".to_string(),
            0x69 => "EA (Electronic Arts)".to_string(),
            0x6E => "Elite Systems".to_string(),
            0x6F => "Electro Brain".to_string(),
            0x70 => "Infogrames".to_string(),
            0x71 => "Interplay Entertainment".to_string(),
            0x72 => "Broderbund".to_string(),
            0x73 => "Sculptured Software".to_string(),
            0x75 => "The Sales Curve Limited".to_string(),
            0x78 => "THQ".to_string(),
            0x79 => "Accolade".to_string(),
            0x7A => "Triffix Entertainment".to_string(),
            0x7C => "MicroProse".to_string(),
            0x7F => "Kemco".to_string(),
            0x80 => "Misawa Entertainment".to_string(),
            0x83 => "LOZC G.".to_string(),
            0x86 => "Tokuma Shoten".to_string(),
            0x8B => "Bullet-Proof Software".to_string(),
            0x8C => "Vic Tokai Corp.".to_string(),
            0x8E => "Ape Inc.".to_string(),
            0x8F => "I'Max".to_string(),
            0x91 => "Chunsoft Co.".to_string(),
            0x92 => "Video System".to_string(),
            0x93 => "Tsubaraya Productions".to_string(),
            0x95 => "Varie".to_string(),
            0x96 => "Yonezawa/S'Pal".to_string(),
            0x97 => "Kemco".to_string(),
            0x99 => "Arc".to_string(),
            0x9A => "Nihon Bussan".to_string(),
            0x9B => "Tecmo".to_string(),
            0x9C => "Imagineer".to_string(),
            0x9D => "Banpresto".to_string(),
            0x9F => "Nova".to_string(),
            0xA1 => "Hori Electric".to_string(),
            0xA2 => "Bandai".to_string(),
            0xA4 => "Konami".to_string(),
            0xA6 => "Kawada".to_string(),
            0xA7 => "Takara".to_string(),
            0xA9 => "Technos Japan".to_string(),
            0xAA => "Broderbund".to_string(),
            0xAC => "Toei Animation".to_string(),
            0xAD => "Toho".to_string(),
            0xAF => "Namco".to_string(),
            0xB0 => "Acclaim Entertainment".to_string(),
            0xB1 => "ASCII Corporation or Nexsoft".to_string(),
            0xB2 => "Bandai".to_string(),
            0xB4 => "Square Enix".to_string(),
            0xB6 => "HAL Laboratory".to_string(),
            0xB7 => "SNK".to_string(),
            0xB9 => "Pony Canyon".to_string(),
            0xBA => "Culture Brain".to_string(),
            0xBB => "Sunsoft".to_string(),
            0xBD => "Sony Imagesoft".to_string(),
            0xBF => "Sammy Corporation".to_string(),
            0xC0 => "Taito".to_string(),
            0xC2 => "Kemco".to_string(),
            0xC3 => "Square".to_string(),
            0xC4 => "Tokuma Shoten".to_string(),
            0xC5 => "Data East".to_string(),
            0xC6 => "Tonkin House".to_string(),
            0xC8 => "Koei".to_string(),
            0xC9 => "UFL".to_string(),
            0xCA => "Ultra Games".to_string(),
            0xCB => "VAP, Inc.".to_string(),
            0xCC => "Use Corporation".to_string(),
            0xCD => "Meldac".to_string(),
            0xCE => "Pony Canyon".to_string(),
            0xCF => "Angel".to_string(),
            0xD0 => "Taito".to_string(),
            0xD1 => "SOFEL".to_string(),
            0xD2 => "Quest".to_string(),
            0xD3 => "Sigma Enterprises".to_string(),
            0xD4 => "ASK Kodansha Co.".to_string(),
            0xD6 => "Naxat Soft".to_string(),
            0xD7 => "Copya System".to_string(),
            0xD9 => "Banpresto".to_string(),
            0xDA => "Tomy".to_string(),
            0xDB => "LJN".to_string(),
            0xDD => "Nippon Computer Systems".to_string(),
            0xDE => "Human Ent.".to_string(),
            0xDF => "Altron".to_string(),
            0xE0 => "Jaleco".to_string(),
            0xE1 => "Towa Chiki".to_string(),
            0xE2 => "Yutaka".to_string(),
            0xE3 => "Varie".to_string(),
            0xE5 => "Epoch".to_string(),
            0xE7 => "Athena".to_string(),
            0xE8 => "Asmik Ace Entertainment".to_string(),
            0xE9 => "Natsume".to_string(),
            0xEA => "King Records".to_string(),
            0xEB => "Atlus".to_string(),
            0xEC => "Epic/Sony Records".to_string(),
            0xEE => "IGS".to_string(),
            0xF0 => "A Wave".to_string(),
            0xF3 => "Extreme Entertainment".to_string(),
            0xFF => "LJN".to_string(),
            _ => format!("Unknown Old Licensee Code (0x{:02X})", old_licensee),
        }
    }

    fn parse_new_licensee(new_licensee: [u8; 2]) -> String {
        let code = Self::parse_ascii(&new_licensee);
        match code.as_str() {
            "00" => "None".to_string(),
            "01" => "Nintendo Research & Development 1".to_string(),
            "08" => "Capcom".to_string(),
            "13" => "EA (Electronic Arts)".to_string(),
            "18" => "Hudson Soft".to_string(),
            "19" => "B-AI".to_string(),
            "20" => "KSS".to_string(),
            "22" => "Planning Office WADA".to_string(),
            "24" => "PCM Complete".to_string(),
            "25" => "San-X".to_string(),
            "28" => "Kemco".to_string(),
            "29" => "SETA Corporation".to_string(),
            "30" => "Viacom".to_string(),
            "31" => "Nintendo".to_string(),
            "32" => "Bandai".to_string(),
            "33" => "Ocean Software/Acclaim Entertainment".to_string(),
            "34" => "Konami".to_string(),
            "35" => "HectorSoft".to_string(),
            "37" => "Taito".to_string(),
            "38" => "Hudson Soft".to_string(),
            "39" => "Banpresto".to_string(),
            "41" => "Ubi Soft".to_string(),
            "42" => "Atlus".to_string(),
            "44" => "Malibu Interactive".to_string(),
            "46" => "Angel".to_string(),
            "47" => "Bullet-Proof Software".to_string(),
            "49" => "Irem".to_string(),
            "50" => "Absolute".to_string(),
            "51" => "Acclaim Entertainment".to_string(),
            "52" => "Activision".to_string(),
            "53" => "Sammy USA Corporation".to_string(),
            "54" => "Konami".to_string(),
            "55" => "Hi Tech Expressions".to_string(),
            "56" => "LJN".to_string(),
            "57" => "Matchbox".to_string(),
            "58" => "Mattel".to_string(),
            "59" => "Milton Bradley Company".to_string(),
            "60" => "Titus Interactive".to_string(),
            "61" => "Virgin Games Ltd.".to_string(),
            "64" => "Lucasfilm Games".to_string(),
            "67" => "Ocean Software".to_string(),
            "69" => "EA (Electronic Arts)".to_string(),
            "70" => "Infogrames".to_string(),
            "71" => "Interplay Entertainment".to_string(),
            "72" => "Broderbund".to_string(),
            "73" => "Sculptured Software".to_string(),
            "75" => "The Sales Curve Limited".to_string(),
            "78" => "THQ".to_string(),
            "79" => "Accolade".to_string(),
            "80" => "Misawa Entertainment".to_string(),
            "83" => "lozc".to_string(),
            "86" => "Tokuma Shoten".to_string(),
            "87" => "Tsukuda Original".to_string(),
            "91" => "Chunsoft Co.".to_string(),
            "92" => "Video System".to_string(),
            "93" => "Ocean Software/Acclaim Entertainment".to_string(),
            "95" => "Varie".to_string(),
            "96" => "Yonezawa/s'pal".to_string(),
            "97" => "Kaneko".to_string(),
            "99" => "Pack-In-Video".to_string(),
            "9H" => "Bottom Up".to_string(),
            "A4" => "Konami (Yu-Gi-Oh!)".to_string(),
            "BL" => "MTO".to_string(),
            "DK" => "Kodansha".to_string(),
            _ => format!("Unknown New Licensee Code ({})", code),
        }
    }

    fn parse_global_checksum(data: [u8; 2]) -> u16 {
        construct_u16(data[1], data[0])
    }
}
