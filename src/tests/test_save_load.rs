use crate::game_boy::components::cartridge::Cartridge;
use crate::game_boy::save_state::GameBoySaveState;
use crate::game_boy::GameBoy;
use std::path::PathBuf;

#[test]
fn test_save_load() {
    let test_rom_path = PathBuf::from("./test_roms/01-special.gb");
    let save_path_json = PathBuf::from("./test/test.json");
    let save_path_bin = PathBuf::from("./test/test.bin");
    let cartridge = Cartridge::load(test_rom_path).unwrap();

    let mut game_boy = GameBoy::initialize(&cartridge);
    game_boy.step();
    game_boy.step();

    let save_state = game_boy.save();
    save_state.store_json(&save_path_json).unwrap();
    save_state.store_binary(&save_path_bin).unwrap();

    let save_state_json = GameBoySaveState::load_json(&save_path_json).unwrap();
    let save_state_bin = GameBoySaveState::load_binary(&save_path_bin).unwrap();

    let game_boy_json = GameBoy::load(save_state_json, &cartridge).unwrap();
    let game_boy_bin = GameBoy::load(save_state_bin, &cartridge).unwrap();

    assert_eq!(game_boy_json, game_boy_bin);
    assert_eq!(game_boy, game_boy_bin);
}
