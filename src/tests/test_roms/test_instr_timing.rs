use crate::tests::test_roms::{test_rom_file_path, test_run_game_boy};

const REFERENCE_FRAME_BUFFER: &[u8] =
    include_bytes!("../../../test_roms/reference_data/instr_timing.bin");

#[test]
fn test_instr_timing() {
    let rom_file_path = test_rom_file_path().join("instr_timing.gb");
    let game_boy = test_run_game_boy(&rom_file_path, 500_000);
    assert_eq!(game_boy.get_frame_buffer(), REFERENCE_FRAME_BUFFER);
}
