use crate::game_boy::components::mmu::mbc::mbc1::Mbc1;
use crate::game_boy::components::mmu::mbc::Mbc;

#[test]
fn test_mbc1_initial_state() {
    let mbc1 = Mbc::Mbc1(Mbc1::initialize(false));
    assert_eq!(mbc1.get_lower_rom_index(), 0);
    assert_eq!(mbc1.get_upper_rom_index(), 1);
    assert_eq!(mbc1.get_ram_index(), 0);
    assert!(!mbc1.ram_enabled());
}

#[test]
fn test_mbc1_ram_enable() {
    let mut mbc1 = Mbc::Mbc1(Mbc1::initialize(false));

    // RAM should be disabled by default
    assert!(!mbc1.ram_enabled());

    // Writing 0x0A to 0x0000-0x1FFF enables RAM
    mbc1.handle_write(0x0000, 0x0A);
    assert!(mbc1.ram_enabled());

    // Any other value should disable RAM
    mbc1.handle_write(0x0000, 0x00);
    assert!(!mbc1.ram_enabled());
}

#[test]
fn test_mbc1_rom_bank_switching() {
    let mut mbc1 = Mbc::Mbc1(Mbc1::initialize(false));

    // Test ROM bank selection (0x2000-0x3FFF)
    // Writing 0 is treated as 1
    mbc1.handle_write(0x2000, 0x00);
    assert_eq!(mbc1.get_upper_rom_index(), 1);

    // Test various bank numbers
    mbc1.handle_write(0x2000, 0x1F); // Max 5-bit value
    assert_eq!(mbc1.get_upper_rom_index(), 0x1F);

    // Test that values get masked properly
    mbc1.handle_write(0x2000, 0xFF);
    assert_eq!(mbc1.get_upper_rom_index(), 0x1F); // Should mask to 5 bits
}

#[test]
fn test_mbc1_ram_bank_mode_selection() {
    let mut mbc1 = Mbc::Mbc1(Mbc1::initialize(false));

    // Test RAM banking mode selection (0x6000-0x7FFF)
    // Default is ROM banking mode (0)
    assert_eq!(mbc1.get_ram_index(), 0);

    // Switch to RAM banking mode (1)
    mbc1.handle_write(0x6000, 0x01);
    mbc1.handle_write(0x4000, 0x03); // Set RAM bank to 3
    assert_eq!(mbc1.get_ram_index(), 3);

    // Switch back to ROM banking mode
    mbc1.handle_write(0x6000, 0x00);
    assert_eq!(mbc1.get_ram_index(), 0);
}

#[test]
fn test_mbc1_upper_bits_banking() {
    let mut mbc1 = Mbc::Mbc1(Mbc1::initialize(false));

    // Test upper bits (bank2) selection (0x4000-0x5FFF)
    mbc1.handle_write(0x4000, 0x03); // Set upper bits
    mbc1.handle_write(0x2000, 0x1F); // Set lower bits

    // In ROM mode, this affects the upper ROM bank bits
    assert_eq!(mbc1.get_upper_rom_index(), 0x7F); // Combined value
}

#[test]
fn test_mbc1_multicart() {
    let mut mbc1 = Mbc::Mbc1(Mbc1::initialize(true));

    // Test multicart ROM banking behavior
    mbc1.handle_write(0x2000, 0x0F); // Set lower bits
    mbc1.handle_write(0x4000, 0x03); // Set upper bits

    // In multicart mode, upper bits are shifted differently
    assert_eq!(mbc1.get_upper_rom_index(), 0x3F); // Combined value for multicart

    // Test banking mode switching in multicart
    mbc1.handle_write(0x6000, 0x01); // Switch to RAM banking mode
    assert_eq!(mbc1.get_lower_rom_index(), 0x30); // Upper bits affect lower bank
}

#[test]
fn test_mbc1_invalid_writes() {
    let mut mbc1 = Mbc::Mbc1(Mbc1::initialize(false));

    // Writing to invalid addresses should have no effect
    let original_state = mbc1.clone();
    mbc1.handle_write(0x8000, 0xFF);
    assert_eq!(mbc1, original_state);
}
