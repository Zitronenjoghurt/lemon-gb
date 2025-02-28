use crate::game_boy::components::cartridge::Cartridge;
use crate::game_boy::GameBoy;
use crate::tests::setup_test_dir;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

mod test_cpu_instrs;
mod test_instr_timing;

pub fn test_rom_file_path() -> PathBuf {
    PathBuf::from("./test_roms")
}

pub fn test_run_game_boy(rom_path: &Path, max_steps: u32) -> GameBoy {
    let path = PathBuf::from(rom_path);
    let cartridge = Cartridge::load(path).unwrap();
    let mut game_boy = GameBoy::initialize(&cartridge);

    for _ in 0..max_steps {
        game_boy.step();
    }

    game_boy
}

pub fn run_and_dump(rom_path: &Path, max_steps: u32, output_directory: &Path) {
    let image_dump_path = output_directory
        .join(rom_path.file_name().unwrap())
        .with_extension("png");
    let framebuffer_dump_path = output_directory
        .join(rom_path.file_name().unwrap())
        .with_extension("bin");

    let game_boy = test_run_game_boy(rom_path, max_steps);
    let frame_image = game_boy.render_image(1.0);
    frame_image.save(image_dump_path).unwrap();

    let frame_buffer = game_boy.get_frame_buffer();
    let mut file = File::create(framebuffer_dump_path).unwrap();
    file.write_all(frame_buffer).unwrap();
}

fn run_and_dump_example() {
    let rom_path = PathBuf::from("./test_roms/cpu_instrs.gb");
    let test_dir = setup_test_dir();
    run_and_dump(&rom_path, 25_000_000, &test_dir);
}
