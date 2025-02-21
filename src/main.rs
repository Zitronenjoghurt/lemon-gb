use crate::game_boy::components::cartridge::Cartridge;
use std::path::PathBuf;

pub mod enums;
pub mod game_boy;
mod helpers;
pub mod instructions;
#[cfg(test)]
mod tests;

fn main() {
    let path = PathBuf::from("./roms/Pokemon Blue.gb");
    let cartridge = Cartridge::load(path).unwrap();
    println!("{:#?}", cartridge.header);
}
