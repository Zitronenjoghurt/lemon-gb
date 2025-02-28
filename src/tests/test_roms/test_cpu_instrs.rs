use crate::tests::test_roms::{test_rom_file_path, test_run_game_boy};

const REFERENCE_FRAME_BUFFER: &[u8] =
    include_bytes!("../../../test_roms/reference_data/cpu_instrs.bin");

#[test]
fn test_cpu_instrs() {
    let rom_file_path = test_rom_file_path().join("cpu_instrs.gb");
    let game_boy = test_run_game_boy(&rom_file_path, 25_000_000);
    assert_eq!(game_boy.get_frame_buffer(), REFERENCE_FRAME_BUFFER);
}
