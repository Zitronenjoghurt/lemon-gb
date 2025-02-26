/// https://gbdev.io/pandocs/LCDC.html#lcd-control
#[derive(Debug, Clone, PartialEq)]
pub struct LCDControl {
    /// Controls whether the LCD is on and the PPU is active.
    pub lcd_ppu_enabled: bool,
    /// Controls which background map the WINDOW uses for rendering. When it’s clear (0), the $9800 tilemap is used, otherwise it’s the $9C00 one.
    pub window_tilemap: bool,
    /// Controls whether the window shall be displayed or not. This bit is overridden on DMG by bit 0 (bg_window_enable) if that bit is clear.
    pub window_enable: bool,
    /// Controls which addressing mode the BG and Window use to pick tiles.
    /// https://gbdev.io/pandocs/Tile_Data.html#vram-tile-data
    pub bg_window_tiles: bool,
    /// Controls which background map the BACKGROUND uses for rendering. When it’s clear (0), the $9800 tilemap is used, otherwise it’s the $9C00 one.
    pub bg_tilemap: bool,
    /// Controls the size of all objects (1 tile or 2 stacked vertically)
    pub obj_size: bool,
    /// Controls whether objects are displayed or not.
    pub obj_enable: bool,
    /// Controls whether the background and window shall be displayed or not. If false, it overrides bit 5 (window_enable).
    pub bg_window_enable: bool,
}

impl From<u8> for LCDControl {
    fn from(value: u8) -> Self {
        let bg_window_enable = (value & 0b0000_0001) != 0;
        Self {
            lcd_ppu_enabled: (value & 0b1000_0000) != 0,
            window_tilemap: (value & 0b0100_0000) != 0,
            window_enable: ((value & 0b0010_0000) != 0) && bg_window_enable,
            bg_window_tiles: (value & 0b0001_0000) != 0,
            bg_tilemap: (value & 0b0000_1000) != 0,
            obj_size: (value & 0b0000_0100) != 0,
            obj_enable: (value & 0b0000_0010) != 0,
            bg_window_enable,
        }
    }
}
