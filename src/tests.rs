use std::fs::{create_dir, remove_dir_all};
use std::path::PathBuf;

mod test_cpu_registers;
mod test_instructions;
mod test_interrupts;
mod test_mbc;
mod test_save_load;
mod test_timer;

pub fn setup_test_dir() {
    let test_dir = PathBuf::from("./test");
    remove_dir_all(&test_dir).unwrap();
    create_dir(&test_dir).unwrap();
}
