use crate::game_boy::components::cpu::registers::builder::CPURegistersBuilderTrait;
use crate::game_boy::components::cpu::registers::CpuRegistersAccessTrait;
use crate::game_boy::components::cpu::CPU;
use crate::game_boy::components::mmu::MMU;
use crate::helpers::bit_operations::construct_u16;
use rstest::rstest;

/// ADD register (B, C, D, E, H, L)
#[rstest]
// Tests for ADD B (0x80)
#[case::b_simple_add(0x80, 10, 20, 30, false, false, false, false)]
#[case::b_zero_result(0x80, 0, 0, 0, true, false, false, false)]
#[case::b_complement_to_zero(0x80, 255, 1, 0, true, false, true, true)]
#[case::b_half_carry(0x80, 15, 1, 16, false, false, true, false)]
#[case::b_half_carry_complex(0x80, 0x0F, 0x01, 0x10, false, false, true, false)]
#[case::b_both_carries(0x80, 0xFF, 0x01, 0x00, true, false, true, true)]
#[case::b_sprite_position(0x80, 128, 16, 144, false, false, false, false)]
#[case::b_max_no_overflow(0x80, 127, 1, 128, false, false, true, false)]
#[case::b_max_with_overflow(0x80, 255, 255, 254, false, false, true, true)]
#[case::b_identity_operation(0x80, 42, 0, 42, false, false, false, false)]
#[case::b_half_carry_edge(0x80, 0x0F, 0x01, 0x10, false, false, true, false)]
#[case::b_half_carry_chain(0x80, 0x1F, 0x01, 0x20, false, false, true, false)]
// Tests for ADD C (0x81)
#[case::c_simple_add(0x81, 10, 20, 30, false, false, false, false)]
#[case::c_zero_result(0x81, 0, 0, 0, true, false, false, false)]
#[case::c_complement_to_zero(0x81, 255, 1, 0, true, false, true, true)]
#[case::c_half_carry(0x81, 15, 1, 16, false, false, true, false)]
#[case::c_half_carry_complex(0x81, 0x0F, 0x01, 0x10, false, false, true, false)]
#[case::c_both_carries(0x81, 0xFF, 0x01, 0x00, true, false, true, true)]
#[case::c_sprite_position(0x81, 128, 16, 144, false, false, false, false)]
#[case::c_max_no_overflow(0x81, 127, 1, 128, false, false, true, false)]
#[case::c_max_with_overflow(0x81, 255, 255, 254, false, false, true, true)]
#[case::c_identity_operation(0x81, 42, 0, 42, false, false, false, false)]
#[case::c_half_carry_edge(0x81, 0x0F, 0x01, 0x10, false, false, true, false)]
#[case::c_half_carry_chain(0x81, 0x1F, 0x01, 0x20, false, false, true, false)]
// Tests for ADD D (0x82)
#[case::d_simple_add(0x82, 10, 20, 30, false, false, false, false)]
#[case::d_zero_result(0x82, 0, 0, 0, true, false, false, false)]
#[case::d_complement_to_zero(0x82, 255, 1, 0, true, false, true, true)]
#[case::d_half_carry(0x82, 15, 1, 16, false, false, true, false)]
#[case::d_half_carry_complex(0x82, 0x0F, 0x01, 0x10, false, false, true, false)]
#[case::d_both_carries(0x82, 0xFF, 0x01, 0x00, true, false, true, true)]
#[case::d_sprite_position(0x82, 128, 16, 144, false, false, false, false)]
#[case::d_max_no_overflow(0x82, 127, 1, 128, false, false, true, false)]
#[case::d_max_with_overflow(0x82, 255, 255, 254, false, false, true, true)]
#[case::d_identity_operation(0x82, 42, 0, 42, false, false, false, false)]
#[case::d_half_carry_edge(0x82, 0x0F, 0x01, 0x10, false, false, true, false)]
#[case::d_half_carry_chain(0x82, 0x1F, 0x01, 0x20, false, false, true, false)]
// Tests for ADD E (0x83)
#[case::e_simple_add(0x83, 10, 20, 30, false, false, false, false)]
#[case::e_zero_result(0x83, 0, 0, 0, true, false, false, false)]
#[case::e_complement_to_zero(0x83, 255, 1, 0, true, false, true, true)]
#[case::e_half_carry(0x83, 15, 1, 16, false, false, true, false)]
#[case::e_half_carry_complex(0x83, 0x0F, 0x01, 0x10, false, false, true, false)]
#[case::e_both_carries(0x83, 0xFF, 0x01, 0x00, true, false, true, true)]
#[case::e_sprite_position(0x83, 128, 16, 144, false, false, false, false)]
#[case::e_max_no_overflow(0x83, 127, 1, 128, false, false, true, false)]
#[case::e_max_with_overflow(0x83, 255, 255, 254, false, false, true, true)]
#[case::e_identity_operation(0x83, 42, 0, 42, false, false, false, false)]
#[case::e_half_carry_edge(0x83, 0x0F, 0x01, 0x10, false, false, true, false)]
#[case::e_half_carry_chain(0x83, 0x1F, 0x01, 0x20, false, false, true, false)]
// Tests for ADD H (0x84)
#[case::h_simple_add(0x84, 10, 20, 30, false, false, false, false)]
#[case::h_zero_result(0x84, 0, 0, 0, true, false, false, false)]
#[case::h_complement_to_zero(0x84, 255, 1, 0, true, false, true, true)]
#[case::h_half_carry(0x84, 15, 1, 16, false, false, true, false)]
#[case::h_half_carry_complex(0x84, 0x0F, 0x01, 0x10, false, false, true, false)]
#[case::h_both_carries(0x84, 0xFF, 0x01, 0x00, true, false, true, true)]
#[case::h_sprite_position(0x84, 128, 16, 144, false, false, false, false)]
#[case::h_max_no_overflow(0x84, 127, 1, 128, false, false, true, false)]
#[case::h_max_with_overflow(0x84, 255, 255, 254, false, false, true, true)]
#[case::h_identity_operation(0x84, 42, 0, 42, false, false, false, false)]
#[case::h_half_carry_edge(0x84, 0x0F, 0x01, 0x10, false, false, true, false)]
#[case::h_half_carry_chain(0x84, 0x1F, 0x01, 0x20, false, false, true, false)]
// Tests for ADD L (0x85)
#[case::l_simple_add(0x85, 10, 20, 30, false, false, false, false)]
#[case::l_zero_result(0x85, 0, 0, 0, true, false, false, false)]
#[case::l_complement_to_zero(0x85, 255, 1, 0, true, false, true, true)]
#[case::l_half_carry(0x85, 15, 1, 16, false, false, true, false)]
#[case::l_half_carry_complex(0x85, 0x0F, 0x01, 0x10, false, false, true, false)]
#[case::l_both_carries(0x85, 0xFF, 0x01, 0x00, true, false, true, true)]
#[case::l_sprite_position(0x85, 128, 16, 144, false, false, false, false)]
#[case::l_max_no_overflow(0x85, 127, 1, 128, false, false, true, false)]
#[case::l_max_with_overflow(0x85, 255, 255, 254, false, false, true, true)]
#[case::l_identity_operation(0x85, 42, 0, 42, false, false, false, false)]
#[case::l_half_carry_edge(0x85, 0x0F, 0x01, 0x10, false, false, true, false)]
#[case::l_half_carry_chain(0x85, 0x1F, 0x01, 0x20, false, false, true, false)]
fn test_add_register(
    #[case] opcode: u8,
    #[case] a: u8,
    #[case] value: u8,
    #[case] expected_a: u8,
    #[case] expected_z: bool,
    #[case] expected_s: bool,
    #[case] expected_hc: bool,
    #[case] expected_c: bool,
) {
    let mut mmu = MMU::builder().rom(0, opcode).build();
    let mut cpu = CPU::builder()
        .a(a)
        .b(if opcode == 0x80 { value } else { 0 })
        .c(if opcode == 0x81 { value } else { 0 })
        .d(if opcode == 0x82 { value } else { 0 })
        .e(if opcode == 0x83 { value } else { 0 })
        .h(if opcode == 0x84 { value } else { 0 })
        .l(if opcode == 0x85 { value } else { 0 })
        .build();

    let m = cpu.step(&mut mmu);

    assert_eq!(cpu.get_registers().get_a(), expected_a);
    assert_eq!(cpu.get_registers().get_f_zero(), expected_z);
    assert_eq!(cpu.get_registers().get_f_subtract(), expected_s);
    assert_eq!(cpu.get_registers().get_f_half_carry(), expected_hc);
    assert_eq!(cpu.get_registers().get_f_carry(), expected_c);
    assert_eq!(cpu.get_pc(), 1);
    assert_eq!(m, 2);
}

/// ADD HL (0x86)
#[rstest]
#[case::simple_add(10, 20, 30, false, false, false, false)]
#[case::zero_result(0, 0, 0, true, false, false, false)]
#[case::complement_to_zero(255, 1, 0, true, false, true, true)]
#[case::half_carry(15, 1, 16, false, false, true, false)]
#[case::half_carry_complex(0x0F, 0x01, 0x10, false, false, true, false)]
#[case::both_carries(0xFF, 0x01, 0x00, true, false, true, true)]
#[case::sprite_position(128, 16, 144, false, false, false, false)]
#[case::max_no_overflow(127, 1, 128, false, false, true, false)]
#[case::max_with_overflow(255, 255, 254, false, false, true, true)]
#[case::identity_operation(42, 0, 42, false, false, false, false)]
#[case::half_carry_edge(0x0F, 0x01, 0x10, false, false, true, false)]
#[case::half_carry_chain(0x1F, 0x01, 0x20, false, false, true, false)]
fn test_add_hl(
    #[case] a: u8,
    #[case] value: u8,
    #[case] expected_a: u8,
    #[case] expected_z: bool,
    #[case] expected_s: bool,
    #[case] expected_hc: bool,
    #[case] expected_c: bool,
) {
    const ADDRESS: u16 = 0xAB;

    let mut mmu = MMU::builder().rom(0, 0x86).rom(ADDRESS, value).build();
    let mut cpu = CPU::builder().a(a).hl(ADDRESS).build();
    let m = cpu.step(&mut mmu);

    assert_eq!(cpu.get_registers().get_a(), expected_a);
    assert_eq!(cpu.get_registers().get_f_zero(), expected_z);
    assert_eq!(cpu.get_registers().get_f_subtract(), expected_s);
    assert_eq!(cpu.get_registers().get_f_half_carry(), expected_hc);
    assert_eq!(cpu.get_registers().get_f_carry(), expected_c);
    assert_eq!(cpu.get_pc(), 1);
    assert_eq!(m, 3);
}

/// ADD A (0x87)
#[rstest]
#[case::simple_add(15, 30, false, false, true, false)]
#[case::zero_result(0, 0, true, false, false, false)]
#[case::overflow(128, 0, true, false, false, true)]
#[case::half_carry(8, 16, false, false, true, false)]
#[case::half_carry_complex(8, 16, false, false, true, false)]
#[case::carry_only(192, 128, false, false, false, true)]
#[case::both_carries(143, 30, false, false, true, true)]
#[case::max_value(255, 254, false, false, true, true)]
fn test_add_a(
    #[case] a: u8,
    #[case] expected_a: u8,
    #[case] expected_z: bool,
    #[case] expected_s: bool,
    #[case] expected_hc: bool,
    #[case] expected_c: bool,
) {
    let mut mmu = MMU::builder().rom(0, 0x87).build();
    let mut cpu = CPU::builder().a(a).build();
    let m = cpu.step(&mut mmu);

    assert_eq!(cpu.get_registers().get_a(), expected_a);
    assert_eq!(cpu.get_registers().get_f_zero(), expected_z);
    assert_eq!(cpu.get_registers().get_f_subtract(), expected_s);
    assert_eq!(cpu.get_registers().get_f_half_carry(), expected_hc);
    assert_eq!(cpu.get_registers().get_f_carry(), expected_c);
    assert_eq!(cpu.get_pc(), 1);
    assert_eq!(m, 2);
}

/// ADD HL r16
#[rstest]
#[case::bc_no_carry(0x09, 0x1234, 0x2345, 0x3579, false, false)]
#[case::bc_half_carry(0x09, 0x0800, 0x0800, 0x1000, true, false)]
#[case::bc_carry(0x09, 0x8000, 0x8000, 0x0000, false, true)]
#[case::bc_both_carries(0x09, 0xFFFF, 0x0001, 0x0000, true, true)]
#[case::de_no_carry(0x19, 0x1234, 0x2345, 0x3579, false, false)]
#[case::de_half_carry(0x19, 0x0800, 0x0800, 0x1000, true, false)]
#[case::de_carry(0x19, 0x8000, 0x8000, 0x0000, false, true)]
#[case::de_both_carries(0x19, 0xFFFF, 0x0001, 0x0000, true, true)]
#[case::hl_no_carry(0x29, 0x2222, 0x2222, 0x4444, false, false)]
#[case::hl_half_carry(0x29, 0x0800, 0x0800, 0x1000, true, false)]
#[case::hl_carry(0x29, 0x8000, 0x8000, 0x0000, false, true)]
#[case::hl_both_carries(0x39, 0x8800, 0x8800, 0x1000, true, true)]
#[case::sp_no_carry(0x39, 0x1234, 0x2345, 0x3579, false, false)]
#[case::sp_half_carry(0x39, 0x0800, 0x0800, 0x1000, true, false)]
#[case::sp_carry(0x39, 0x8000, 0x8000, 0x0000, false, true)]
#[case::sp_both_carries(0x39, 0xFFFF, 0x0001, 0x0000, true, true)]
fn test_add_hl_r16(
    #[case] opcode: u8,
    #[case] value_hl: u16,
    #[case] value_r16: u16,
    #[case] expected_hl: u16,
    #[case] expected_half_carry: bool,
    #[case] expected_carry: bool,
) {
    let mut mmu = MMU::builder().rom(0, opcode).build();
    let mut cpu = CPU::builder()
        .bc(if opcode == 0x09 { value_r16 } else { 0 })
        .de(if opcode == 0x19 { value_r16 } else { 0 })
        .hl(value_hl)
        .sp(if opcode == 0x39 { value_r16 } else { 0 })
        .f_subtract(true)
        .f_zero(true)
        .build();
    let m = cpu.step(&mut mmu);

    assert_eq!(m, 2);
    assert_eq!(cpu.get_pc(), 1);
    assert_eq!(cpu.get_hl(), expected_hl);
    assert_eq!(cpu.get_f_half_carry(), expected_half_carry);
    assert_eq!(cpu.get_f_carry(), expected_carry);
    assert!(!cpu.get_f_subtract());
    assert!(cpu.get_f_zero());
}

/// CPL (0x2F)
#[test]
fn test_cpl() {
    let mut mmu = MMU::builder().rom(0, 0x2F).build();
    let mut cpu = CPU::builder().a(0b1010_1010).build();
    let m = cpu.step(&mut mmu);

    assert_eq!(m, 1);
    assert_eq!(cpu.get_pc(), 1);
    assert_eq!(cpu.get_a(), 0b0101_0101);
    assert!(cpu.get_f_subtract());
    assert!(cpu.get_f_half_carry());
    assert!(!cpu.get_f_carry());
    assert!(!cpu.get_f_zero());
}

/// CCF (0x3F)
#[test]
fn test_ccf() {
    let mut mmu = MMU::builder().rom(0, 0x3F).build();
    let mut cpu = CPU::builder()
        .f_subtract(true)
        .f_half_carry(true)
        .f_zero(true)
        .build();
    let m = cpu.step(&mut mmu);

    assert_eq!(m, 1);
    assert_eq!(cpu.get_pc(), 1);
    assert!(cpu.get_f_carry());
    assert!(!cpu.get_f_subtract());
    assert!(!cpu.get_f_half_carry());
    assert!(cpu.get_f_zero());
}

/// DAA (0x27)
#[rstest] // Also check zero flag
#[case::add_nc_nhc(0x6B, false, false, false, 0x71, false)] // 0x42 + 0x29 =(DAA)> 0x71
#[case::add_c_nhc(0x14, false, true, false, 0x74, true)] // 0x91 + 0x83 =(DAA)> 0x74
#[case::add_nc_hc(0x81, false, false, true, 0x87, false)] // 0x59 + 0x28 =(DAA)> 0x87
#[case::add_c_hc(0x32, false, true, true, 0x98, true)] // 0x99 + 0x99 =(DAA)> 0x98
#[case::sub_nc_nhc(0x20, true, false, false, 0x20, false)] // 0x42 - 0x22 =(DAA)> 0x20
#[case::sub_c_nhc(0xE1, true, true, false, 0x81, true)] // 0x53 - 0x72 =(DAA)> 0x81
#[case::sub_nc_hc(0x0E, true, false, true, 0x08, false)] // 0x35 - 0x27 =(DAA)> 0x08
#[case::sub_c_hc(0xBE, true, true, true, 0x58, true)] // 0x35 - 0x77 =(DAA)> 0x58
fn test_daa(
    #[case] value: u8,
    #[case] subtract: bool,
    #[case] carry: bool,
    #[case] half_carry: bool,
    #[case] expected_value: u8,
    #[case] expected_carry: bool,
) {
    let mut mmu = MMU::builder().rom(0, 0x27).build();
    let mut cpu = CPU::builder()
        .a(value)
        .f_subtract(subtract)
        .f_carry(carry)
        .f_half_carry(half_carry)
        .f_zero(false)
        .build();
    let m = cpu.step(&mut mmu);

    assert_eq!(m, 1);
    assert_eq!(cpu.get_pc(), 1);
    assert_eq!(cpu.get_a(), expected_value);
    assert_eq!(cpu.get_f_carry(), expected_carry);
    assert_eq!(cpu.get_f_subtract(), subtract);
    assert!(!cpu.get_f_half_carry());

    if expected_value == 0 {
        assert!(cpu.get_f_zero());
    } else {
        assert!(!cpu.get_f_zero());
    }
}

/// DEC r8 (except HL)
#[rstest]
#[case::decrement_b(0x05, 23)]
#[case::decrement_c(0x0D, 23)]
#[case::decrement_d(0x15, 23)]
#[case::decrement_e(0x1D, 23)]
#[case::decrement_h(0x25, 23)]
#[case::decrement_l(0x2D, 23)]
#[case::decrement_a(0x3D, 23)]
fn test_dec_r8(#[case] opcode: u8, #[case] value: u8) {
    let mut mmu = MMU::builder().rom(0, opcode).build();
    let mut cpu = CPU::builder()
        .b(if opcode == 0x05 { value } else { 0 })
        .c(if opcode == 0x0D { value } else { 0 })
        .d(if opcode == 0x15 { value } else { 0 })
        .e(if opcode == 0x1D { value } else { 0 })
        .h(if opcode == 0x25 { value } else { 0 })
        .l(if opcode == 0x2D { value } else { 0 })
        .a(if opcode == 0x3D { value } else { 0 })
        .build();
    let m = cpu.step(&mut mmu);

    assert_eq!(m, 1);
    assert_eq!(cpu.get_pc(), 1);

    match opcode {
        0x05 => assert_eq!(cpu.get_b(), value - 1),
        0x0D => assert_eq!(cpu.get_c(), value - 1),
        0x15 => assert_eq!(cpu.get_d(), value - 1),
        0x1D => assert_eq!(cpu.get_e(), value - 1),
        0x25 => assert_eq!(cpu.get_h(), value - 1),
        0x2D => assert_eq!(cpu.get_l(), value - 1),
        0x3D => assert_eq!(cpu.get_a(), value - 1),
        _ => panic!("Unexpected opcode"),
    }
}

/// DEC HL (0x35)
#[test]
fn test_dec_hl() {
    const ADDRESS: u16 = 0xC000;
    const VALUE: u8 = 23;

    let mut mmu = MMU::builder().rom(0, 0x35).write(ADDRESS, VALUE).build();
    let mut cpu = CPU::builder().hl(ADDRESS).build();
    let m = cpu.step(&mut mmu);

    assert_eq!(m, 3);
    assert_eq!(cpu.get_pc(), 1);
    assert_eq!(mmu.read(ADDRESS), VALUE - 1);
}

/// DEC r16
#[rstest]
#[case::decrement_bc(0x0B, 23)]
#[case::decrement_de(0x1B, 23)]
#[case::decrement_hl(0x2B, 23)]
#[case::decrement_sp(0x3B, 23)]
fn test_dec_r16(#[case] opcode: u8, #[case] value: u16) {
    let mut mmu = MMU::builder().rom(0, opcode).build();
    let mut cpu = CPU::builder()
        .bc(if opcode == 0x0B { value } else { 0 })
        .de(if opcode == 0x1B { value } else { 0 })
        .hl(if opcode == 0x2B { value } else { 0 })
        .sp(if opcode == 0x3B { value } else { 0 })
        .build();
    let m = cpu.step(&mut mmu);

    assert_eq!(m, 2);
    assert_eq!(cpu.get_pc(), 1);
    match opcode {
        0x0B => assert_eq!(cpu.get_bc(), value - 1),
        0x1B => assert_eq!(cpu.get_de(), value - 1),
        0x2B => assert_eq!(cpu.get_hl(), value - 1),
        0x3B => assert_eq!(cpu.get_sp(), value - 1),
        _ => panic!("Unexpected opcode"),
    }
}

/// DI (0xF3)
#[test]
fn test_di() {
    let mut mmu = MMU::builder().rom(0, 0xF3).build();
    let mut cpu = CPU::builder().ime(true).deferred_set_ime(true).build();
    let m = cpu.step(&mut mmu);

    assert_eq!(m, 1);
    assert_eq!(cpu.get_pc(), 1);
    assert!(!cpu.get_ime());
    assert!(!cpu.get_deferred_set_ime());
}

/// EI (0xFB)
#[test]
fn test_ei() {
    let mut mmu = MMU::builder().rom(0, 0xFB).build();
    let mut cpu = CPU::default();
    let m = cpu.step(&mut mmu);

    assert_eq!(m, 1);
    assert_eq!(cpu.get_pc(), 1);
    assert!(!cpu.get_ime());
    assert!(cpu.get_deferred_set_ime());

    cpu.step(&mut mmu);
    assert!(cpu.get_ime());
    assert!(!cpu.get_deferred_set_ime());
}

/// INC r8 (except HL)
#[rstest]
#[case::increment_b(0x04, 23)]
#[case::increment_c(0x0C, 23)]
#[case::increment_d(0x14, 23)]
#[case::increment_e(0x1C, 23)]
#[case::increment_h(0x24, 23)]
#[case::increment_l(0x2C, 23)]
#[case::increment_a(0x3C, 23)]
fn test_inc_r8(#[case] opcode: u8, #[case] value: u8) {
    let mut mmu = MMU::builder().rom(0, opcode).build();
    let mut cpu = CPU::builder()
        .b(if opcode == 0x04 { value } else { 0 })
        .c(if opcode == 0x0C { value } else { 0 })
        .d(if opcode == 0x14 { value } else { 0 })
        .e(if opcode == 0x1C { value } else { 0 })
        .h(if opcode == 0x24 { value } else { 0 })
        .l(if opcode == 0x2C { value } else { 0 })
        .a(if opcode == 0x3C { value } else { 0 })
        .build();
    let m = cpu.step(&mut mmu);

    assert_eq!(m, 1);
    assert_eq!(cpu.get_pc(), 1);

    match opcode {
        0x04 => assert_eq!(cpu.get_b(), value + 1),
        0x0C => assert_eq!(cpu.get_c(), value + 1),
        0x14 => assert_eq!(cpu.get_d(), value + 1),
        0x1C => assert_eq!(cpu.get_e(), value + 1),
        0x24 => assert_eq!(cpu.get_h(), value + 1),
        0x2C => assert_eq!(cpu.get_l(), value + 1),
        0x3C => assert_eq!(cpu.get_a(), value + 1),
        _ => panic!("Unexpected opcode"),
    }
}

/// INC HL (0x34)
#[test]
fn test_inc_hl() {
    const ADDRESS: u16 = 0xC000;
    const VALUE: u8 = 23;

    let mut mmu = MMU::builder().rom(0, 0x34).write(ADDRESS, VALUE).build();
    let mut cpu = CPU::builder().hl(ADDRESS).build();
    let m = cpu.step(&mut mmu);

    assert_eq!(m, 3);
    assert_eq!(cpu.get_pc(), 1);
    assert_eq!(mmu.read(ADDRESS), VALUE + 1);
}

/// INC r16
#[rstest]
#[case::decrement_bc(0x03, 23)]
#[case::decrement_de(0x13, 23)]
#[case::decrement_hl(0x23, 23)]
#[case::decrement_sp(0x33, 23)]
fn test_inc_r16(#[case] opcode: u8, #[case] value: u16) {
    let mut mmu = MMU::builder().rom(0, opcode).build();
    let mut cpu = CPU::builder()
        .bc(if opcode == 0x03 { value } else { 0 })
        .de(if opcode == 0x13 { value } else { 0 })
        .hl(if opcode == 0x23 { value } else { 0 })
        .sp(if opcode == 0x33 { value } else { 0 })
        .build();
    let m = cpu.step(&mut mmu);

    assert_eq!(m, 2);
    assert_eq!(cpu.get_pc(), 1);
    match opcode {
        0x03 => assert_eq!(cpu.get_bc(), value + 1),
        0x13 => assert_eq!(cpu.get_de(), value + 1),
        0x23 => assert_eq!(cpu.get_hl(), value + 1),
        0x33 => assert_eq!(cpu.get_sp(), value + 1),
        _ => panic!("Unexpected opcode"),
    }
}

/// NOP (0x00)
#[test]
fn test_nop() {
    let mut mmu = MMU::builder().rom(0, 0x00).build();
    let mut cpu = CPU::default();

    let m = cpu.step(&mut mmu);
    assert_eq!(cpu.get_pc(), 1);
    assert_eq!(m, 1);
}

/// Load A r16
#[rstest]
#[case::bc_load(0x0A, 0xC337, 0x5A)]
#[case::de_load(0x1A, 0xC337, 0x5A)]
#[case::hli_load(0x2A, 0xC337, 0x5A)]
#[case::hld_load(0x3A, 0xC337, 0x5A)]
fn test_ld_a_r16(#[case] opcode: u8, #[case] address: u16, #[case] value: u8) {
    let mut mmu = MMU::builder().rom(0, opcode).write(address, value).build();
    let mut cpu = CPU::builder()
        .a(value)
        .bc(if opcode == 0x0A { address } else { 0 })
        .de(if opcode == 0x1A { address } else { 0 })
        .hl(if opcode == 0x2A || opcode == 0x3A {
            address
        } else {
            0
        })
        .build();
    let m = cpu.step(&mut mmu);

    assert_eq!(m, 2);
    assert_eq!(cpu.get_pc(), 1);
    assert_eq!(cpu.get_a(), value);

    if opcode == 0x2A {
        assert_eq!(cpu.get_hl(), address + 1);
    } else if opcode == 0x3A {
        assert_eq!(cpu.get_hl(), address - 1);
    }
}

/// Load r16 A
#[rstest]
#[case::bc_load(0x02, 0xC337, 0x5A)]
#[case::de_load(0x12, 0xC337, 0x5A)]
#[case::hli_load(0x22, 0xC337, 0x5A)]
#[case::hld_load(0x32, 0xC337, 0x5A)]
fn test_ld_r16_a(#[case] opcode: u8, #[case] address: u16, #[case] value: u8) {
    let mut mmu = MMU::builder().rom(0, opcode).build();
    let mut cpu = CPU::builder()
        .a(value)
        .bc(if opcode == 0x02 { address } else { 0 })
        .de(if opcode == 0x12 { address } else { 0 })
        .hl(if opcode == 0x22 || opcode == 0x32 {
            address
        } else {
            0
        })
        .build();
    let m = cpu.step(&mut mmu);

    assert_eq!(m, 2);
    assert_eq!(cpu.get_pc(), 1);
    assert_eq!(mmu.read(address), value);

    if opcode == 0x22 {
        assert_eq!(cpu.get_hl(), address + 1);
    } else if opcode == 0x32 {
        assert_eq!(cpu.get_hl(), address - 1);
    }
}

/// LOAD imm16 SP (0x08)
#[rstest]
#[case::basic_load(0xFFFE, 0x37, 0x13, 0x00, 0xC0)]
fn test_ld_imm16_sp(
    #[case] sp: u16,
    #[case] value_lsb: u8,
    #[case] value_msb: u8,
    #[case] addr_lsb: u8,
    #[case] addr_msb: u8,
) {
    let mut mmu = MMU::builder()
        .rom(0, 0x08)
        .rom(1, addr_lsb)
        .rom(2, addr_msb)
        .write(sp, value_lsb)
        .write(sp + 1, value_msb)
        .build();
    let mut cpu = CPU::builder().sp(sp).build();
    let m = cpu.step(&mut mmu);

    assert_eq!(m, 5);
    assert_eq!(cpu.get_pc(), 3);

    let address = construct_u16(addr_lsb, addr_msb);
    assert_eq!(mmu.read(address), value_lsb);
    assert_eq!(mmu.read(address + 1), value_msb);
}

/// LOAD r8 imm8 (except HL)
#[rstest]
#[case::load_b(0x06, 0xF5)]
#[case::load_c(0x0E, 0xF5)]
#[case::load_d(0x16, 0xF5)]
#[case::load_e(0x1E, 0xF5)]
#[case::load_h(0x26, 0xF5)]
#[case::load_l(0x2E, 0xF5)]
#[case::load_a(0x3E, 0xF5)]
fn test_ld_r8_imm8(#[case] opcode: u8, #[case] value: u8) {
    let mut mmu = MMU::builder().rom(0, opcode).rom(1, value).build();
    let mut cpu = CPU::default();
    let m = cpu.step(&mut mmu);

    assert_eq!(m, 2);
    assert_eq!(cpu.get_pc(), 2);

    match opcode {
        0x06 => assert_eq!(cpu.get_b(), value),
        0x0E => assert_eq!(cpu.get_c(), value),
        0x16 => assert_eq!(cpu.get_d(), value),
        0x1E => assert_eq!(cpu.get_e(), value),
        0x26 => assert_eq!(cpu.get_h(), value),
        0x2E => assert_eq!(cpu.get_l(), value),
        0x3E => assert_eq!(cpu.get_a(), value),
        _ => panic!("Unexpected opcode"),
    }
}

/// LD HL IMM8 (0x36)
#[test]
fn test_ld_hl_imm8() {
    const ADDRESS: u16 = 0xC000;
    const VALUE: u8 = 0xF5;

    let mut mmu = MMU::builder().rom(0, 0x36).rom(1, VALUE).build();
    let mut cpu = CPU::builder().hl(ADDRESS).build();
    let m = cpu.step(&mut mmu);

    assert_eq!(m, 3);
    assert_eq!(cpu.get_pc(), 2);
    assert_eq!(mmu.read(ADDRESS), VALUE);
}

/// LOAD r16 imm16
#[rstest]
#[case::bc_load(0x01, 0x37, 0x13, 0x1337)]
#[case::de_load(0x11, 0x37, 0x13, 0x1337)]
#[case::hl_load(0x21, 0x37, 0x13, 0x1337)]
#[case::sp_load(0x31, 0x37, 0x13, 0x1337)]
fn test_ld_r16_imm16(
    #[case] opcode: u8,
    #[case] imm1: u8,
    #[case] imm2: u8,
    #[case] expected_value: u16,
) {
    let mut mmu = MMU::builder()
        .rom(0, opcode)
        .rom(1, imm1)
        .rom(2, imm2)
        .build();
    let mut cpu = CPU::default();
    let m = cpu.step(&mut mmu);

    match opcode {
        0x01 => assert_eq!(cpu.get_bc(), expected_value),
        0x11 => assert_eq!(cpu.get_de(), expected_value),
        0x21 => assert_eq!(cpu.get_hl(), expected_value),
        0x31 => assert_eq!(cpu.get_sp(), expected_value),
        _ => panic!("unexpected opcode"),
    }

    assert_eq!(cpu.get_pc(), 3);
    assert_eq!(m, 3);
}

/// JUMP imm16 (0xC3)
#[rstest]
#[case::basic_jump(0x11, 0x22, 0x2211)]
#[case::jump_to_start(0, 0, 0)]
#[case::jump_to_end(0xFF, 0xFF, 0xFFFF)]
fn test_jump_imm16(#[case] imm1: u8, #[case] imm2: u8, #[case] expected_pc: u16) {
    let mut mmu = MMU::builder()
        .rom(0, 0xC3)
        .rom(1, imm1)
        .rom(2, imm2)
        .build();
    let mut cpu = CPU::default();

    let m = cpu.step(&mut mmu);
    assert_eq!(cpu.get_pc(), expected_pc);
    assert_eq!(m, 4);
}

/// JUMP COND imm16
#[rstest]
#[case::nz_jump(0xC2, 0x34, 0x12, false, false, 0x1234, 4)]
#[case::nz_no_jump(0xC2, 0x34, 0x12, true, false, 3, 3)]
#[case::z_jump(0xCA, 0x34, 0x12, true, false, 0x1234, 4)]
#[case::z_no_jump(0xCA, 0x34, 0x12, false, false, 3, 3)]
#[case::nc_jump(0xD2, 0x34, 0x12, false, false, 0x1234, 4)]
#[case::nc_no_jump(0xD2, 0x34, 0x12, false, true, 3, 3)]
#[case::c_jump(0xDA, 0x34, 0x12, false, true, 0x1234, 4)]
#[case::c_no_jump(0xDA, 0x34, 0x12, false, false, 3, 3)]
fn test_jump_cond_imm16(
    #[case] opcode: u8,
    #[case] imm1: u8,
    #[case] imm2: u8,
    #[case] f_zero: bool,
    #[case] f_carry: bool,
    #[case] expected_pc: u16,
    #[case] expected_m: u8,
) {
    let mut mmu = MMU::builder()
        .rom(0, opcode)
        .rom(1, imm1)
        .rom(2, imm2)
        .build();
    let mut cpu = CPU::builder().f_zero(f_zero).f_carry(f_carry).build();
    let m = cpu.step(&mut mmu);

    assert_eq!(cpu.get_pc(), expected_pc);
    assert_eq!(m, expected_m);
}

/// JUMP HL (0xE9)
#[rstest]
#[case::basic_jump(0x1337)]
fn test_jump_hl(#[case] target_address: u16) {
    let mut mmu = MMU::builder().rom(0, 0xE9).build();
    let mut cpu = CPU::builder().hl(target_address).build();
    let m = cpu.step(&mut mmu);

    assert_eq!(cpu.get_pc(), target_address);
    assert_eq!(m, 1);
}

/// JR imm8 (0x18)
#[test]
fn test_jr_imm8() {
    const RELATIVE_JUMP: u8 = 28;

    let mut mmu = MMU::builder().rom(0, 0x18).rom(1, RELATIVE_JUMP).build();
    let mut cpu = CPU::default();
    let m = cpu.step(&mut mmu);

    assert_eq!(m, 3);
    assert_eq!(cpu.get_pc(), RELATIVE_JUMP as u16 + 2);
}

/// JR cond imm8
#[rstest]
#[case::nz_jump(0x20, 30, 32, 3, false, false)]
#[case::nz_no_jump(0x20, 30, 2, 2, true, false)]
#[case::z_jump(0x28, 30, 32, 3, true, false)]
#[case::z_no_jump(0x28, 30, 2, 2, false, false)]
#[case::nc_jump(0x30, 30, 32, 3, false, false)]
#[case::nc_no_jump(0x30, 30, 2, 2, false, true)]
#[case::c_jump(0x38, 30, 32, 3, false, true)]
#[case::c_no_jump(0x38, 30, 2, 2, false, false)]
fn test_jr_cond_imm8(
    #[case] opcode: u8,
    #[case] immediate: u8,
    #[case] target_pc: u16,
    #[case] target_m: u8,
    #[case] zero_flag: bool,
    #[case] carry_flag: bool,
) {
    let mut mmu = MMU::builder().rom(0, opcode).rom(1, immediate).build();
    let mut cpu = CPU::builder().f_zero(zero_flag).f_carry(carry_flag).build();
    let m = cpu.step(&mut mmu);

    assert_eq!(m, target_m);
    assert_eq!(cpu.get_pc(), target_pc);
}

/// PUSH r16
#[rstest]
#[case::bc_push_basic(0xC5, 0xFFFE, 0x1337, 0x1337)]
#[case::de_push_basic(0xD5, 0xFFFE, 0x1337, 0x1337)]
#[case::hl_push_basic(0xE5, 0xFFFE, 0x1337, 0x1337)]
// The lower 4 bits of the F register are hardwired to be read as zero, that's why this value deviates
#[case::af_push_basic(0xF5, 0xFFFE, 0x1337, 0x1330)]
fn test_push_r16(
    #[case] opcode: u8,
    #[case] sp: u16,
    #[case] push_value: u16,
    #[case] expected_value: u16,
) {
    let mut mmu = MMU::builder().rom(0, opcode).build();
    let mut cpu = CPU::builder()
        .sp(sp)
        .bc(if opcode == 0xC5 { push_value } else { 0 })
        .de(if opcode == 0xD5 { push_value } else { 0 })
        .hl(if opcode == 0xE5 { push_value } else { 0 })
        .af(if opcode == 0xF5 { push_value } else { 0 })
        .build();
    let m = cpu.step(&mut mmu);

    assert_eq!(cpu.get_sp(), sp - 2);
    assert_eq!(mmu.read_16(cpu.get_sp()), expected_value);
    assert_eq!(cpu.get_pc(), 1);
    assert_eq!(m, 4);
}

/// POP r16
#[rstest]
#[case::bc_pop_basic(0xC1, 0xFFFC, 0x37, 0x13, 0x1337)]
#[case::de_pop_basic(0xD1, 0xFFFC, 0x37, 0x13, 0x1337)]
#[case::hl_pop_basic(0xE1, 0xFFFC, 0x37, 0x13, 0x1337)]
// The lower 4 bits of the F register are hardwired to be read as zero, that's why this value deviates
#[case::af_pop_basic(0xF1, 0xFFFC, 0x37, 0x13, 0x1330)]
fn test_pop_r16(
    #[case] opcode: u8,
    #[case] sp: u16,
    #[case] imm1: u8,
    #[case] imm2: u8,
    #[case] expected_value: u16,
) {
    let mut mmu = MMU::builder()
        .rom(0, opcode)
        .write(sp, imm1)
        .write(sp + 1, imm2)
        .build();
    let mut cpu = CPU::builder().sp(sp).build();
    let m = cpu.step(&mut mmu);

    assert_eq!(cpu.get_sp(), sp + 2);
    assert_eq!(cpu.get_pc(), 1);
    assert_eq!(m, 3);

    match opcode {
        0xC1 => assert_eq!(cpu.get_bc(), expected_value),
        0xD1 => assert_eq!(cpu.get_de(), expected_value),
        0xE1 => assert_eq!(cpu.get_hl(), expected_value),
        0xF1 => assert_eq!(cpu.get_af(), expected_value),
        _ => panic!("Unexpected opcode"),
    }
}

/// RET (0xC9)
#[test]
fn test_ret() {
    const SP: u16 = 0xFFFC;
    const ADDR_LSB: u8 = 0x11;
    const ADDR_MSB: u8 = 0xCC;

    let mut mmu = MMU::builder()
        .rom(0, 0xC9)
        .write(SP, ADDR_LSB)
        .write(SP + 1, ADDR_MSB)
        .build();
    let mut cpu = CPU::builder().sp(SP).build();
    let m = cpu.step(&mut mmu);

    assert_eq!(m, 4);
    assert_eq!(cpu.get_sp(), SP + 2);
    assert_eq!(cpu.get_pc(), construct_u16(ADDR_LSB, ADDR_MSB));
}

/// RET cond
#[rstest]
#[case::jump_nz(0xC0, 0xFFFC, 0x11, 0xCC, false, false, 5, 0xFFFE, 0xCC11)]
#[case::no_jump_nz(0xC0, 0xFFFC, 0x11, 0xCC, true, false, 2, 0xFFFC, 1)]
#[case::jump_z(0xC8, 0xFFFC, 0x11, 0xCC, true, false, 5, 0xFFFE, 0xCC11)]
#[case::no_jump_z(0xC8, 0xFFFC, 0x11, 0xCC, false, false, 2, 0xFFFC, 1)]
#[case::jump_nc(0xD0, 0xFFFC, 0x11, 0xCC, false, false, 5, 0xFFFE, 0xCC11)]
#[case::no_jump_nc(0xD0, 0xFFFC, 0x11, 0xCC, false, true, 2, 0xFFFC, 1)]
#[case::jump_c(0xD8, 0xFFFC, 0x11, 0xCC, false, true, 5, 0xFFFE, 0xCC11)]
#[case::no_jump_c(0xD8, 0xFFFC, 0x11, 0xCC, false, false, 2, 0xFFFC, 1)]
fn test_ret_cond(
    #[case] opcode: u8,
    #[case] sp: u16,
    #[case] addr_lsb: u8,
    #[case] addr_msb: u8,
    #[case] zero_flag: bool,
    #[case] carry_flag: bool,
    #[case] expected_m: u8,
    #[case] expected_sp: u16,
    #[case] expected_pc: u16,
) {
    let mut mmu = MMU::builder()
        .rom(0, opcode)
        .write(sp, addr_lsb)
        .write(sp + 1, addr_msb)
        .build();
    let mut cpu = CPU::builder()
        .sp(sp)
        .f_zero(zero_flag)
        .f_carry(carry_flag)
        .build();
    let m = cpu.step(&mut mmu);

    assert_eq!(m, expected_m);
    assert_eq!(cpu.get_sp(), expected_sp);
    assert_eq!(cpu.get_pc(), expected_pc);
}

/// RETI (0xD9)
#[test]
fn test_reti() {
    const SP: u16 = 0xFFFC;
    const ADDR_LSB: u8 = 0x11;
    const ADDR_MSB: u8 = 0xCC;
    let mut mmu = MMU::builder()
        .rom(0, 0xD9)
        .write(SP, ADDR_LSB)
        .write(SP + 1, ADDR_MSB)
        .build();
    let mut cpu = CPU::builder().sp(SP).ime(false).build();
    let m = cpu.step(&mut mmu);

    assert_eq!(m, 4);
    assert_eq!(cpu.get_sp(), SP + 2);
    assert_eq!(cpu.get_pc(), construct_u16(ADDR_LSB, ADDR_MSB));
    assert!(cpu.get_ime());
}

/// RLA (0x17) & RRA (0x1F)
#[rstest]
#[case::left_nc_nc(0x17, 0b0110_0110, false, 0b1100_1100, false)]
#[case::left_c_nc(0x17, 0b0110_0110, true, 0b1100_1101, false)]
#[case::left_nc_c(0x17, 0b1110_0110, false, 0b1100_1100, true)]
#[case::left_c_c(0x17, 0b1110_0110, true, 0b1100_1101, true)]
#[case::right_nc_nc(0x1F, 0b0110_0110, false, 0b0011_0011, false)]
#[case::right_c_nc(0x1F, 0b0110_0110, true, 0b1011_0011, false)]
#[case::right_nc_c(0x1F, 0b0110_0111, false, 0b0011_0011, true)]
#[case::right_c_c(0x1F, 0b0110_0111, true, 0b1011_0011, true)]
fn test_rla_rra(
    #[case] opcode: u8,
    #[case] value: u8,
    #[case] carry: bool,
    #[case] expected_value: u8,
    #[case] expected_carry: bool,
) {
    let mut mmu = MMU::builder().rom(0, opcode).build();
    let mut cpu = CPU::builder()
        .a(value)
        .f_carry(carry)
        .f_subtract(true)
        .f_half_carry(true)
        .f_zero(true)
        .build();
    let m = cpu.step(&mut mmu);

    assert_eq!(m, 1);
    assert_eq!(cpu.get_pc(), 1);
    assert_eq!(cpu.get_a(), expected_value);
    assert_eq!(cpu.get_f_carry(), expected_carry);
    assert!(!cpu.get_f_subtract());
    assert!(!cpu.get_f_half_carry());
    assert!(!cpu.get_f_zero());
}

/// RLCA (0x07) & RRCA (0x0F)
#[rstest]
#[case::left_nc(0x07, 0b0110_0110, 0b1100_1100, false)]
#[case::left_c(0x07, 0b1110_0110, 0b1100_1101, true)]
#[case::right_nc(0x0F, 0b0110_0110, 0b0011_0011, false)]
#[case::right_c(0x0F, 0b0110_0111, 0b1011_0011, true)]
fn test_rlca_rrca(
    #[case] opcode: u8,
    #[case] value: u8,
    #[case] expected_value: u8,
    #[case] expected_carry: bool,
) {
    let mut mmu = MMU::builder().rom(0, opcode).build();
    let mut cpu = CPU::builder()
        .a(value)
        .f_subtract(true)
        .f_half_carry(true)
        .f_zero(true)
        .build();
    let m = cpu.step(&mut mmu);

    assert_eq!(m, 1);
    assert_eq!(cpu.get_pc(), 1);
    assert_eq!(cpu.get_a(), expected_value);
    assert_eq!(cpu.get_f_carry(), expected_carry);
    assert!(!cpu.get_f_subtract());
    assert!(!cpu.get_f_half_carry());
    assert!(!cpu.get_f_zero());
}

/// SCF (0x37)
#[test]
fn test_scf() {
    let mut mmu = MMU::builder().rom(0, 0x37).build();
    let mut cpu = CPU::builder()
        .f_carry(false)
        .f_subtract(true)
        .f_half_carry(true)
        .f_zero(true)
        .build();
    let m = cpu.step(&mut mmu);

    assert_eq!(m, 1);
    assert_eq!(cpu.get_pc(), 1);
    assert!(cpu.get_f_carry());
    assert!(!cpu.get_f_subtract());
    assert!(!cpu.get_f_half_carry());
    assert!(cpu.get_f_zero());
}
