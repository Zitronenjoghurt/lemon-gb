use crate::game_boy::components::cartridge::Cartridge;
use crate::game_boy::GameBoy;
use log::LevelFilter;
use std::path::PathBuf;

pub mod enums;
pub mod game_boy;
#[cfg(feature = "gui")]
mod gui;
mod helpers;
pub mod instructions;
#[cfg(test)]
mod tests;

fn main() {
    env_logger::Builder::new()
        .filter_level(LevelFilter::Error)
        .init();

    let path = PathBuf::from("./test_roms/cpu_instrs.gb");
    let cartridge = Cartridge::load(path).unwrap();
    let mut game_boy = GameBoy::initialize(&cartridge);

    #[cfg(feature = "gui")]
    gui::run(&mut game_boy);

    //
    //
    // for _ in 0..1000 {
    //
    //
    //    game_boy.step();
    //}

    //let state_json = PathBuf::from("./test/test.json");
    //game_boy.save().store_json(&state_json).unwrap();
}
