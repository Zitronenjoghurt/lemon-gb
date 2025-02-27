/// This will determine which colors tiles with a certain color ID have
/// https://gbdev.io/pandocs/Palettes.html?highlight=bgp#ff47--bgp-non-cgb-mode-only-bg-palette-data
#[derive(Debug, Clone, PartialEq)]
pub struct BackgroundPalette {
    pub id_0: u8,
    pub id_1: u8,
    pub id_2: u8,
    pub id_3: u8,
}

impl BackgroundPalette {
    pub fn get_color_by_id(&self, id: u8) -> u8 {
        let id = id & 0b0000_0011;
        match id {
            0 => self.id_0,
            1 => self.id_1,
            2 => self.id_2,
            3 => self.id_3,
            _ => unreachable!(),
        }
    }
}

impl From<u8> for BackgroundPalette {
    fn from(value: u8) -> Self {
        Self {
            id_0: value & 0b0000_0011,
            id_1: (value & 0b0000_1100) >> 2,
            id_2: (value & 0b0011_0000) >> 4,
            id_3: (value & 0b1100_0000) >> 6,
        }
    }
}
