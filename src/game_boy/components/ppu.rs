use crate::game_boy::components::mmu::{
    BGP_ADDRESS, DMA_ADDRESS, LCDC_ADDRESS, LYC_ADDRESS, LY_ADDRESS, MMU, SCX_ADDRESS, SCY_ADDRESS,
    STAT_ADDRESS,
};
use crate::game_boy::components::ppu::background_palette::BackgroundPalette;
use crate::game_boy::components::ppu::lcd_control::LCDControl;
use crate::game_boy::components::ppu::lcd_status::LCDStatus;
use crate::game_boy::components::ppu::mode::PPUMode;
use image::imageops::Nearest;
use image::{imageops, ImageBuffer, Rgba};

mod background_palette;
mod lcd_control;
mod lcd_status;
mod mode;

pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;

/// Using the Game Boy Pocket color scheme
/// https://en.wikipedia.org/wiki/List_of_video_game_console_palettes
const COLOR_SCHEME: [[u8; 4]; 4] = [
    [0xC5, 0xCA, 0xA4, 0xFF],
    [0x8C, 0x92, 0x6B, 0xFF],
    [0x4A, 0x51, 0x38, 0xFF],
    [0x18, 0x18, 0x18, 0xFF],
];

#[derive(Debug, Clone, PartialEq)]
pub struct PPU {
    mode: PPUMode,
    frame_buffer: [u8; SCREEN_HEIGHT * SCREEN_WIDTH * 4],
    mode_clock: u32,
    current_line: u8,
    vblank_interrupt: bool,
    stat_interrupt: bool,
    frame_complete: bool,
}

impl PPU {
    pub fn new() -> PPU {
        PPU {
            mode: PPUMode::OAMSearch,
            frame_buffer: [0u8; SCREEN_HEIGHT * SCREEN_WIDTH * 4],
            mode_clock: 0,
            current_line: 0,
            vblank_interrupt: false,
            stat_interrupt: false,
            frame_complete: false,
        }
    }

    pub fn step(&mut self, m_cycles: u8, mmu: &mut MMU) -> (bool, bool, bool) {
        self.vblank_interrupt = false;
        self.stat_interrupt = false;
        self.frame_complete = false;

        self.handle_dma(mmu);

        self.mode_clock = self.mode_clock.wrapping_add(m_cycles as u32 * 4);
        self.execute_mode(mmu);
        self.update_memory_state(mmu);

        (
            self.vblank_interrupt,
            self.stat_interrupt,
            self.frame_complete,
        )
    }

    fn execute_mode(&mut self, mmu: &mut MMU) {
        match self.mode {
            PPUMode::OAMSearch => self.run_oam_search(),
            PPUMode::PixelTransfer => self.run_pixel_transfer(mmu),
            PPUMode::HBlank => self.run_h_blank(),
            PPUMode::VBlank => self.run_v_blank(),
        }
    }

    // ToDo: Check if timing is important, maybe handle the exact cycle length
    // https://gbdev.io/pandocs/OAM_DMA_Transfer.html#oam-dma-transfer
    fn handle_dma(&self, mmu: &mut MMU) {
        let dma = mmu.read(DMA_ADDRESS);
        if dma < 0xFF {
            // Copying from XX00-XX9F to FE00-FE9F
            let source_addr = (dma as u16) << 8;
            for i in 0..0xA0 {
                let data = mmu.read(source_addr + i);
                mmu.write(0xFE00 + i, data);
            }
        }
    }

    pub fn get_frame_buffer(&self) -> &[u8] {
        &self.frame_buffer
    }
}

/// PPU Mode functions
impl PPU {
    fn run_oam_search(&mut self) {
        if self.mode_clock >= 80 {
            self.mode_clock -= 80;
            self.mode = PPUMode::PixelTransfer;
        }
    }

    fn run_pixel_transfer(&mut self, mmu: &mut MMU) {
        if self.mode_clock >= 172 {
            self.mode_clock -= 172;
            self.mode = PPUMode::HBlank;
            self.render_line(mmu);
        }
    }

    fn run_h_blank(&mut self) {
        if self.mode_clock >= 204 {
            self.mode_clock -= 204;
            self.current_line += 1;

            if self.current_line == 144 {
                self.mode = PPUMode::VBlank;
                self.vblank_interrupt = true;
                self.frame_complete = true;
            } else {
                self.mode = PPUMode::OAMSearch;
            }
        }
    }

    fn run_v_blank(&mut self) {
        if self.mode_clock >= 456 {
            self.mode_clock -= 456;
            self.current_line += 1;
        }
        if self.current_line > 153 {
            self.mode = PPUMode::OAMSearch;
            self.current_line = 0;
        }
    }
}

/// Rendering
impl PPU {
    fn get_frame_buffer_index(&self, x: usize) -> usize {
        (self.current_line as usize * SCREEN_WIDTH + x) * 4
    }

    fn render_line(&mut self, mmu: &mut MMU) {
        if self.current_line >= 144 {
            return;
        }

        let lcdc = self.get_lcdc(mmu);

        if lcdc.bg_window_enable {
            self.render_background(mmu);
        } else {
            for x in 0..SCREEN_WIDTH {
                let index = self.get_frame_buffer_index(x);
                self.frame_buffer[index] = 255;
                self.frame_buffer[index + 1] = 255;
                self.frame_buffer[index + 2] = 255;
                self.frame_buffer[index + 3] = 255;
            }
        }
    }

    fn render_background(&mut self, mmu: &mut MMU) {
        let bg_palette = self.get_background_palette(mmu);
        let lcd_control = self.get_lcdc(mmu);
        let scroll_x = mmu.read(SCX_ADDRESS);
        let scroll_y = mmu.read(SCY_ADDRESS);

        let y_pos = (scroll_y as u16 + self.current_line as u16) & 255;
        let tile_y = y_pos / 8;

        for x in 0..SCREEN_WIDTH as u16 {
            let x_pos = (scroll_x as u16 + x) & 255;
            let tile_x = x_pos / 8;

            let tile_address = lcd_control.get_tile_address(tile_x, tile_y);
            let tile_id = mmu.read(tile_address);

            let tile_line_data_address = lcd_control.get_tile_line_data_address(tile_id, y_pos);

            let low_byte = mmu.read(tile_line_data_address);
            let high_byte = mmu.read(tile_line_data_address + 1);

            let bit_index = 7 - (x_pos % 8);
            let color_index = (((high_byte >> bit_index) & 1) << 1) | ((low_byte >> bit_index) & 1);

            let color = bg_palette.get_color_by_id(color_index);
            let buffer_index = self.get_frame_buffer_index(x as usize);

            let color_values = &COLOR_SCHEME[color as usize];
            self.frame_buffer[buffer_index..buffer_index + 4].copy_from_slice(color_values);
        }
    }
}

/// Memory Access
impl PPU {
    fn get_lcdc(&self, mmu: &MMU) -> LCDControl {
        mmu.read(LCDC_ADDRESS).into()
    }

    fn get_stat(&self, mmu: &MMU) -> LCDStatus {
        mmu.read(STAT_ADDRESS).into()
    }

    fn get_background_palette(&self, mmu: &MMU) -> BackgroundPalette {
        mmu.read(BGP_ADDRESS).into()
    }

    /// Update STAT and other important memory registers
    fn update_memory_state(&mut self, mmu: &mut MMU) {
        let mut current_stat = self.get_stat(mmu);
        current_stat.ppu_mode = self.mode;

        // Check for LYC stat interrupt
        if self.current_line == mmu.read(LYC_ADDRESS) {
            current_stat.lyc_equals_ly = true;
            if current_stat.lyc_interrupt {
                self.stat_interrupt = true;
            }
        } else {
            current_stat.lyc_equals_ly = false;
        }

        // Check for mode stat interrupts
        match self.mode {
            PPUMode::HBlank => {
                if current_stat.mode0_interrupt {
                    self.stat_interrupt = true;
                }
            }
            PPUMode::VBlank => {
                if current_stat.mode1_interrupt {
                    self.stat_interrupt = true;
                }
            }
            PPUMode::OAMSearch => {
                if current_stat.mode2_interrupt {
                    self.stat_interrupt = true;
                }
            }
            _ => {}
        }

        mmu.write(STAT_ADDRESS, current_stat.into());
        mmu.write(LY_ADDRESS, self.current_line);
    }
}

/// Miscellaneous
impl PPU {
    pub fn render_image(&self, scale_factor: f32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let image = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(
            SCREEN_WIDTH as u32,
            SCREEN_HEIGHT as u32,
            self.frame_buffer.to_vec(),
        )
        .unwrap();

        let scaled_width = (SCREEN_WIDTH as f32 * scale_factor) as u32;
        let scaled_height = (SCREEN_HEIGHT as f32 * scale_factor) as u32;

        imageops::resize(&image, scaled_width, scaled_height, Nearest)
    }
}

impl Default for PPU {
    fn default() -> Self {
        Self::new()
    }
}
