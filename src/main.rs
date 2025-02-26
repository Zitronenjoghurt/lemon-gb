use crate::game_boy::components::cartridge::Cartridge;
use crate::game_boy::GameBoy;
use log::LevelFilter;
use std::path::PathBuf;

pub mod enums;
pub mod game_boy;
mod helpers;
pub mod instructions;
#[cfg(test)]
mod tests;

const WINDOW_SCALE_FACTOR: u32 = 3;

fn main() {
    env_logger::Builder::new()
        .filter_level(LevelFilter::Debug)
        .init();

    let path = PathBuf::from("./roms/Dr. Mario.gb");
    let cartridge = Cartridge::load(path).unwrap();
    let mut game_boy = GameBoy::initialize(&cartridge);

    //let event_loop = EventLoop::new().unwrap();

    //let window_size = LogicalSize::new(
    //    SCREEN_WIDTH as f64 * WINDOW_SCALE_FACTOR as f64,
    //    SCREEN_HEIGHT as f64 * WINDOW_SCALE_FACTOR as f64,
    //);
    //let window_attributes = WindowAttributes::default()
    //    .with_title("Game Boy Emulator")
    //    .with_inner_size(window_size)
    //    .with_min_inner_size(window_size);
    //let window = event_loop.create_window(window_attributes).unwrap();

    //let mut pixels = {
    //    let window_size = window.inner_size();
    //    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    //    Pixels::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32, surface_texture)
    //        .expect("Failed to create pixel buffer")
    //};

    let state = game_boy.save();
    state
        .store_json(&PathBuf::from("./test/test.json"))
        .unwrap();
}
