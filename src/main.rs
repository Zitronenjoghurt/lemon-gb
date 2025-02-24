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

fn main() {
    env_logger::Builder::new()
        .filter_level(LevelFilter::Debug)
        .init();

    let path = PathBuf::from("./roms/Super Mario Land.gb");
    let cartridge = Cartridge::load(path).unwrap();

    let mut game_boy = GameBoy::initialize(&cartridge);
    game_boy.step();
    game_boy.step();
    game_boy.step();
    game_boy.step();
    game_boy.step();

    let state = game_boy.save();
    state
        .store_binary(&PathBuf::from("./test/test.bin"))
        .unwrap();
}
