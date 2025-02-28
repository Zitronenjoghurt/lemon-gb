use std::fs::create_dir;
use std::path::PathBuf;

mod test_cpu_registers;
mod test_halt;
mod test_instructions;
mod test_interrupts;
mod test_mbc;
pub mod test_roms;
mod test_save_load;
mod test_timer;

pub fn setup_test_dir() -> PathBuf {
    let test_dir = PathBuf::from("./test");
    if !test_dir.exists() {
        create_dir(&test_dir).unwrap();
    }
    test_dir
}
