use crate::enums::parameter_groups::R8;
use crate::game_boy::components::cpu::registers::builder::CPURegistersBuilderTrait;
use crate::game_boy::components::cpu::registers::CpuRegistersAccessTrait;
use crate::game_boy::components::cpu::{CPU, PREFIX_INSTRUCTION_BYTE};
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
    assert_eq!(m, 1);
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
    assert_eq!(m, 2);
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
    assert_eq!(m, 1);
}

/// ADD imm8 (0xC6)
#[rstest]
#[case::nc_nh_nz(0x12, 0x23, 0x35, false, false, false)]
#[case::c_nh_nz(0x12, 0xF3, 0x05, true, false, false)]
#[case::nc_h_nz(0x12, 0x2F, 0x41, false, true, false)]
#[case::c_h_nz(0x12, 0xFF, 0x11, true, true, false)]
#[case::nc_nh_z(0x00, 0x00, 0x00, false, false, true)]
#[case::c_h_z(0xFF, 0x01, 0x00, true, true, true)]
fn test_add_imm8(
    #[case] value_a: u8,
    #[case] imm: u8,
    #[case] expected_value: u8,
    #[case] expected_carry: bool,
    #[case] expected_half_carry: bool,
    #[case] expected_zero: bool,
) {
    let mut mmu = MMU::builder().rom(0, 0xC6).rom(1, imm).build();
    let mut cpu = CPU::builder().a(value_a).build();
    let m = cpu.step(&mut mmu);

    assert_eq!(m, 2);
    assert_eq!(cpu.get_pc(), 2);
    assert_eq!(cpu.get_a(), expected_value);
    assert_eq!(cpu.get_f_carry(), expected_carry);
    assert_eq!(cpu.get_f_half_carry(), expected_half_carry);
    assert_eq!(cpu.get_f_zero(), expected_zero);
    assert!(!cpu.get_f_subtract());
}

/// ADC r8
#[rstest]
#[case::b_nc_nc_nh(0x88, 0x23, 0x12, R8::B, false, 0x35, false, false, false)]
#[case::b_c_nc_nh(0x88, 0x23, 0x12, R8::B, true, 0x36, false, false, false)]
#[case::b_nc_c_nh(0x88, 0xF3, 0x12, R8::B, false, 0x05, true, false, false)]
#[case::b_c_c_nh(0x88, 0xF3, 0x12, R8::B, true, 0x06, true, false, false)]
#[case::b_nc_nc_h(0x88, 0x23, 0x1F, R8::B, false, 0x42, false, true, false)]
#[case::b_c_nc_h(0x88, 0x23, 0x1F, R8::B, true, 0x43, false, true, false)]
#[case::b_nc_c_h(0x88, 0xF3, 0x1F, R8::B, false, 0x12, true, true, false)]
#[case::b_c_c_h(0x88, 0xF3, 0x1F, R8::B, true, 0x13, true, true, false)]
#[case::b_zero(0x88, 0xFE, 0x01, R8::B, true, 0x00, true, true, true)]
#[case::c_nc_nc_nh(0x89, 0x23, 0x12, R8::C, false, 0x35, false, false, false)]
#[case::c_c_nc_nh(0x89, 0x23, 0x12, R8::C, true, 0x36, false, false, false)]
#[case::c_nc_c_nh(0x89, 0xF3, 0x12, R8::C, false, 0x05, true, false, false)]
#[case::c_c_c_nh(0x89, 0xF3, 0x12, R8::C, true, 0x06, true, false, false)]
#[case::c_nc_nc_h(0x89, 0x23, 0x1F, R8::C, false, 0x42, false, true, false)]
#[case::c_c_nc_h(0x89, 0x23, 0x1F, R8::C, true, 0x43, false, true, false)]
#[case::c_nc_c_h(0x89, 0xF3, 0x1F, R8::C, false, 0x12, true, true, false)]
#[case::c_c_c_h(0x89, 0xF3, 0x1F, R8::C, true, 0x13, true, true, false)]
#[case::c_zero(0x89, 0xFE, 0x01, R8::C, true, 0x00, true, true, true)]
#[case::d_nc_nc_nh(0x8A, 0x23, 0x12, R8::D, false, 0x35, false, false, false)]
#[case::d_c_nc_nh(0x8A, 0x23, 0x12, R8::D, true, 0x36, false, false, false)]
#[case::d_nc_c_nh(0x8A, 0xF3, 0x12, R8::D, false, 0x05, true, false, false)]
#[case::d_c_c_nh(0x8A, 0xF3, 0x12, R8::D, true, 0x06, true, false, false)]
#[case::d_nc_nc_h(0x8A, 0x23, 0x1F, R8::D, false, 0x42, false, true, false)]
#[case::d_c_nc_h(0x8A, 0x23, 0x1F, R8::D, true, 0x43, false, true, false)]
#[case::d_nc_c_h(0x8A, 0xF3, 0x1F, R8::D, false, 0x12, true, true, false)]
#[case::d_c_c_h(0x8A, 0xF3, 0x1F, R8::D, true, 0x13, true, true, false)]
#[case::d_zero(0x8A, 0xFE, 0x01, R8::D, true, 0x00, true, true, true)]
#[case::e_nc_nc_nh(0x8B, 0x23, 0x12, R8::E, false, 0x35, false, false, false)]
#[case::e_c_nc_nh(0x8B, 0x23, 0x12, R8::E, true, 0x36, false, false, false)]
#[case::e_nc_c_nh(0x8B, 0xF3, 0x12, R8::E, false, 0x05, true, false, false)]
#[case::e_c_c_nh(0x8B, 0xF3, 0x12, R8::E, true, 0x06, true, false, false)]
#[case::e_nc_nc_h(0x8B, 0x23, 0x1F, R8::E, false, 0x42, false, true, false)]
#[case::e_c_nc_h(0x8B, 0x23, 0x1F, R8::E, true, 0x43, false, true, false)]
#[case::e_nc_c_h(0x8B, 0xF3, 0x1F, R8::E, false, 0x12, true, true, false)]
#[case::e_c_c_h(0x8B, 0xF3, 0x1F, R8::E, true, 0x13, true, true, false)]
#[case::e_zero(0x8B, 0xFE, 0x01, R8::E, true, 0x00, true, true, true)]
#[case::h_nc_nc_nh(0x8C, 0x23, 0x12, R8::H, false, 0x35, false, false, false)]
#[case::h_c_nc_nh(0x8C, 0x23, 0x12, R8::H, true, 0x36, false, false, false)]
#[case::h_nc_c_nh(0x8C, 0xF3, 0x12, R8::H, false, 0x05, true, false, false)]
#[case::h_c_c_nh(0x8C, 0xF3, 0x12, R8::H, true, 0x06, true, false, false)]
#[case::h_nc_nc_h(0x8C, 0x23, 0x1F, R8::H, false, 0x42, false, true, false)]
#[case::h_c_nc_h(0x8C, 0x23, 0x1F, R8::H, true, 0x43, false, true, false)]
#[case::h_nc_c_h(0x8C, 0xF3, 0x1F, R8::H, false, 0x12, true, true, false)]
#[case::h_c_c_h(0x8C, 0xF3, 0x1F, R8::H, true, 0x13, true, true, false)]
#[case::h_zero(0x8C, 0xFE, 0x01, R8::H, true, 0x00, true, true, true)]
#[case::l_nc_nc_nh(0x8D, 0x23, 0x12, R8::L, false, 0x35, false, false, false)]
#[case::l_c_nc_nh(0x8D, 0x23, 0x12, R8::L, true, 0x36, false, false, false)]
#[case::l_nc_c_nh(0x8D, 0xF3, 0x12, R8::L, false, 0x05, true, false, false)]
#[case::l_c_c_nh(0x8D, 0xF3, 0x12, R8::L, true, 0x06, true, false, false)]
#[case::l_nc_nc_h(0x8D, 0x23, 0x1F, R8::L, false, 0x42, false, true, false)]
#[case::l_c_nc_h(0x8D, 0x23, 0x1F, R8::L, true, 0x43, false, true, false)]
#[case::l_nc_c_h(0x8D, 0xF3, 0x1F, R8::L, false, 0x12, true, true, false)]
#[case::l_c_c_h(0x8D, 0xF3, 0x1F, R8::L, true, 0x13, true, true, false)]
#[case::l_zero(0x8D, 0xFE, 0x01, R8::L, true, 0x00, true, true, true)]
#[case::hl_nc_nc_nh(0x8E, 0x23, 0x12, R8::HL, false, 0x35, false, false, false)]
#[case::hl_c_nc_nh(0x8E, 0x23, 0x12, R8::HL, true, 0x36, false, false, false)]
#[case::hl_nc_c_nh(0x8E, 0xF3, 0x12, R8::HL, false, 0x05, true, false, false)]
#[case::hl_c_c_nh(0x8E, 0xF3, 0x12, R8::HL, true, 0x06, true, false, false)]
#[case::hl_nc_nc_h(0x8E, 0x23, 0x1F, R8::HL, false, 0x42, false, true, false)]
#[case::hl_c_nc_h(0x8E, 0x23, 0x1F, R8::HL, true, 0x43, false, true, false)]
#[case::hl_nc_c_h(0x8E, 0xF3, 0x1F, R8::HL, false, 0x12, true, true, false)]
#[case::hl_c_c_h(0x8E, 0xF3, 0x1F, R8::HL, true, 0x13, true, true, false)]
#[case::hl_zero(0x8E, 0xFE, 0x01, R8::HL, true, 0x00, true, true, true)]
#[case::a_nc_nc_nh(0x8F, 0x12, 0x12, R8::A, false, 0x24, false, false, false)]
#[case::a_c_nc_nh(0x8F, 0x12, 0x12, R8::A, true, 0x25, false, false, false)]
#[case::a_nc_c_nh(0x8F, 0x92, 0x92, R8::A, false, 0x24, true, false, false)]
#[case::a_c_c_nh(0x8F, 0x92, 0x92, R8::A, true, 0x25, true, false, false)]
#[case::a_nc_nc_h(0x8F, 0x19, 0x19, R8::A, false, 0x32, false, true, false)]
#[case::a_c_nc_h(0x8F, 0x19, 0x19, R8::A, true, 0x33, false, true, false)]
#[case::a_nc_c_h(0x8F, 0x88, 0x88, R8::A, false, 0x10, true, true, false)]
#[case::a_c_c_h(0x8F, 0x88, 0x88, R8::A, true, 0x11, true, true, false)]
#[case::a_zero(0x8F, 0x80, 0x80, R8::A, false, 0x00, true, false, true)]
fn test_adc_r8(
    #[case] opcode: u8,
    #[case] value_a: u8,
    #[case] value_r: u8,
    #[case] register: R8,
    #[case] carry: bool,
    #[case] expected_value: u8,
    #[case] expected_carry: bool,
    #[case] expected_half_carry: bool,
    #[case] expected_zero: bool,
) {
    let mut mmu = MMU::builder().rom(0, opcode).write(0xCCCC, value_r).build();
    let mut cpu = CPU::builder()
        .a(value_a)
        .hl(0xCCCC)
        .f_carry(carry)
        .r8(register, value_r, &mut mmu)
        .build();
    let m = cpu.step(&mut mmu);

    if register == R8::HL {
        assert_eq!(m, 2);
    } else {
        assert_eq!(m, 1);
    }
    assert_eq!(cpu.get_pc(), 1);
    assert_eq!(cpu.get_a(), expected_value);
    assert!(!cpu.get_f_subtract());
    assert_eq!(cpu.get_f_carry(), expected_carry);
    assert_eq!(cpu.get_f_half_carry(), expected_half_carry);
    assert_eq!(cpu.get_f_zero(), expected_zero);
}

/// ADC imm8 (0xCE)
#[rstest]
#[case::nc_nc_nh_nz(0x12, 0x23, false, 0x35, false, false, false)]
#[case::c_nc_nh_nz(0x12, 0x23, true, 0x36, false, false, false)]
#[case::nc_c_nh_nz(0x12, 0xF3, false, 0x05, true, false, false)]
#[case::c_c_nh_nz(0x12, 0xF3, true, 0x06, true, false, false)]
#[case::nc_nc_h_nz(0x12, 0x2F, false, 0x41, false, true, false)]
#[case::c_nc_h_nz(0x12, 0x2F, true, 0x42, false, true, false)]
#[case::nc_c_h_nz(0x12, 0xFF, false, 0x11, true, true, false)]
#[case::c_c_h_nz(0x12, 0xFF, true, 0x12, true, true, false)]
#[case::nc_nc_nh_z(0x00, 0x00, false, 0x00, false, false, true)]
#[case::nc_c_h_z(0xFF, 0x01, false, 0x00, true, true, true)]
#[case::c_c_h_z(0xFE, 0x01, true, 0x00, true, true, true)]
fn test_adc_imm8(
    #[case] value_a: u8,
    #[case] imm: u8,
    #[case] carry: bool,
    #[case] expected_value: u8,
    #[case] expected_carry: bool,
    #[case] expected_half_carry: bool,
    #[case] expected_zero: bool,
) {
    let mut mmu = MMU::builder().rom(0, 0xCE).rom(1, imm).build();
    let mut cpu = CPU::builder().a(value_a).f_carry(carry).build();
    let m = cpu.step(&mut mmu);

    assert_eq!(m, 2);
    assert_eq!(cpu.get_pc(), 2);
    assert_eq!(cpu.get_a(), expected_value);
    assert_eq!(cpu.get_f_carry(), expected_carry);
    assert_eq!(cpu.get_f_half_carry(), expected_half_carry);
    assert_eq!(cpu.get_f_zero(), expected_zero);
    assert!(!cpu.get_f_subtract());
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

/// ADD SP imm8 (0xE8)
#[rstest]
#[case::negative(0xFFFE, -1, 0xFFFD)]
#[case::negative(0xFFFE, -16, 0xFFEE)]
#[case::positive(0xFFFE, 1, 0xFFFF)]
#[case::positive(0xFFEF, 16, 0xFFFF)]
fn test_add_sp_imm8(#[case] value_sp: u16, #[case] imm: i8, #[case] expected_sp: u16) {
    let mut mmu = MMU::builder().rom(0, 0xE8).rom(1, imm as u8).build();
    let mut cpu = CPU::builder().sp(value_sp).build();
    let m = cpu.step(&mut mmu);

    assert_eq!(m, 4);
    assert_eq!(cpu.get_pc(), 2);
    assert_eq!(cpu.get_sp(), expected_sp);
}

/// AND r8
#[rstest]
#[case::b_nz(0xA0, 0b0101_1011, 0b0100_0101, R8::B, 0b0100_0001, false)]
#[case::b_z(0xA0, 0b0101_1010, 0b1000_0101, R8::B, 0b0000_0000, true)]
#[case::c_nz(0xA1, 0b0101_1011, 0b0100_0101, R8::C, 0b0100_0001, false)]
#[case::c_z(0xA1, 0b0101_1010, 0b1000_0101, R8::C, 0b0000_0000, true)]
#[case::d_nz(0xA2, 0b0101_1011, 0b0100_0101, R8::D, 0b0100_0001, false)]
#[case::d_z(0xA2, 0b0101_1010, 0b1000_0101, R8::D, 0b0000_0000, true)]
#[case::e_nz(0xA3, 0b0101_1011, 0b0100_0101, R8::E, 0b0100_0001, false)]
#[case::e_z(0xA3, 0b0101_1010, 0b1000_0101, R8::E, 0b0000_0000, true)]
#[case::h_nz(0xA4, 0b0101_1011, 0b0100_0101, R8::H, 0b0100_0001, false)]
#[case::h_z(0xA4, 0b0101_1010, 0b1000_0101, R8::H, 0b0000_0000, true)]
#[case::l_nz(0xA5, 0b0101_1011, 0b0100_0101, R8::L, 0b0100_0001, false)]
#[case::l_z(0xA5, 0b0101_1010, 0b1000_0101, R8::L, 0b0000_0000, true)]
#[case::hl_nz(0xA6, 0b0101_1011, 0b0100_0101, R8::HL, 0b0100_0001, false)]
#[case::hl_z(0xA6, 0b0101_1010, 0b1000_0101, R8::HL, 0b0000_0000, true)]
#[case::a_nz(0xA7, 0b0101_1011, 0b0101_1011, R8::A, 0b0101_1011, false)]
fn test_and_r8(
    #[case] opcode: u8,
    #[case] value_a: u8,
    #[case] value_r: u8,
    #[case] register: R8,
    #[case] expected_value: u8,
    #[case] expected_zero: bool,
) {
    let mut mmu = MMU::builder().rom(0, opcode).write(0xCCCC, value_r).build();
    let mut cpu = CPU::builder()
        .a(value_a)
        .hl(0xCCCC)
        .r8(register, value_r, &mut mmu)
        .build();
    let m = cpu.step(&mut mmu);

    if register == R8::HL {
        assert_eq!(m, 2);
    } else {
        assert_eq!(m, 1);
    }
    assert_eq!(cpu.get_pc(), 1);
    assert_eq!(cpu.get_a(), expected_value);
    assert!(!cpu.get_f_subtract());
    assert!(!cpu.get_f_carry());
    assert!(cpu.get_f_half_carry());
    assert_eq!(cpu.get_f_zero(), expected_zero);
}

/// AND imm8 (0xE6)
#[rstest]
#[case::nz(0b0101_1011, 0b0100_0101, 0b0100_0001, false)]
#[case::z(0b0101_1010, 0b1000_0101, 0b0000_0000, true)]
fn test_and_imm8(
    #[case] value_a: u8,
    #[case] imm: u8,
    #[case] expected_value: u8,
    #[case] expected_zero: bool,
) {
    let mut mmu = MMU::builder().rom(0, 0xE6).rom(1, imm).build();
    let mut cpu = CPU::builder().a(value_a).build();
    let m = cpu.step(&mut mmu);

    assert_eq!(m, 2);
    assert_eq!(cpu.get_pc(), 2);
    assert_eq!(cpu.get_a(), expected_value);
    assert!(!cpu.get_f_subtract());
    assert!(!cpu.get_f_carry());
    assert!(cpu.get_f_half_carry());
    assert_eq!(cpu.get_f_zero(), expected_zero);
}

/// Call (0xCD)
#[test]
fn test_call() {
    let mut mmu = MMU::builder()
        .rom(0x11FD, 0xCD)
        .rom(0x11FE, 0xFF)
        .rom(0x11FF, 0xCC)
        .build();
    let mut cpu = CPU::builder().pc(0x11FD).sp(0xFFFE).build();
    let m = cpu.step(&mut mmu);

    assert_eq!(m, 6);
    assert_eq!(cpu.get_pc(), 0xCCFF);
    assert_eq!(mmu.read_16(0xFFFC), 0x1200);
}

/// Call cond
#[rstest]
#[case::nz_no(0xC4, 0x11FD, 0xFFFE, 0xFF, 0xCC, true, false, 0x1200, 0xFFFE, 3)]
#[case::nz_yes(0xC4, 0x11FD, 0xFFFE, 0xFF, 0xCC, false, false, 0xCCFF, 0xFFFC, 6)]
#[case::z_no(0xCC, 0x11FD, 0xFFFE, 0xFF, 0xCC, false, false, 0x1200, 0xFFFE, 3)]
#[case::z_yes(0xCC, 0x11FD, 0xFFFE, 0xFF, 0xCC, true, false, 0xCCFF, 0xFFFC, 6)]
#[case::nc_no(0xD4, 0x11FD, 0xFFFE, 0xFF, 0xCC, false, true, 0x1200, 0xFFFE, 3)]
#[case::nc_yes(0xD4, 0x11FD, 0xFFFE, 0xFF, 0xCC, false, false, 0xCCFF, 0xFFFC, 6)]
#[case::c_no(0xDC, 0x11FD, 0xFFFE, 0xFF, 0xCC, false, false, 0x1200, 0xFFFE, 3)]
#[case::c_yes(0xDC, 0x11FD, 0xFFFE, 0xFF, 0xCC, false, true, 0xCCFF, 0xFFFC, 6)]
fn test_call_cond(
    #[case] opcode: u8,
    #[case] initial_pc: u16,
    #[case] initial_sp: u16,
    #[case] imm1: u8,
    #[case] imm2: u8,
    #[case] flag_zero: bool,
    #[case] flag_carry: bool,
    #[case] expected_pc: u16,
    #[case] expected_sp: u16,
    #[case] expected_m: u8,
) {
    let mut mmu = MMU::builder()
        .rom(initial_pc, opcode)
        .rom(initial_pc + 1, imm1)
        .rom(initial_pc + 2, imm2)
        .build();
    let mut cpu = CPU::builder()
        .pc(initial_pc)
        .sp(initial_sp)
        .f_zero(flag_zero)
        .f_carry(flag_carry)
        .build();
    let m = cpu.step(&mut mmu);

    assert_eq!(m, expected_m);
    assert_eq!(cpu.get_pc(), expected_pc);
    assert_eq!(cpu.get_sp(), expected_sp);
}

/// CP r8
#[rstest]
#[case::b_nc_nh(0xB8, 0x34, 0x21, R8::B, 0x34, false, false, false)]
#[case::b_c_nh(0xB8, 0x34, 0x51, R8::B, 0x34, true, false, false)]
#[case::b_nc_h(0xB8, 0x34, 0x25, R8::B, 0x34, false, true, false)]
#[case::b_c_h(0xB8, 0x34, 0x55, R8::B, 0x34, true, true, false)]
#[case::b_zero(0xB8, 0x00, 0x55, R8::B, 0x00, true, true, false)]
#[case::c_nc_nh(0xB9, 0x34, 0x21, R8::C, 0x34, false, false, false)]
#[case::c_c_nh(0xB9, 0x34, 0x51, R8::C, 0x34, true, false, false)]
#[case::c_nc_h(0xB9, 0x34, 0x25, R8::C, 0x34, false, true, false)]
#[case::c_c_h(0xB9, 0x34, 0x55, R8::C, 0x34, true, true, false)]
#[case::c_zero(0xB9, 0x00, 0x55, R8::C, 0x00, true, true, false)]
#[case::d_nc_nh(0xBA, 0x34, 0x21, R8::D, 0x34, false, false, false)]
#[case::d_c_nh(0xBA, 0x34, 0x51, R8::D, 0x34, true, false, false)]
#[case::d_nc_h(0xBA, 0x34, 0x25, R8::D, 0x34, false, true, false)]
#[case::d_c_h(0xBA, 0x34, 0x55, R8::D, 0x34, true, true, false)]
#[case::d_zero(0xBA, 0x00, 0x55, R8::D, 0x00, true, true, false)]
#[case::e_nc_nh(0xBB, 0x34, 0x21, R8::E, 0x34, false, false, false)]
#[case::e_c_nh(0xBB, 0x34, 0x51, R8::E, 0x34, true, false, false)]
#[case::e_nc_h(0xBB, 0x34, 0x25, R8::E, 0x34, false, true, false)]
#[case::e_c_h(0xBB, 0x34, 0x55, R8::E, 0x34, true, true, false)]
#[case::e_zero(0xBB, 0x00, 0x55, R8::E, 0x00, true, true, false)]
#[case::h_nc_nh(0xBC, 0x34, 0x21, R8::H, 0x34, false, false, false)]
#[case::h_c_nh(0xBC, 0x34, 0x51, R8::H, 0x34, true, false, false)]
#[case::h_nc_h(0xBC, 0x34, 0x25, R8::H, 0x34, false, true, false)]
#[case::h_c_h(0xBC, 0x34, 0x55, R8::H, 0x34, true, true, false)]
#[case::h_zero(0xBC, 0x00, 0x55, R8::H, 0x00, true, true, false)]
#[case::l_nc_nh(0xBD, 0x34, 0x21, R8::L, 0x34, false, false, false)]
#[case::l_c_nh(0xBD, 0x34, 0x51, R8::L, 0x34, true, false, false)]
#[case::l_nc_h(0xBD, 0x34, 0x25, R8::L, 0x34, false, true, false)]
#[case::l_c_h(0xBD, 0x34, 0x55, R8::L, 0x34, true, true, false)]
#[case::l_zero(0xBD, 0x00, 0x55, R8::L, 0x00, true, true, false)]
#[case::hl_nc_nh(0xBE, 0x34, 0x21, R8::HL, 0x34, false, false, false)]
#[case::hl_c_nh(0xBE, 0x34, 0x51, R8::HL, 0x34, true, false, false)]
#[case::hl_nc_h(0xBE, 0x34, 0x25, R8::HL, 0x34, false, true, false)]
#[case::hl_c_h(0xBE, 0x34, 0x55, R8::HL, 0x34, true, true, false)]
#[case::hl_zero(0xBE, 0x00, 0x55, R8::HL, 0x00, true, true, false)]
#[case::a_zero(0xBF, 0x00, 0x00, R8::A, 0x00, false, false, true)]
fn test_cp_r8(
    #[case] opcode: u8,
    #[case] value_a: u8,
    #[case] value_r: u8,
    #[case] register: R8,
    #[case] expected_value: u8,
    #[case] expected_carry: bool,
    #[case] expected_half_carry: bool,
    #[case] expected_zero: bool,
) {
    let mut mmu = MMU::builder().rom(0, opcode).write(0xCCCC, value_r).build();
    let mut cpu = CPU::builder()
        .a(value_a)
        .hl(0xCCCC)
        .r8(register, value_r, &mut mmu)
        .build();
    let m = cpu.step(&mut mmu);

    if register == R8::HL {
        assert_eq!(m, 2);
    } else {
        assert_eq!(m, 1);
    }
    assert_eq!(cpu.get_pc(), 1);
    assert_eq!(cpu.get_a(), expected_value);
    assert!(cpu.get_f_subtract());
    assert_eq!(cpu.get_f_carry(), expected_carry);
    assert_eq!(cpu.get_f_half_carry(), expected_half_carry);
    assert_eq!(cpu.get_f_zero(), expected_zero);
}

/// CP imm8 (0xFE)
#[rstest]
#[case::b_nc_nh(0x34, 0x21, 0x34, false, false, false)]
#[case::b_c_nh(0x34, 0x51, 0x34, true, false, false)]
#[case::b_nc_h(0x34, 0x25, 0x34, false, true, false)]
#[case::b_c_h(0x34, 0x55, 0x34, true, true, false)]
#[case::b_zero(0x00, 0x55, 0x00, true, true, false)]
fn test_cp_imm8(
    #[case] value_a: u8,
    #[case] imm: u8,
    #[case] expected_value: u8,
    #[case] expected_carry: bool,
    #[case] expected_half_carry: bool,
    #[case] expected_zero: bool,
) {
    let mut mmu = MMU::builder().rom(0, 0xFE).rom(1, imm).build();
    let mut cpu = CPU::builder().a(value_a).build();
    let m = cpu.step(&mut mmu);

    assert_eq!(m, 2);
    assert_eq!(cpu.get_pc(), 2);
    assert_eq!(cpu.get_a(), expected_value);
    assert!(cpu.get_f_subtract());
    assert_eq!(cpu.get_f_carry(), expected_carry);
    assert_eq!(cpu.get_f_half_carry(), expected_half_carry);
    assert_eq!(cpu.get_f_zero(), expected_zero);
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
#[case::increment_b_zero(0x05, 1)]
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

    if value == 1 {
        assert!(cpu.get_f_zero());
    }

    match opcode {
        0x05 => assert_eq!(cpu.get_b(), value.wrapping_sub(1)),
        0x0D => assert_eq!(cpu.get_c(), value.wrapping_sub(1)),
        0x15 => assert_eq!(cpu.get_d(), value.wrapping_sub(1)),
        0x1D => assert_eq!(cpu.get_e(), value.wrapping_sub(1)),
        0x25 => assert_eq!(cpu.get_h(), value.wrapping_sub(1)),
        0x2D => assert_eq!(cpu.get_l(), value.wrapping_sub(1)),
        0x3D => assert_eq!(cpu.get_a(), value.wrapping_sub(1)),
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
#[case::increment_b_zero(0x04, 0xFF)]
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

    if value == 0xFF {
        assert!(cpu.get_f_zero());
    }

    match opcode {
        0x04 => assert_eq!(cpu.get_b(), value.wrapping_add(1)),
        0x0C => assert_eq!(cpu.get_c(), value.wrapping_add(1)),
        0x14 => assert_eq!(cpu.get_d(), value.wrapping_add(1)),
        0x1C => assert_eq!(cpu.get_e(), value.wrapping_add(1)),
        0x24 => assert_eq!(cpu.get_h(), value.wrapping_add(1)),
        0x2C => assert_eq!(cpu.get_l(), value.wrapping_add(1)),
        0x3C => assert_eq!(cpu.get_a(), value.wrapping_add(1)),
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

/// LOAD r8 r8
#[rstest]
#[case::b_b(0x40, R8::B, R8::B)]
#[case::b_c(0x41, R8::B, R8::C)]
#[case::b_d(0x42, R8::B, R8::D)]
#[case::b_e(0x43, R8::B, R8::E)]
#[case::b_h(0x44, R8::B, R8::H)]
#[case::b_l(0x45, R8::B, R8::L)]
#[case::b_hl(0x46, R8::B, R8::HL)]
#[case::b_a(0x47, R8::B, R8::A)]
#[case::c_b(0x48, R8::C, R8::B)]
#[case::c_c(0x49, R8::C, R8::C)]
#[case::c_d(0x4A, R8::C, R8::D)]
#[case::c_e(0x4B, R8::C, R8::E)]
#[case::c_h(0x4C, R8::C, R8::H)]
#[case::c_l(0x4D, R8::C, R8::L)]
#[case::c_hl(0x4E, R8::C, R8::HL)]
#[case::c_a(0x4F, R8::C, R8::A)]
#[case::d_b(0x50, R8::D, R8::B)]
#[case::d_c(0x51, R8::D, R8::C)]
#[case::d_d(0x52, R8::D, R8::D)]
#[case::d_e(0x53, R8::D, R8::E)]
#[case::d_h(0x54, R8::D, R8::H)]
#[case::d_l(0x55, R8::D, R8::L)]
#[case::d_hl(0x56, R8::D, R8::HL)]
#[case::d_a(0x57, R8::D, R8::A)]
#[case::e_b(0x58, R8::E, R8::B)]
#[case::e_c(0x59, R8::E, R8::C)]
#[case::e_d(0x5A, R8::E, R8::D)]
#[case::e_e(0x5B, R8::E, R8::E)]
#[case::e_h(0x5C, R8::E, R8::H)]
#[case::e_l(0x5D, R8::E, R8::L)]
#[case::e_hl(0x5E, R8::E, R8::HL)]
#[case::e_a(0x5F, R8::E, R8::A)]
#[case::h_b(0x60, R8::H, R8::B)]
#[case::h_c(0x61, R8::H, R8::C)]
#[case::h_d(0x62, R8::H, R8::D)]
#[case::h_e(0x63, R8::H, R8::E)]
#[case::h_h(0x64, R8::H, R8::H)]
#[case::h_l(0x65, R8::H, R8::L)]
#[case::h_hl(0x66, R8::H, R8::HL)]
#[case::h_a(0x67, R8::H, R8::A)]
#[case::l_b(0x68, R8::L, R8::B)]
#[case::l_c(0x69, R8::L, R8::C)]
#[case::l_d(0x6A, R8::L, R8::D)]
#[case::l_e(0x6B, R8::L, R8::E)]
#[case::l_h(0x6C, R8::L, R8::H)]
#[case::l_l(0x6D, R8::L, R8::L)]
#[case::l_hl(0x6E, R8::L, R8::HL)]
#[case::l_a(0x6F, R8::L, R8::A)]
#[case::hl_b(0x70, R8::HL, R8::B)]
#[case::hl_c(0x71, R8::HL, R8::C)]
#[case::hl_d(0x72, R8::HL, R8::D)]
#[case::hl_e(0x73, R8::HL, R8::E)]
#[case::hl_h(0x74, R8::HL, R8::H)]
#[case::hl_l(0x75, R8::HL, R8::L)]
// LD HL HL = HALT
#[case::hl_a(0x77, R8::HL, R8::A)]
#[case::a_b(0x78, R8::A, R8::B)]
#[case::a_c(0x79, R8::A, R8::C)]
#[case::a_d(0x7A, R8::A, R8::D)]
#[case::a_e(0x7B, R8::A, R8::E)]
#[case::a_h(0x7C, R8::A, R8::H)]
#[case::a_l(0x7D, R8::A, R8::L)]
#[case::a_hl(0x7E, R8::A, R8::HL)]
#[case::a_a(0x7F, R8::A, R8::A)]
fn test_ld_r8_r8(#[case] opcode: u8, #[case] target_reg: R8, #[case] source_reg: R8) {
    let mut mmu = MMU::builder().rom(0, opcode).build();
    let mut cpu = CPU::builder()
        .hl(0xCCCC)
        .r8(source_reg, 0xCC, &mut mmu)
        .build();
    let m = cpu.step(&mut mmu);

    if target_reg == R8::HL || source_reg == R8::HL {
        assert_eq!(m, 2);
    } else {
        assert_eq!(m, 1);
    }
    assert_eq!(cpu.get_pc(), 1);
    assert_eq!(cpu.get_r8(target_reg, &mmu), 0xCC);
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

/// LDH A, C (0xF2)
#[test]
fn test_ldh_a_c() {
    let mut mmu = MMU::builder().rom(0, 0xF2).write(0xFF13, 0x68).build();
    let mut cpu = CPU::builder().c(0x13).build();
    let m = cpu.step(&mut mmu);

    assert_eq!(m, 2);
    assert_eq!(cpu.get_pc(), 1);
    assert_eq!(cpu.get_a(), 0x68);
}

/// LDH C, A (0xE2)
#[test]
fn test_ldh_c_a() {
    let mut mmu = MMU::builder().rom(0, 0xE2).build();
    let mut cpu = CPU::builder().a(0x68).c(0x13).build();
    let m = cpu.step(&mut mmu);

    assert_eq!(m, 2);
    assert_eq!(cpu.get_pc(), 1);
    assert_eq!(mmu.read(0xFF13), 0x68);
}

/// LDH A, imm8 (0xF0)
#[test]
fn test_ldh_a_imm8() {
    let mut mmu = MMU::builder()
        .rom(0, 0xF0)
        .rom(1, 0x77)
        .write(0xFF77, 0x68)
        .build();
    let mut cpu = CPU::default();
    let m = cpu.step(&mut mmu);

    assert_eq!(m, 3);
    assert_eq!(cpu.get_pc(), 2);
    assert_eq!(cpu.get_a(), 0x68);
}

/// LDH imm8, A (0xE0)
#[test]
fn test_ldh_imm8_a() {
    let mut mmu = MMU::builder().rom(0, 0xE0).rom(1, 0x77).build();
    let mut cpu = CPU::builder().a(0x68).build();
    let m = cpu.step(&mut mmu);

    assert_eq!(m, 3);
    assert_eq!(cpu.get_pc(), 2);
    assert_eq!(mmu.read(0xFF77), 0x68);
}

/// LD A, imm16 (0xFA)
#[test]
fn test_ld_a_imm16() {
    let mut mmu = MMU::builder()
        .rom(0, 0xFA)
        .rom(1, 0x33)
        .rom(2, 0xCC)
        .write(0xCC33, 0x68)
        .build();
    let mut cpu = CPU::default();
    let m = cpu.step(&mut mmu);

    assert_eq!(m, 4);
    assert_eq!(cpu.get_pc(), 3);
    assert_eq!(cpu.get_a(), 0x68);
}

/// LD imm16, A (0xEA)
#[test]
fn test_ld_imm16_a() {
    let mut mmu = MMU::builder()
        .rom(0, 0xEA)
        .rom(1, 0x33)
        .rom(2, 0xCC)
        .build();
    let mut cpu = CPU::builder().a(0x68).build();
    let m = cpu.step(&mut mmu);

    assert_eq!(m, 4);
    assert_eq!(cpu.get_pc(), 3);
    assert_eq!(mmu.read(0xCC33), 0x68);
}

/// LD HL, SP+e (0xF8)
#[rstest]
#[case::positive(0xFFDF, 16, 0xFFEF)]
#[case::negative(0xFFDF, -16, 0xFFCF)]
fn test_load_hl_sp_imm8(#[case] sp: u16, #[case] imm: i8, #[case] expected_hl: u16) {
    let mut mmu = MMU::builder().rom(0, 0xF8).rom(1, imm as u8).build();
    let mut cpu = CPU::builder().sp(sp).build();
    let m = cpu.step(&mut mmu);

    assert_eq!(m, 3);
    assert_eq!(cpu.get_pc(), 2);
    assert_eq!(cpu.get_hl(), expected_hl);
}

/// LD SP, HL (0xF9)
#[test]
fn test_load_sp_hl() {
    let mut mmu = MMU::builder().rom(0, 0xF9).build();
    let mut cpu = CPU::builder().hl(0x1337).build();
    let m = cpu.step(&mut mmu);

    assert_eq!(m, 2);
    assert_eq!(cpu.get_pc(), 1);
    assert_eq!(cpu.get_sp(), 0x1337);
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
#[case::nz_jump(0x20, 30, 0, 32, 3, false, false)]
#[case::nz_no_jump(0x20, 30, 0, 2, 2, true, false)]
#[case::z_jump(0x28, 30, 0, 32, 3, true, false)]
#[case::z_no_jump(0x28, 30, 0, 2, 2, false, false)]
#[case::nc_jump(0x30, 30, 0, 32, 3, false, false)]
#[case::nc_no_jump(0x30, 30, 0, 2, 2, false, true)]
#[case::c_jump(0x38, 30, 0, 32, 3, false, true)]
#[case::c_no_jump(0x38, 30, 0, 2, 2, false, false)]
#[case::jump_negative(0x38, -30, 32, 4, 3, false, true)]
fn test_jr_cond_imm8(
    #[case] opcode: u8,
    #[case] immediate: i8,
    #[case] pc: u16,
    #[case] target_pc: u16,
    #[case] target_m: u8,
    #[case] zero_flag: bool,
    #[case] carry_flag: bool,
) {
    let mut mmu = MMU::builder()
        .rom(pc, opcode)
        .rom(pc + 1, immediate as u8)
        .build();
    let mut cpu = CPU::builder()
        .pc(pc)
        .f_zero(zero_flag)
        .f_carry(carry_flag)
        .build();
    let m = cpu.step(&mut mmu);

    assert_eq!(m, target_m);
    assert_eq!(cpu.get_pc(), target_pc);
}

/// OR r8
#[rstest]
#[case::b_nz(0xB0, 0b1010_1010, 0b0101_0101, R8::B, 0b1111_1111, false)]
#[case::b_z(0xB0, 0b0000_0000, 0b0000_0000, R8::B, 0b0000_0000, true)]
#[case::c_nz(0xB1, 0b1010_1010, 0b0101_0101, R8::C, 0b1111_1111, false)]
#[case::c_z(0xB1, 0b0000_0000, 0b0000_0000, R8::C, 0b0000_0000, true)]
#[case::d_nz(0xB2, 0b1010_1010, 0b0101_0101, R8::D, 0b1111_1111, false)]
#[case::d_z(0xB2, 0b0000_0000, 0b0000_0000, R8::D, 0b0000_0000, true)]
#[case::e_nz(0xB3, 0b1010_1010, 0b0101_0101, R8::E, 0b1111_1111, false)]
#[case::e_z(0xB3, 0b0000_0000, 0b0000_0000, R8::E, 0b0000_0000, true)]
#[case::h_nz(0xB4, 0b1010_1010, 0b0101_0101, R8::H, 0b1111_1111, false)]
#[case::h_z(0xB4, 0b0000_0000, 0b0000_0000, R8::H, 0b0000_0000, true)]
#[case::l_nz(0xB5, 0b1010_1010, 0b0101_0101, R8::L, 0b1111_1111, false)]
#[case::l_z(0xB5, 0b0000_0000, 0b0000_0000, R8::L, 0b0000_0000, true)]
#[case::hl_nz(0xB6, 0b1010_1010, 0b0101_0101, R8::HL, 0b1111_1111, false)]
#[case::hl_z(0xB6, 0b0000_0000, 0b0000_0000, R8::HL, 0b0000_0000, true)]
#[case::a_z(0xB7, 0b0000_0000, 0b0000_0000, R8::A, 0b0000_0000, true)]
fn test_or_r8(
    #[case] opcode: u8,
    #[case] value_a: u8,
    #[case] value_r: u8,
    #[case] register: R8,
    #[case] expected_value: u8,
    #[case] expected_zero: bool,
) {
    let mut mmu = MMU::builder().rom(0, opcode).write(0xCCCC, value_r).build();
    let mut cpu = CPU::builder()
        .a(value_a)
        .hl(0xCCCC)
        .r8(register, value_r, &mut mmu)
        .build();
    let m = cpu.step(&mut mmu);

    if register == R8::HL {
        assert_eq!(m, 2);
    } else {
        assert_eq!(m, 1);
    }
    assert_eq!(cpu.get_pc(), 1);
    assert_eq!(cpu.get_a(), expected_value);
    assert!(!cpu.get_f_subtract());
    assert!(!cpu.get_f_carry());
    assert!(!cpu.get_f_half_carry());
    assert_eq!(cpu.get_f_zero(), expected_zero);
}

/// OR imm8 (0xF6)
#[rstest]
#[case::b_nz(0b1010_1010, 0b0101_0101, 0b1111_1111, false)]
#[case::b_z(0b0000_0000, 0b0000_0000, 0b0000_0000, true)]
fn test_or_imm8(
    #[case] value_a: u8,
    #[case] imm: u8,
    #[case] expected_value: u8,
    #[case] expected_zero: bool,
) {
    let mut mmu = MMU::builder().rom(0, 0xF6).rom(1, imm).build();
    let mut cpu = CPU::builder().a(value_a).build();
    let m = cpu.step(&mut mmu);

    assert_eq!(m, 2);
    assert_eq!(cpu.get_pc(), 2);
    assert_eq!(cpu.get_a(), expected_value);
    assert!(!cpu.get_f_subtract());
    assert!(!cpu.get_f_carry());
    assert!(!cpu.get_f_half_carry());
    assert_eq!(cpu.get_f_zero(), expected_zero);
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

/// RST
#[rstest]
#[case(0xC7, 0x0000)]
#[case(0xCF, 0x0008)]
#[case(0xD7, 0x0010)]
#[case(0xDF, 0x0018)]
#[case(0xE7, 0x0020)]
#[case(0xEF, 0x0028)]
#[case(0xF7, 0x0030)]
#[case(0xFF, 0x0038)]
fn test_rst(#[case] opcode: u8, #[case] expected_pc: u16) {
    let mut mmu = MMU::builder().rom(0, opcode).build();
    let mut cpu = CPU::default();
    let m = cpu.step(&mut mmu);

    assert_eq!(m, 4);
    assert_eq!(cpu.get_pc(), expected_pc);
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

/// SUB r8
#[rstest]
#[case::b_nc_nh(0x90, 0x34, 0x21, R8::B, 0x13, false, false, false)]
#[case::b_c_nh(0x90, 0x34, 0x51, R8::B, 0xE3, true, false, false)]
#[case::b_nc_h(0x90, 0x34, 0x25, R8::B, 0x0F, false, true, false)]
#[case::b_c_h(0x90, 0x34, 0x55, R8::B, 0xDF, true, true, false)]
#[case::b_zero(0x90, 0x55, 0x55, R8::B, 0x00, false, false, true)]
#[case::c_nc_nh(0x91, 0x34, 0x21, R8::C, 0x13, false, false, false)]
#[case::c_c_nh(0x91, 0x34, 0x51, R8::C, 0xE3, true, false, false)]
#[case::c_nc_h(0x91, 0x34, 0x25, R8::C, 0x0F, false, true, false)]
#[case::c_c_h(0x91, 0x34, 0x55, R8::C, 0xDF, true, true, false)]
#[case::c_zero(0x91, 0x55, 0x55, R8::C, 0x00, false, false, true)]
#[case::d_nc_nh(0x92, 0x34, 0x21, R8::D, 0x13, false, false, false)]
#[case::d_c_nh(0x92, 0x34, 0x51, R8::D, 0xE3, true, false, false)]
#[case::d_nc_h(0x92, 0x34, 0x25, R8::D, 0x0F, false, true, false)]
#[case::d_c_h(0x92, 0x34, 0x55, R8::D, 0xDF, true, true, false)]
#[case::d_zero(0x92, 0x55, 0x55, R8::D, 0x00, false, false, true)]
#[case::e_nc_nh(0x93, 0x34, 0x21, R8::E, 0x13, false, false, false)]
#[case::e_c_nh(0x93, 0x34, 0x51, R8::E, 0xE3, true, false, false)]
#[case::e_nc_h(0x93, 0x34, 0x25, R8::E, 0x0F, false, true, false)]
#[case::e_c_h(0x93, 0x34, 0x55, R8::E, 0xDF, true, true, false)]
#[case::e_zero(0x93, 0x55, 0x55, R8::E, 0x00, false, false, true)]
#[case::h_nc_nh(0x94, 0x34, 0x21, R8::H, 0x13, false, false, false)]
#[case::h_c_nh(0x94, 0x34, 0x51, R8::H, 0xE3, true, false, false)]
#[case::h_nc_h(0x94, 0x34, 0x25, R8::H, 0x0F, false, true, false)]
#[case::h_c_h(0x94, 0x34, 0x55, R8::H, 0xDF, true, true, false)]
#[case::h_zero(0x94, 0x55, 0x55, R8::H, 0x00, false, false, true)]
#[case::l_nc_nh(0x95, 0x34, 0x21, R8::L, 0x13, false, false, false)]
#[case::l_c_nh(0x95, 0x34, 0x51, R8::L, 0xE3, true, false, false)]
#[case::l_nc_h(0x95, 0x34, 0x25, R8::L, 0x0F, false, true, false)]
#[case::l_c_h(0x95, 0x34, 0x55, R8::L, 0xDF, true, true, false)]
#[case::l_zero(0x95, 0x55, 0x55, R8::L, 0x00, false, false, true)]
#[case::hl_nc_nh(0x96, 0x34, 0x21, R8::HL, 0x13, false, false, false)]
#[case::hl_c_nh(0x96, 0x34, 0x51, R8::HL, 0xE3, true, false, false)]
#[case::hl_nc_h(0x96, 0x34, 0x25, R8::HL, 0x0F, false, true, false)]
#[case::hl_c_h(0x96, 0x34, 0x55, R8::HL, 0xDF, true, true, false)]
#[case::hl_zero(0x96, 0x55, 0x55, R8::HL, 0x00, false, false, true)]
#[case::a_zero(0x97, 0x55, 0x55, R8::A, 0x00, false, false, true)]
fn test_sub_r8(
    #[case] opcode: u8,
    #[case] value_a: u8,
    #[case] value_r: u8,
    #[case] register: R8,
    #[case] expected_value: u8,
    #[case] expected_carry: bool,
    #[case] expected_half_carry: bool,
    #[case] expected_zero: bool,
) {
    let mut mmu = MMU::builder().rom(0, opcode).write(0xCCCC, value_r).build();
    let mut cpu = CPU::builder()
        .a(value_a)
        .hl(0xCCCC)
        .r8(register, value_r, &mut mmu)
        .build();
    let m = cpu.step(&mut mmu);

    if register == R8::HL {
        assert_eq!(m, 2);
    } else {
        assert_eq!(m, 1);
    }
    assert_eq!(cpu.get_pc(), 1);
    assert_eq!(cpu.get_a(), expected_value);
    assert!(cpu.get_f_subtract());
    assert_eq!(cpu.get_f_carry(), expected_carry);
    assert_eq!(cpu.get_f_half_carry(), expected_half_carry);
    assert_eq!(cpu.get_f_zero(), expected_zero);
}

/// SUB imm8 (0xD6)
#[rstest]
#[case::b_nc_nh(0x34, 0x21, 0x13, false, false, false)]
#[case::b_c_nh(0x34, 0x51, 0xE3, true, false, false)]
#[case::b_nc_h(0x34, 0x25, 0x0F, false, true, false)]
#[case::b_c_h(0x34, 0x55, 0xDF, true, true, false)]
#[case::b_zero(0x55, 0x55, 0x00, false, false, true)]
fn test_sub_imm8(
    #[case] value_a: u8,
    #[case] imm: u8,
    #[case] expected_value: u8,
    #[case] expected_carry: bool,
    #[case] expected_half_carry: bool,
    #[case] expected_zero: bool,
) {
    let mut mmu = MMU::builder().rom(0, 0xD6).rom(1, imm).build();
    let mut cpu = CPU::builder().a(value_a).build();
    let m = cpu.step(&mut mmu);

    assert_eq!(m, 2);
    assert_eq!(cpu.get_pc(), 2);
    assert_eq!(cpu.get_a(), expected_value);
    assert!(cpu.get_f_subtract());
    assert_eq!(cpu.get_f_carry(), expected_carry);
    assert_eq!(cpu.get_f_half_carry(), expected_half_carry);
    assert_eq!(cpu.get_f_zero(), expected_zero);
}

/// SBC r8
#[rstest]
#[case::b_nc_nc_nh(0x98, 0x34, 0x21, R8::B, false, 0x13, false, false, false)]
#[case::b_c_nc_nh(0x98, 0x34, 0x21, R8::B, true, 0x12, false, false, false)]
#[case::b_nc_c_nh(0x98, 0x34, 0x51, R8::B, false, 0xE3, true, false, false)]
#[case::b_c_c_nh(0x98, 0x34, 0x51, R8::B, true, 0xE2, true, false, false)]
#[case::b_nc_nc_h(0x98, 0x34, 0x25, R8::B, false, 0x0F, false, true, false)]
#[case::b_c_nc_h(0x98, 0x34, 0x25, R8::B, true, 0x0E, false, true, false)]
#[case::b_nc_c_h(0x98, 0x34, 0x55, R8::B, false, 0xDF, true, true, false)]
#[case::b_c_c_h(0x98, 0x34, 0x55, R8::B, true, 0xDE, true, true, false)]
#[case::b_zero(0x98, 0x55, 0x54, R8::B, true, 0x00, false, false, true)]
#[case::c_nc_nc_nh(0x99, 0x34, 0x21, R8::C, false, 0x13, false, false, false)]
#[case::c_c_nc_nh(0x99, 0x34, 0x21, R8::C, true, 0x12, false, false, false)]
#[case::c_nc_c_nh(0x99, 0x34, 0x51, R8::C, false, 0xE3, true, false, false)]
#[case::c_c_c_nh(0x99, 0x34, 0x51, R8::C, true, 0xE2, true, false, false)]
#[case::c_nc_nc_h(0x99, 0x34, 0x25, R8::C, false, 0x0F, false, true, false)]
#[case::c_c_nc_h(0x99, 0x34, 0x25, R8::C, true, 0x0E, false, true, false)]
#[case::c_nc_c_h(0x99, 0x34, 0x55, R8::C, false, 0xDF, true, true, false)]
#[case::c_c_c_h(0x99, 0x34, 0x55, R8::C, true, 0xDE, true, true, false)]
#[case::c_zero(0x99, 0x55, 0x54, R8::C, true, 0x00, false, false, true)]
#[case::d_nc_nc_nh(0x9A, 0x34, 0x21, R8::D, false, 0x13, false, false, false)]
#[case::d_c_nc_nh(0x9A, 0x34, 0x21, R8::D, true, 0x12, false, false, false)]
#[case::d_nc_c_nh(0x9A, 0x34, 0x51, R8::D, false, 0xE3, true, false, false)]
#[case::d_c_c_nh(0x9A, 0x34, 0x51, R8::D, true, 0xE2, true, false, false)]
#[case::d_nc_nc_h(0x9A, 0x34, 0x25, R8::D, false, 0x0F, false, true, false)]
#[case::d_c_nc_h(0x9A, 0x34, 0x25, R8::D, true, 0x0E, false, true, false)]
#[case::d_nc_c_h(0x9A, 0x34, 0x55, R8::D, false, 0xDF, true, true, false)]
#[case::d_c_c_h(0x9A, 0x34, 0x55, R8::D, true, 0xDE, true, true, false)]
#[case::d_zero(0x9A, 0x55, 0x54, R8::D, true, 0x00, false, false, true)]
#[case::e_nc_nc_nh(0x9B, 0x34, 0x21, R8::E, false, 0x13, false, false, false)]
#[case::e_c_nc_nh(0x9B, 0x34, 0x21, R8::E, true, 0x12, false, false, false)]
#[case::e_nc_c_nh(0x9B, 0x34, 0x51, R8::E, false, 0xE3, true, false, false)]
#[case::e_c_c_nh(0x9B, 0x34, 0x51, R8::E, true, 0xE2, true, false, false)]
#[case::e_nc_nc_h(0x9B, 0x34, 0x25, R8::E, false, 0x0F, false, true, false)]
#[case::e_c_nc_h(0x9B, 0x34, 0x25, R8::E, true, 0x0E, false, true, false)]
#[case::e_nc_c_h(0x9B, 0x34, 0x55, R8::E, false, 0xDF, true, true, false)]
#[case::e_c_c_h(0x9B, 0x34, 0x55, R8::E, true, 0xDE, true, true, false)]
#[case::e_zero(0x9B, 0x55, 0x54, R8::E, true, 0x00, false, false, true)]
#[case::h_nc_nc_nh(0x9C, 0x34, 0x21, R8::H, false, 0x13, false, false, false)]
#[case::h_c_nc_nh(0x9C, 0x34, 0x21, R8::H, true, 0x12, false, false, false)]
#[case::h_nc_c_nh(0x9C, 0x34, 0x51, R8::H, false, 0xE3, true, false, false)]
#[case::h_c_c_nh(0x9C, 0x34, 0x51, R8::H, true, 0xE2, true, false, false)]
#[case::h_nc_nc_h(0x9C, 0x34, 0x25, R8::H, false, 0x0F, false, true, false)]
#[case::h_c_nc_h(0x9C, 0x34, 0x25, R8::H, true, 0x0E, false, true, false)]
#[case::h_nc_c_h(0x9C, 0x34, 0x55, R8::H, false, 0xDF, true, true, false)]
#[case::h_c_c_h(0x9C, 0x34, 0x55, R8::H, true, 0xDE, true, true, false)]
#[case::h_zero(0x9C, 0x55, 0x54, R8::H, true, 0x00, false, false, true)]
#[case::l_nc_nc_nh(0x9D, 0x34, 0x21, R8::L, false, 0x13, false, false, false)]
#[case::l_c_nc_nh(0x9D, 0x34, 0x21, R8::L, true, 0x12, false, false, false)]
#[case::l_nc_c_nh(0x9D, 0x34, 0x51, R8::L, false, 0xE3, true, false, false)]
#[case::l_c_c_nh(0x9D, 0x34, 0x51, R8::L, true, 0xE2, true, false, false)]
#[case::l_nc_nc_h(0x9D, 0x34, 0x25, R8::L, false, 0x0F, false, true, false)]
#[case::l_c_nc_h(0x9D, 0x34, 0x25, R8::L, true, 0x0E, false, true, false)]
#[case::l_nc_c_h(0x9D, 0x34, 0x55, R8::L, false, 0xDF, true, true, false)]
#[case::l_c_c_h(0x9D, 0x34, 0x55, R8::L, true, 0xDE, true, true, false)]
#[case::l_zero(0x9D, 0x55, 0x54, R8::L, true, 0x00, false, false, true)]
#[case::hl_nc_nc_nh(0x9E, 0x34, 0x21, R8::HL, false, 0x13, false, false, false)]
#[case::hl_c_nc_nh(0x9E, 0x34, 0x21, R8::HL, true, 0x12, false, false, false)]
#[case::hl_nc_c_nh(0x9E, 0x34, 0x51, R8::HL, false, 0xE3, true, false, false)]
#[case::hl_c_c_nh(0x9E, 0x34, 0x51, R8::HL, true, 0xE2, true, false, false)]
#[case::hl_nc_nc_h(0x9E, 0x34, 0x25, R8::HL, false, 0x0F, false, true, false)]
#[case::hl_c_nc_h(0x9E, 0x34, 0x25, R8::HL, true, 0x0E, false, true, false)]
#[case::hl_nc_c_h(0x9E, 0x34, 0x55, R8::HL, false, 0xDF, true, true, false)]
#[case::hl_c_c_h(0x9E, 0x34, 0x55, R8::HL, true, 0xDE, true, true, false)]
#[case::hl_zero(0x9E, 0x55, 0x54, R8::HL, true, 0x00, false, false, true)]
#[case::a_zero(0x9F, 0x55, 0x55, R8::A, false, 0x00, false, false, true)]
#[case::a_underflow(0x9F, 0x55, 0x55, R8::A, true, 0xFF, true, true, false)]
fn test_sbc_r8(
    #[case] opcode: u8,
    #[case] value_a: u8,
    #[case] value_r: u8,
    #[case] register: R8,
    #[case] carry: bool,
    #[case] expected_value: u8,
    #[case] expected_carry: bool,
    #[case] expected_half_carry: bool,
    #[case] expected_zero: bool,
) {
    let mut mmu = MMU::builder().rom(0, opcode).write(0xCCCC, value_r).build();
    let mut cpu = CPU::builder()
        .a(value_a)
        .hl(0xCCCC)
        .f_carry(carry)
        .r8(register, value_r, &mut mmu)
        .build();
    let m = cpu.step(&mut mmu);

    if register == R8::HL {
        assert_eq!(m, 2);
    } else {
        assert_eq!(m, 1);
    }
    assert_eq!(cpu.get_pc(), 1);
    assert_eq!(cpu.get_a(), expected_value);
    assert!(cpu.get_f_subtract());
    assert_eq!(cpu.get_f_carry(), expected_carry);
    assert_eq!(cpu.get_f_half_carry(), expected_half_carry);
    assert_eq!(cpu.get_f_zero(), expected_zero);
}

/// SBC imm8 (0xDE)
#[rstest]
#[case::nc_nc_nh(0x34, 0x21, false, 0x13, false, false, false)]
#[case::c_nc_nh(0x34, 0x21, true, 0x12, false, false, false)]
#[case::nc_c_nh(0x34, 0x51, false, 0xE3, true, false, false)]
#[case::c_c_nh(0x34, 0x51, true, 0xE2, true, false, false)]
#[case::nc_nc_h(0x34, 0x25, false, 0x0F, false, true, false)]
#[case::c_nc_h(0x34, 0x25, true, 0x0E, false, true, false)]
#[case::nc_c_h(0x34, 0x55, false, 0xDF, true, true, false)]
#[case::c_c_h(0x34, 0x55, true, 0xDE, true, true, false)]
#[case::nc_zero(0x55, 0x55, false, 0x00, false, false, true)]
#[case::c_zero(0x56, 0x55, true, 0x00, false, false, true)]
fn test_sbc_imm8(
    #[case] value_a: u8,
    #[case] imm: u8,
    #[case] carry: bool,
    #[case] expected_value: u8,
    #[case] expected_carry: bool,
    #[case] expected_half_carry: bool,
    #[case] expected_zero: bool,
) {
    let mut mmu = MMU::builder().rom(0, 0xDE).rom(1, imm).build();
    let mut cpu = CPU::builder().a(value_a).f_carry(carry).build();
    let m = cpu.step(&mut mmu);

    assert_eq!(m, 2);
    assert_eq!(cpu.get_pc(), 2);
    assert_eq!(cpu.get_a(), expected_value);
    assert!(cpu.get_f_subtract());
    assert_eq!(cpu.get_f_carry(), expected_carry);
    assert_eq!(cpu.get_f_half_carry(), expected_half_carry);
    assert_eq!(cpu.get_f_zero(), expected_zero);
}

/// XOR r8
#[rstest]
#[case::b_nz(0xA8, 0b1010_1111, 0b1010_1010, R8::B, 0b0000_0101, false)]
#[case::b_z(0xA8, 0b1010_0000, 0b1010_0000, R8::B, 0b0000_0000, true)]
#[case::c_nz(0xA9, 0b1010_1111, 0b1010_1010, R8::C, 0b0000_0101, false)]
#[case::c_z(0xA9, 0b1010_0000, 0b1010_0000, R8::C, 0b0000_0000, true)]
#[case::d_nz(0xAA, 0b1010_1111, 0b1010_1010, R8::D, 0b0000_0101, false)]
#[case::d_z(0xAA, 0b1010_0000, 0b1010_0000, R8::D, 0b0000_0000, true)]
#[case::e_nz(0xAB, 0b1010_1111, 0b1010_1010, R8::E, 0b0000_0101, false)]
#[case::e_z(0xAB, 0b1010_0000, 0b1010_0000, R8::E, 0b0000_0000, true)]
#[case::h_nz(0xAC, 0b1010_1111, 0b1010_1010, R8::H, 0b0000_0101, false)]
#[case::h_z(0xAC, 0b1010_0000, 0b1010_0000, R8::H, 0b0000_0000, true)]
#[case::l_nz(0xAD, 0b1010_1111, 0b1010_1010, R8::L, 0b0000_0101, false)]
#[case::l_z(0xAD, 0b1010_0000, 0b1010_0000, R8::L, 0b0000_0000, true)]
#[case::hl_nz(0xAE, 0b1010_1111, 0b1010_1010, R8::HL, 0b0000_0101, false)]
#[case::hl_z(0xAE, 0b1010_0000, 0b1010_0000, R8::HL, 0b0000_0000, true)]
#[case::a_z(0xAF, 0b1010_0000, 0b1010_0000, R8::A, 0b0000_0000, true)]
fn test_xor_r8(
    #[case] opcode: u8,
    #[case] value_a: u8,
    #[case] value_r: u8,
    #[case] register: R8,
    #[case] expected_value: u8,
    #[case] expected_zero: bool,
) {
    let mut mmu = MMU::builder().rom(0, opcode).write(0xCCCC, value_r).build();
    let mut cpu = CPU::builder()
        .a(value_a)
        .hl(0xCCCC)
        .r8(register, value_r, &mut mmu)
        .build();
    let m = cpu.step(&mut mmu);

    if register == R8::HL {
        assert_eq!(m, 2);
    } else {
        assert_eq!(m, 1);
    }
    assert_eq!(cpu.get_pc(), 1);
    assert_eq!(cpu.get_a(), expected_value);
    assert!(!cpu.get_f_subtract());
    assert!(!cpu.get_f_carry());
    assert!(!cpu.get_f_half_carry());
    assert_eq!(cpu.get_f_zero(), expected_zero);
}

/// XOR imm8 (0xEE)
#[rstest]
#[case::nz(0b1010_1111, 0b1010_1010, 0b0000_0101, false)]
#[case::z(0b1010_0000, 0b1010_0000, 0b0000_0000, true)]
fn test_xor_imm8(
    #[case] value_a: u8,
    #[case] imm: u8,
    #[case] expected_value: u8,
    #[case] expected_zero: bool,
) {
    let mut mmu = MMU::builder().rom(0, 0xEE).rom(1, imm).build();
    let mut cpu = CPU::builder().a(value_a).build();
    let m = cpu.step(&mut mmu);

    assert_eq!(m, 2);
    assert_eq!(cpu.get_pc(), 2);
    assert_eq!(cpu.get_a(), expected_value);
    assert!(!cpu.get_f_subtract());
    assert!(!cpu.get_f_carry());
    assert!(!cpu.get_f_half_carry());
    assert_eq!(cpu.get_f_zero(), expected_zero);
}

// Prefixed Instructions

/// BIT u3, r8
#[rstest]
#[case::b_0_true(0x40, 0b1111_1110, R8::B, 2, true)]
#[case::b_0_false(0x40, 0b1111_1111, R8::B, 2, false)]
#[case::b_1_true(0x48, 0b1111_1101, R8::B, 2, true)]
#[case::b_1_false(0x48, 0b1111_1111, R8::B, 2, false)]
#[case::b_2_true(0x50, 0b1111_1011, R8::B, 2, true)]
#[case::b_2_false(0x50, 0b1111_1111, R8::B, 2, false)]
#[case::b_3_true(0x58, 0b1111_0111, R8::B, 2, true)]
#[case::b_3_false(0x58, 0b1111_1111, R8::B, 2, false)]
#[case::b_4_true(0x60, 0b1110_1111, R8::B, 2, true)]
#[case::b_4_false(0x60, 0b1111_1111, R8::B, 2, false)]
#[case::b_5_true(0x68, 0b1101_1111, R8::B, 2, true)]
#[case::b_5_false(0x68, 0b1111_1111, R8::B, 2, false)]
#[case::b_6_true(0x70, 0b1011_1111, R8::B, 2, true)]
#[case::b_6_false(0x70, 0b1111_1111, R8::B, 2, false)]
#[case::b_7_true(0x78, 0b0111_1111, R8::B, 2, true)]
#[case::b_7_false(0x78, 0b1111_1111, R8::B, 2, false)]
#[case::c_0_true(0x41, 0b1111_1110, R8::C, 2, true)]
#[case::c_0_false(0x41, 0b1111_1111, R8::C, 2, false)]
#[case::c_1_true(0x49, 0b1111_1101, R8::C, 2, true)]
#[case::c_1_false(0x49, 0b1111_1111, R8::C, 2, false)]
#[case::c_2_true(0x51, 0b1111_1011, R8::C, 2, true)]
#[case::c_2_false(0x51, 0b1111_1111, R8::C, 2, false)]
#[case::c_3_true(0x59, 0b1111_0111, R8::C, 2, true)]
#[case::c_3_false(0x59, 0b1111_1111, R8::C, 2, false)]
#[case::c_4_true(0x61, 0b1110_1111, R8::C, 2, true)]
#[case::c_4_false(0x61, 0b1111_1111, R8::C, 2, false)]
#[case::c_5_true(0x69, 0b1101_1111, R8::C, 2, true)]
#[case::c_5_false(0x69, 0b1111_1111, R8::C, 2, false)]
#[case::c_6_true(0x71, 0b1011_1111, R8::C, 2, true)]
#[case::c_6_false(0x71, 0b1111_1111, R8::C, 2, false)]
#[case::c_7_true(0x79, 0b0111_1111, R8::C, 2, true)]
#[case::c_7_false(0x79, 0b1111_1111, R8::C, 2, false)]
#[case::d_0_true(0x42, 0b1111_1110, R8::D, 2, true)]
#[case::d_0_false(0x42, 0b1111_1111, R8::D, 2, false)]
#[case::d_1_true(0x4A, 0b1111_1101, R8::D, 2, true)]
#[case::d_1_false(0x4A, 0b1111_1111, R8::D, 2, false)]
#[case::d_2_true(0x52, 0b1111_1011, R8::D, 2, true)]
#[case::d_2_false(0x52, 0b1111_1111, R8::D, 2, false)]
#[case::d_3_true(0x5A, 0b1111_0111, R8::D, 2, true)]
#[case::d_3_false(0x5A, 0b1111_1111, R8::D, 2, false)]
#[case::d_4_true(0x62, 0b1110_1111, R8::D, 2, true)]
#[case::d_4_false(0x62, 0b1111_1111, R8::D, 2, false)]
#[case::d_5_true(0x6A, 0b1101_1111, R8::D, 2, true)]
#[case::d_5_false(0x6A, 0b1111_1111, R8::D, 2, false)]
#[case::d_6_true(0x72, 0b1011_1111, R8::D, 2, true)]
#[case::d_6_false(0x72, 0b1111_1111, R8::D, 2, false)]
#[case::d_7_true(0x7A, 0b0111_1111, R8::D, 2, true)]
#[case::d_7_false(0x7A, 0b1111_1111, R8::D, 2, false)]
#[case::e_0_true(0x43, 0b1111_1110, R8::E, 2, true)]
#[case::e_0_false(0x43, 0b1111_1111, R8::E, 2, false)]
#[case::e_1_true(0x4B, 0b1111_1101, R8::E, 2, true)]
#[case::e_1_false(0x4B, 0b1111_1111, R8::E, 2, false)]
#[case::e_2_true(0x53, 0b1111_1011, R8::E, 2, true)]
#[case::e_2_false(0x53, 0b1111_1111, R8::E, 2, false)]
#[case::e_3_true(0x5B, 0b1111_0111, R8::E, 2, true)]
#[case::e_3_false(0x5B, 0b1111_1111, R8::E, 2, false)]
#[case::e_4_true(0x63, 0b1110_1111, R8::E, 2, true)]
#[case::e_4_false(0x63, 0b1111_1111, R8::E, 2, false)]
#[case::e_5_true(0x6B, 0b1101_1111, R8::E, 2, true)]
#[case::e_5_false(0x6B, 0b1111_1111, R8::E, 2, false)]
#[case::e_6_true(0x73, 0b1011_1111, R8::E, 2, true)]
#[case::e_6_false(0x73, 0b1111_1111, R8::E, 2, false)]
#[case::e_7_true(0x7B, 0b0111_1111, R8::E, 2, true)]
#[case::e_7_false(0x7B, 0b1111_1111, R8::E, 2, false)]
#[case::h_0_true(0x44, 0b1111_1110, R8::H, 2, true)]
#[case::h_0_false(0x44, 0b1111_1111, R8::H, 2, false)]
#[case::h_1_true(0x4C, 0b1111_1101, R8::H, 2, true)]
#[case::h_1_false(0x4C, 0b1111_1111, R8::H, 2, false)]
#[case::h_2_true(0x54, 0b1111_1011, R8::H, 2, true)]
#[case::h_2_false(0x54, 0b1111_1111, R8::H, 2, false)]
#[case::h_3_true(0x5C, 0b1111_0111, R8::H, 2, true)]
#[case::h_3_false(0x5C, 0b1111_1111, R8::H, 2, false)]
#[case::h_4_true(0x64, 0b1110_1111, R8::H, 2, true)]
#[case::h_4_false(0x64, 0b1111_1111, R8::H, 2, false)]
#[case::h_5_true(0x6C, 0b1101_1111, R8::H, 2, true)]
#[case::h_5_false(0x6C, 0b1111_1111, R8::H, 2, false)]
#[case::h_6_true(0x74, 0b1011_1111, R8::H, 2, true)]
#[case::h_6_false(0x74, 0b1111_1111, R8::H, 2, false)]
#[case::h_7_true(0x7C, 0b0111_1111, R8::H, 2, true)]
#[case::h_7_false(0x7C, 0b1111_1111, R8::H, 2, false)]
#[case::l_0_true(0x45, 0b1111_1110, R8::L, 2, true)]
#[case::l_0_false(0x45, 0b1111_1111, R8::L, 2, false)]
#[case::l_1_true(0x4D, 0b1111_1101, R8::L, 2, true)]
#[case::l_1_false(0x4D, 0b1111_1111, R8::L, 2, false)]
#[case::l_2_true(0x55, 0b1111_1011, R8::L, 2, true)]
#[case::l_2_false(0x55, 0b1111_1111, R8::L, 2, false)]
#[case::l_3_true(0x5D, 0b1111_0111, R8::L, 2, true)]
#[case::l_3_false(0x5D, 0b1111_1111, R8::L, 2, false)]
#[case::l_4_true(0x65, 0b1110_1111, R8::L, 2, true)]
#[case::l_4_false(0x65, 0b1111_1111, R8::L, 2, false)]
#[case::l_5_true(0x6D, 0b1101_1111, R8::L, 2, true)]
#[case::l_5_false(0x6D, 0b1111_1111, R8::L, 2, false)]
#[case::l_6_true(0x75, 0b1011_1111, R8::L, 2, true)]
#[case::l_6_false(0x75, 0b1111_1111, R8::L, 2, false)]
#[case::l_7_true(0x7D, 0b0111_1111, R8::L, 2, true)]
#[case::l_7_false(0x7D, 0b1111_1111, R8::L, 2, false)]
#[case::hl_0_true(0x46, 0b1111_1110, R8::HL, 3, true)]
#[case::hl_0_false(0x46, 0b1111_1111, R8::HL, 3, false)]
#[case::hl_1_true(0x4E, 0b1111_1101, R8::HL, 3, true)]
#[case::hl_1_false(0x4E, 0b1111_1111, R8::HL, 3, false)]
#[case::hl_2_true(0x56, 0b1111_1011, R8::HL, 3, true)]
#[case::hl_2_false(0x56, 0b1111_1111, R8::HL, 3, false)]
#[case::hl_3_true(0x5E, 0b1111_0111, R8::HL, 3, true)]
#[case::hl_3_false(0x5E, 0b1111_1111, R8::HL, 3, false)]
#[case::hl_4_true(0x66, 0b1110_1111, R8::HL, 3, true)]
#[case::hl_4_false(0x66, 0b1111_1111, R8::HL, 3, false)]
#[case::hl_5_true(0x6E, 0b1101_1111, R8::HL, 3, true)]
#[case::hl_5_false(0x6E, 0b1111_1111, R8::HL, 3, false)]
#[case::hl_6_true(0x76, 0b1011_1111, R8::HL, 3, true)]
#[case::hl_6_false(0x76, 0b1111_1111, R8::HL, 3, false)]
#[case::hl_7_true(0x7E, 0b0111_1111, R8::HL, 3, true)]
#[case::hl_7_false(0x7E, 0b1111_1111, R8::HL, 3, false)]
#[case::a_0_true(0x47, 0b1111_1110, R8::A, 2, true)]
#[case::a_0_false(0x47, 0b1111_1111, R8::A, 2, false)]
#[case::a_1_true(0x4F, 0b1111_1101, R8::A, 2, true)]
#[case::a_1_false(0x4F, 0b1111_1111, R8::A, 2, false)]
#[case::a_2_true(0x57, 0b1111_1011, R8::A, 2, true)]
#[case::a_2_false(0x57, 0b1111_1111, R8::A, 2, false)]
#[case::a_3_true(0x5F, 0b1111_0111, R8::A, 2, true)]
#[case::a_3_false(0x5F, 0b1111_1111, R8::A, 2, false)]
#[case::a_4_true(0x67, 0b1110_1111, R8::A, 2, true)]
#[case::a_4_false(0x67, 0b1111_1111, R8::A, 2, false)]
#[case::a_5_true(0x6F, 0b1101_1111, R8::A, 2, true)]
#[case::a_5_false(0x6F, 0b1111_1111, R8::A, 2, false)]
#[case::a_6_true(0x77, 0b1011_1111, R8::A, 2, true)]
#[case::a_6_false(0x77, 0b1111_1111, R8::A, 2, false)]
#[case::a_7_true(0x7F, 0b0111_1111, R8::A, 2, true)]
#[case::a_7_false(0x7F, 0b1111_1111, R8::A, 2, false)]
fn test_bit_check(
    #[case] opcode: u8,
    #[case] value: u8,
    #[case] register: R8,
    #[case] expected_m: u8,
    #[case] expected_zero: bool,
) {
    let mut mmu = MMU::builder()
        .rom(0, PREFIX_INSTRUCTION_BYTE)
        .rom(1, opcode)
        .write(0xCCCC, value)
        .build();
    let mut cpu = CPU::builder()
        .hl(0xCCCC)
        .r8(register, value, &mut mmu)
        .f_zero(false)
        .build();
    let m = cpu.step(&mut mmu);

    assert_eq!(m, expected_m);
    assert_eq!(cpu.get_pc(), 2);
    assert_eq!(cpu.get_f_zero(), expected_zero);
    assert!(!cpu.get_f_subtract());
    assert!(cpu.get_f_half_carry());
    assert!(!cpu.get_f_carry());
}

/// RES u3, r8
#[rstest]
#[case::b_0(0x80, R8::B, 0b1111_1110, 2)]
#[case::c_0(0x81, R8::C, 0b1111_1110, 2)]
#[case::d_0(0x82, R8::D, 0b1111_1110, 2)]
#[case::e_0(0x83, R8::E, 0b1111_1110, 2)]
#[case::h_0(0x84, R8::H, 0b1111_1110, 2)]
#[case::l_0(0x85, R8::L, 0b1111_1110, 2)]
#[case::hl_0(0x86, R8::HL, 0b1111_1110, 4)]
#[case::a_0(0x87, R8::A, 0b1111_1110, 2)]
#[case::b_1(0x88, R8::B, 0b1111_1101, 2)]
#[case::c_1(0x89, R8::C, 0b1111_1101, 2)]
#[case::d_1(0x8A, R8::D, 0b1111_1101, 2)]
#[case::e_1(0x8B, R8::E, 0b1111_1101, 2)]
#[case::h_1(0x8C, R8::H, 0b1111_1101, 2)]
#[case::l_1(0x8D, R8::L, 0b1111_1101, 2)]
#[case::hl_1(0x8E, R8::HL, 0b1111_1101, 4)]
#[case::a_1(0x8F, R8::A, 0b1111_1101, 2)]
#[case::b_2(0x90, R8::B, 0b1111_1011, 2)]
#[case::c_2(0x91, R8::C, 0b1111_1011, 2)]
#[case::d_2(0x92, R8::D, 0b1111_1011, 2)]
#[case::e_2(0x93, R8::E, 0b1111_1011, 2)]
#[case::h_2(0x94, R8::H, 0b1111_1011, 2)]
#[case::l_2(0x95, R8::L, 0b1111_1011, 2)]
#[case::hl_2(0x96, R8::HL, 0b1111_1011, 4)]
#[case::a_2(0x97, R8::A, 0b1111_1011, 2)]
#[case::b_3(0x98, R8::B, 0b1111_0111, 2)]
#[case::c_3(0x99, R8::C, 0b1111_0111, 2)]
#[case::d_3(0x9A, R8::D, 0b1111_0111, 2)]
#[case::e_3(0x9B, R8::E, 0b1111_0111, 2)]
#[case::h_3(0x9C, R8::H, 0b1111_0111, 2)]
#[case::l_3(0x9D, R8::L, 0b1111_0111, 2)]
#[case::hl_3(0x9E, R8::HL, 0b1111_0111, 4)]
#[case::a_3(0x9F, R8::A, 0b1111_0111, 2)]
#[case::b_4(0xA0, R8::B, 0b1110_1111, 2)]
#[case::c_4(0xA1, R8::C, 0b1110_1111, 2)]
#[case::d_4(0xA2, R8::D, 0b1110_1111, 2)]
#[case::e_4(0xA3, R8::E, 0b1110_1111, 2)]
#[case::h_4(0xA4, R8::H, 0b1110_1111, 2)]
#[case::l_4(0xA5, R8::L, 0b1110_1111, 2)]
#[case::hl_4(0xA6, R8::HL, 0b1110_1111, 4)]
#[case::a_4(0xA7, R8::A, 0b1110_1111, 2)]
#[case::b_5(0xA8, R8::B, 0b1101_1111, 2)]
#[case::c_5(0xA9, R8::C, 0b1101_1111, 2)]
#[case::d_5(0xAA, R8::D, 0b1101_1111, 2)]
#[case::e_5(0xAB, R8::E, 0b1101_1111, 2)]
#[case::h_5(0xAC, R8::H, 0b1101_1111, 2)]
#[case::l_5(0xAD, R8::L, 0b1101_1111, 2)]
#[case::hl_5(0xAE, R8::HL, 0b1101_1111, 4)]
#[case::a_5(0xAF, R8::A, 0b1101_1111, 2)]
#[case::b_6(0xB0, R8::B, 0b1011_1111, 2)]
#[case::c_6(0xB1, R8::C, 0b1011_1111, 2)]
#[case::d_6(0xB2, R8::D, 0b1011_1111, 2)]
#[case::e_6(0xB3, R8::E, 0b1011_1111, 2)]
#[case::h_6(0xB4, R8::H, 0b1011_1111, 2)]
#[case::l_6(0xB5, R8::L, 0b1011_1111, 2)]
#[case::hl_6(0xB6, R8::HL, 0b1011_1111, 4)]
#[case::a_6(0xB7, R8::A, 0b1011_1111, 2)]
#[case::b_7(0xB8, R8::B, 0b0111_1111, 2)]
#[case::c_7(0xB9, R8::C, 0b0111_1111, 2)]
#[case::d_7(0xBA, R8::D, 0b0111_1111, 2)]
#[case::e_7(0xBB, R8::E, 0b0111_1111, 2)]
#[case::h_7(0xBC, R8::H, 0b0111_1111, 2)]
#[case::l_7(0xBD, R8::L, 0b0111_1111, 2)]
#[case::hl_7(0xBE, R8::HL, 0b0111_1111, 4)]
#[case::a_7(0xBF, R8::A, 0b0111_1111, 2)]
fn test_bit_reset(
    #[case] opcode: u8,
    #[case] register: R8,
    #[case] expected_value: u8,
    #[case] expected_m: u8,
) {
    let mut mmu = MMU::builder()
        .rom(0, PREFIX_INSTRUCTION_BYTE)
        .rom(1, opcode)
        .write(0xCCCC, 0xFF)
        .build();
    let mut cpu = CPU::builder()
        .hl(0xCCCC)
        .r8(register, 0xFF, &mut mmu)
        .build();
    let m = cpu.step(&mut mmu);

    assert_eq!(m, expected_m);
    assert_eq!(cpu.get_pc(), 2);
    assert_eq!(cpu.get_r8(register, &mmu), expected_value);
}

/// RES u3, r8
#[rstest]
#[case::b_0(0xC0, R8::B, 0b0000_0001, 2)]
#[case::c_0(0xC1, R8::C, 0b0000_0001, 2)]
#[case::d_0(0xC2, R8::D, 0b0000_0001, 2)]
#[case::e_0(0xC3, R8::E, 0b0000_0001, 2)]
#[case::h_0(0xC4, R8::H, 0b0000_0001, 2)]
#[case::l_0(0xC5, R8::L, 0b0000_0001, 2)]
#[case::hl_0(0xC6, R8::HL, 0b0000_0001, 4)]
#[case::a_0(0xC7, R8::A, 0b0000_0001, 2)]
#[case::b_1(0xC8, R8::B, 0b0000_0010, 2)]
#[case::c_1(0xC9, R8::C, 0b0000_0010, 2)]
#[case::d_1(0xCA, R8::D, 0b0000_0010, 2)]
#[case::e_1(0xCB, R8::E, 0b0000_0010, 2)]
#[case::h_1(0xCC, R8::H, 0b0000_0010, 2)]
#[case::l_1(0xCD, R8::L, 0b0000_0010, 2)]
#[case::hl_1(0xCE, R8::HL, 0b0000_0010, 4)]
#[case::a_1(0xCF, R8::A, 0b0000_0010, 2)]
#[case::b_2(0xD0, R8::B, 0b0000_0100, 2)]
#[case::c_2(0xD1, R8::C, 0b0000_0100, 2)]
#[case::d_2(0xD2, R8::D, 0b0000_0100, 2)]
#[case::e_2(0xD3, R8::E, 0b0000_0100, 2)]
#[case::h_2(0xD4, R8::H, 0b0000_0100, 2)]
#[case::l_2(0xD5, R8::L, 0b0000_0100, 2)]
#[case::hl_2(0xD6, R8::HL, 0b0000_0100, 4)]
#[case::a_2(0xD7, R8::A, 0b0000_0100, 2)]
#[case::b_3(0xD8, R8::B, 0b0000_1000, 2)]
#[case::c_3(0xD9, R8::C, 0b0000_1000, 2)]
#[case::d_3(0xDA, R8::D, 0b0000_1000, 2)]
#[case::e_3(0xDB, R8::E, 0b0000_1000, 2)]
#[case::h_3(0xDC, R8::H, 0b0000_1000, 2)]
#[case::l_3(0xDD, R8::L, 0b0000_1000, 2)]
#[case::hl_3(0xDE, R8::HL, 0b0000_1000, 4)]
#[case::a_3(0xDF, R8::A, 0b0000_1000, 2)]
#[case::b_4(0xE0, R8::B, 0b0001_0000, 2)]
#[case::c_4(0xE1, R8::C, 0b0001_0000, 2)]
#[case::d_4(0xE2, R8::D, 0b0001_0000, 2)]
#[case::e_4(0xE3, R8::E, 0b0001_0000, 2)]
#[case::h_4(0xE4, R8::H, 0b0001_0000, 2)]
#[case::l_4(0xE5, R8::L, 0b0001_0000, 2)]
#[case::hl_4(0xE6, R8::HL, 0b0001_0000, 4)]
#[case::a_4(0xE7, R8::A, 0b0001_0000, 2)]
#[case::b_5(0xE8, R8::B, 0b0010_0000, 2)]
#[case::c_5(0xE9, R8::C, 0b0010_0000, 2)]
#[case::d_5(0xEA, R8::D, 0b0010_0000, 2)]
#[case::e_5(0xEB, R8::E, 0b0010_0000, 2)]
#[case::h_5(0xEC, R8::H, 0b0010_0000, 2)]
#[case::l_5(0xED, R8::L, 0b0010_0000, 2)]
#[case::hl_5(0xEE, R8::HL, 0b0010_0000, 4)]
#[case::a_5(0xEF, R8::A, 0b0010_0000, 2)]
#[case::b_6(0xF0, R8::B, 0b0100_0000, 2)]
#[case::c_6(0xF1, R8::C, 0b0100_0000, 2)]
#[case::d_6(0xF2, R8::D, 0b0100_0000, 2)]
#[case::e_6(0xF3, R8::E, 0b0100_0000, 2)]
#[case::h_6(0xF4, R8::H, 0b0100_0000, 2)]
#[case::l_6(0xF5, R8::L, 0b0100_0000, 2)]
#[case::hl_6(0xF6, R8::HL, 0b0100_0000, 4)]
#[case::a_6(0xF7, R8::A, 0b0100_0000, 2)]
#[case::b_7(0xF8, R8::B, 0b1000_0000, 2)]
#[case::c_7(0xF9, R8::C, 0b1000_0000, 2)]
#[case::d_7(0xFA, R8::D, 0b1000_0000, 2)]
#[case::e_7(0xFB, R8::E, 0b1000_0000, 2)]
#[case::h_7(0xFC, R8::H, 0b1000_0000, 2)]
#[case::l_7(0xFD, R8::L, 0b1000_0000, 2)]
#[case::hl_7(0xFE, R8::HL, 0b1000_0000, 4)]
#[case::a_7(0xFF, R8::A, 0b1000_0000, 2)]
fn test_bit_set(
    #[case] opcode: u8,
    #[case] register: R8,
    #[case] expected_value: u8,
    #[case] expected_m: u8,
) {
    let mut mmu = MMU::builder()
        .rom(0, PREFIX_INSTRUCTION_BYTE)
        .rom(1, opcode)
        .write(0xCCCC, 0)
        .build();
    let mut cpu = CPU::builder().hl(0xCCCC).r8(register, 0, &mut mmu).build();
    let m = cpu.step(&mut mmu);

    assert_eq!(m, expected_m);
    assert_eq!(cpu.get_pc(), 2);
    assert_eq!(cpu.get_r8(register, &mmu), expected_value);
}

/// RL r8 & RR r8
#[rstest]
#[case::left_b_nc_nc(0x10, R8::B, 0b0000_1000, false, 2, 0b0001_0000, false, false)]
#[case::left_b_c_nc(0x10, R8::B, 0b0000_1000, true, 2, 0b0001_0001, false, false)]
#[case::left_b_nc_c(0x10, R8::B, 0b1000_1000, false, 2, 0b0001_0000, true, false)]
#[case::left_b_c_c(0x10, R8::B, 0b1000_1000, true, 2, 0b0001_0001, true, false)]
#[case::left_b_zero(0x10, R8::B, 0, false, 2, 0, false, true)]
#[case::left_c_nc_nc(0x11, R8::C, 0b0000_1000, false, 2, 0b0001_0000, false, false)]
#[case::left_c_c_nc(0x11, R8::C, 0b0000_1000, true, 2, 0b0001_0001, false, false)]
#[case::left_c_nc_c(0x11, R8::C, 0b1000_1000, false, 2, 0b0001_0000, true, false)]
#[case::left_c_c_c(0x11, R8::C, 0b1000_1000, true, 2, 0b0001_0001, true, false)]
#[case::left_c_zero(0x11, R8::C, 0, false, 2, 0, false, true)]
#[case::left_d_nc_nc(0x12, R8::D, 0b0000_1000, false, 2, 0b0001_0000, false, false)]
#[case::left_d_c_nc(0x12, R8::D, 0b0000_1000, true, 2, 0b0001_0001, false, false)]
#[case::left_d_nc_c(0x12, R8::D, 0b1000_1000, false, 2, 0b0001_0000, true, false)]
#[case::left_d_c_c(0x12, R8::D, 0b1000_1000, true, 2, 0b0001_0001, true, false)]
#[case::left_d_zero(0x12, R8::D, 0, false, 2, 0, false, true)]
#[case::left_e_nc_nc(0x13, R8::E, 0b0000_1000, false, 2, 0b0001_0000, false, false)]
#[case::left_e_c_nc(0x13, R8::E, 0b0000_1000, true, 2, 0b0001_0001, false, false)]
#[case::left_e_nc_c(0x13, R8::E, 0b1000_1000, false, 2, 0b0001_0000, true, false)]
#[case::left_e_c_c(0x13, R8::E, 0b1000_1000, true, 2, 0b0001_0001, true, false)]
#[case::left_e_zero(0x13, R8::E, 0, false, 2, 0, false, true)]
#[case::left_h_nc_nc(0x14, R8::H, 0b0000_1000, false, 2, 0b0001_0000, false, false)]
#[case::left_h_c_nc(0x14, R8::H, 0b0000_1000, true, 2, 0b0001_0001, false, false)]
#[case::left_h_nc_c(0x14, R8::H, 0b1000_1000, false, 2, 0b0001_0000, true, false)]
#[case::left_h_c_c(0x14, R8::H, 0b1000_1000, true, 2, 0b0001_0001, true, false)]
#[case::left_h_zero(0x14, R8::H, 0, false, 2, 0, false, true)]
#[case::left_l_nc_nc(0x15, R8::L, 0b0000_1000, false, 2, 0b0001_0000, false, false)]
#[case::left_l_c_nc(0x15, R8::L, 0b0000_1000, true, 2, 0b0001_0001, false, false)]
#[case::left_l_nc_c(0x15, R8::L, 0b1000_1000, false, 2, 0b0001_0000, true, false)]
#[case::left_l_c_c(0x15, R8::L, 0b1000_1000, true, 2, 0b0001_0001, true, false)]
#[case::left_l_zero(0x15, R8::L, 0, false, 2, 0, false, true)]
#[case::left_hl_nc_nc(0x16, R8::HL, 0b0000_1000, false, 4, 0b0001_0000, false, false)]
#[case::left_hl_c_nc(0x16, R8::HL, 0b0000_1000, true, 4, 0b0001_0001, false, false)]
#[case::left_hl_nc_c(0x16, R8::HL, 0b1000_1000, false, 4, 0b0001_0000, true, false)]
#[case::left_hl_c_c(0x16, R8::HL, 0b1000_1000, true, 4, 0b0001_0001, true, false)]
#[case::left_hl_zero(0x16, R8::HL, 0, false, 4, 0, false, true)]
#[case::left_a_nc_nc(0x17, R8::A, 0b0000_1000, false, 2, 0b0001_0000, false, false)]
#[case::left_a_c_nc(0x17, R8::A, 0b0000_1000, true, 2, 0b0001_0001, false, false)]
#[case::left_a_nc_c(0x17, R8::A, 0b1000_1000, false, 2, 0b0001_0000, true, false)]
#[case::left_a_c_c(0x17, R8::A, 0b1000_1000, true, 2, 0b0001_0001, true, false)]
#[case::left_a_zero(0x17, R8::A, 0, false, 2, 0, false, true)]
#[case::right_b_nc_nc(0x18, R8::B, 0b0000_1000, false, 2, 0b0000_0100, false, false)]
#[case::right_b_c_nc(0x18, R8::B, 0b0000_1000, true, 2, 0b1000_0100, false, false)]
#[case::right_b_nc_c(0x18, R8::B, 0b0010_0001, false, 2, 0b0001_0000, true, false)]
#[case::right_b_c_c(0x18, R8::B, 0b0000_1001, true, 2, 0b1000_0100, true, false)]
#[case::right_b_zero(0x18, R8::B, 0, false, 2, 0, false, true)]
#[case::right_c_nc_nc(0x19, R8::C, 0b0000_1000, false, 2, 0b0000_0100, false, false)]
#[case::right_c_c_nc(0x19, R8::C, 0b0000_1000, true, 2, 0b1000_0100, false, false)]
#[case::right_c_nc_c(0x19, R8::C, 0b0010_0001, false, 2, 0b0001_0000, true, false)]
#[case::right_c_c_c(0x19, R8::C, 0b0000_1001, true, 2, 0b1000_0100, true, false)]
#[case::right_c_zero(0x19, R8::C, 0, false, 2, 0, false, true)]
#[case::right_d_nc_nc(0x1A, R8::D, 0b0000_1000, false, 2, 0b0000_0100, false, false)]
#[case::right_d_c_nc(0x1A, R8::D, 0b0000_1000, true, 2, 0b1000_0100, false, false)]
#[case::right_d_nc_c(0x1A, R8::D, 0b0010_0001, false, 2, 0b0001_0000, true, false)]
#[case::right_d_c_c(0x1A, R8::D, 0b0000_1001, true, 2, 0b1000_0100, true, false)]
#[case::right_d_zero(0x1A, R8::D, 0, false, 2, 0, false, true)]
#[case::right_e_nc_nc(0x1B, R8::E, 0b0000_1000, false, 2, 0b0000_0100, false, false)]
#[case::right_e_c_nc(0x1B, R8::E, 0b0000_1000, true, 2, 0b1000_0100, false, false)]
#[case::right_e_nc_c(0x1B, R8::E, 0b0010_0001, false, 2, 0b0001_0000, true, false)]
#[case::right_e_c_c(0x1B, R8::E, 0b0000_1001, true, 2, 0b1000_0100, true, false)]
#[case::right_e_zero(0x1B, R8::E, 0, false, 2, 0, false, true)]
#[case::right_h_nc_nc(0x1C, R8::H, 0b0000_1000, false, 2, 0b0000_0100, false, false)]
#[case::right_h_c_nc(0x1C, R8::H, 0b0000_1000, true, 2, 0b1000_0100, false, false)]
#[case::right_h_nc_c(0x1C, R8::H, 0b0010_0001, false, 2, 0b0001_0000, true, false)]
#[case::right_h_c_c(0x1C, R8::H, 0b0000_1001, true, 2, 0b1000_0100, true, false)]
#[case::right_h_zero(0x1C, R8::H, 0, false, 2, 0, false, true)]
#[case::right_l_nc_nc(0x1D, R8::L, 0b0000_1000, false, 2, 0b0000_0100, false, false)]
#[case::right_l_c_nc(0x1D, R8::L, 0b0000_1000, true, 2, 0b1000_0100, false, false)]
#[case::right_l_nc_c(0x1D, R8::L, 0b0010_0001, false, 2, 0b0001_0000, true, false)]
#[case::right_l_c_c(0x1D, R8::L, 0b0000_1001, true, 2, 0b1000_0100, true, false)]
#[case::right_l_zero(0x1D, R8::L, 0, false, 2, 0, false, true)]
#[case::right_hl_nc_nc(0x1E, R8::HL, 0b0000_1000, false, 4, 0b0000_0100, false, false)]
#[case::right_hl_c_nc(0x1E, R8::HL, 0b0000_1000, true, 4, 0b1000_0100, false, false)]
#[case::right_hl_nc_c(0x1E, R8::HL, 0b0010_0001, false, 4, 0b0001_0000, true, false)]
#[case::right_hl_c_c(0x1E, R8::HL, 0b0000_1001, true, 4, 0b1000_0100, true, false)]
#[case::right_hl_zero(0x1E, R8::HL, 0, false, 4, 0, false, true)]
#[case::right_a_nc_nc(0x1F, R8::A, 0b0000_1000, false, 2, 0b0000_0100, false, false)]
#[case::right_a_c_nc(0x1F, R8::A, 0b0000_1000, true, 2, 0b1000_0100, false, false)]
#[case::right_a_nc_c(0x1F, R8::A, 0b0010_0001, false, 2, 0b0001_0000, true, false)]
#[case::right_a_c_c(0x1F, R8::A, 0b0000_1001, true, 2, 0b1000_0100, true, false)]
#[case::right_a_zero(0x1F, R8::A, 0, false, 2, 0, false, true)]
fn test_rl_rr_r8(
    #[case] opcode: u8,
    #[case] register: R8,
    #[case] value: u8,
    #[case] carry: bool,
    #[case] expected_m: u8,
    #[case] expected_value: u8,
    #[case] expected_carry: bool,
    #[case] expected_zero: bool,
) {
    let mut mmu = MMU::builder()
        .rom(0, PREFIX_INSTRUCTION_BYTE)
        .rom(1, opcode)
        .write(0xCCCC, value)
        .build();
    let mut cpu = CPU::builder()
        .hl(0xCCCC)
        .f_carry(carry)
        .f_subtract(true)
        .f_half_carry(true)
        .r8(register, value, &mut mmu)
        .build();
    let m = cpu.step(&mut mmu);

    assert_eq!(m, expected_m);
    assert_eq!(cpu.get_pc(), 2);
    assert_eq!(cpu.get_r8(register, &mmu), expected_value);
    assert_eq!(cpu.get_f_carry(), expected_carry);
    assert_eq!(cpu.get_f_zero(), expected_zero);
    assert!(!cpu.get_f_subtract());
    assert!(!cpu.get_f_half_carry());
}

/// RLC r8 & RRC r8
#[rstest]
#[case::left_b_nc(0x00, R8::B, 0b0000_1000, 2, 0b0001_0000, false, false)]
#[case::left_b_c(0x00, R8::B, 0b1000_1000, 2, 0b0001_0001, true, false)]
#[case::left_b_zero(0x00, R8::B, 0, 2, 0, false, true)]
#[case::left_c_nc(0x01, R8::C, 0b0000_1000, 2, 0b0001_0000, false, false)]
#[case::left_c_c(0x01, R8::C, 0b1000_1000, 2, 0b0001_0001, true, false)]
#[case::left_c_zero(0x01, R8::C, 0, 2, 0, false, true)]
#[case::left_d_nc(0x02, R8::D, 0b0000_1000, 2, 0b0001_0000, false, false)]
#[case::left_d_c(0x02, R8::D, 0b1000_1000, 2, 0b0001_0001, true, false)]
#[case::left_d_zero(0x02, R8::D, 0, 2, 0, false, true)]
#[case::left_e_nc(0x03, R8::E, 0b0000_1000, 2, 0b0001_0000, false, false)]
#[case::left_e_c(0x03, R8::E, 0b1000_1000, 2, 0b0001_0001, true, false)]
#[case::left_e_zero(0x03, R8::E, 0, 2, 0, false, true)]
#[case::left_h_nc(0x04, R8::H, 0b0000_1000, 2, 0b0001_0000, false, false)]
#[case::left_h_c(0x04, R8::H, 0b1000_1000, 2, 0b0001_0001, true, false)]
#[case::left_h_zero(0x04, R8::H, 0, 2, 0, false, true)]
#[case::left_l_nc(0x05, R8::L, 0b0000_1000, 2, 0b0001_0000, false, false)]
#[case::left_l_c(0x05, R8::L, 0b1000_1000, 2, 0b0001_0001, true, false)]
#[case::left_l_zero(0x05, R8::L, 0, 2, 0, false, true)]
#[case::left_hl_nc(0x06, R8::HL, 0b0000_1000, 4, 0b0001_0000, false, false)]
#[case::left_hl_c(0x06, R8::HL, 0b1000_1000, 4, 0b0001_0001, true, false)]
#[case::left_hl_zero(0x06, R8::HL, 0, 4, 0, false, true)]
#[case::left_a_nc(0x07, R8::A, 0b0000_1000, 2, 0b0001_0000, false, false)]
#[case::left_a_c(0x07, R8::A, 0b1000_1000, 2, 0b0001_0001, true, false)]
#[case::left_a_zero(0x07, R8::A, 0, 2, 0, false, true)]
#[case::right_b_nc(0x08, R8::B, 0b0010_0000, 2, 0b0001_0000, false, false)]
#[case::right_b_c(0x08, R8::B, 0b0010_0001, 2, 0b1001_0000, true, false)]
#[case::right_b_zero(0x08, R8::B, 0, 2, 0, false, true)]
#[case::right_c_nc(0x09, R8::C, 0b0010_0000, 2, 0b0001_0000, false, false)]
#[case::right_c_c(0x09, R8::C, 0b0010_0001, 2, 0b1001_0000, true, false)]
#[case::right_c_zero(0x09, R8::C, 0, 2, 0, false, true)]
#[case::right_d_nc(0x0A, R8::D, 0b0010_0000, 2, 0b0001_0000, false, false)]
#[case::right_d_c(0x0A, R8::D, 0b0010_0001, 2, 0b1001_0000, true, false)]
#[case::right_d_zero(0x0A, R8::D, 0, 2, 0, false, true)]
#[case::right_e_nc(0x0B, R8::E, 0b0010_0000, 2, 0b0001_0000, false, false)]
#[case::right_e_c(0x0B, R8::E, 0b0010_0001, 2, 0b1001_0000, true, false)]
#[case::right_e_zero(0x0B, R8::E, 0, 2, 0, false, true)]
#[case::right_h_nc(0x0C, R8::H, 0b0010_0000, 2, 0b0001_0000, false, false)]
#[case::right_h_c(0x0C, R8::H, 0b0010_0001, 2, 0b1001_0000, true, false)]
#[case::right_h_zero(0x0C, R8::H, 0, 2, 0, false, true)]
#[case::right_l_nc(0x0D, R8::L, 0b0010_0000, 2, 0b0001_0000, false, false)]
#[case::right_l_c(0x0D, R8::L, 0b0010_0001, 2, 0b1001_0000, true, false)]
#[case::right_l_zero(0x0D, R8::L, 0, 2, 0, false, true)]
#[case::right_hl_nc(0x0E, R8::HL, 0b0010_0000, 4, 0b0001_0000, false, false)]
#[case::right_hl_c(0x0E, R8::HL, 0b0010_0001, 4, 0b1001_0000, true, false)]
#[case::right_hl_zero(0x0E, R8::HL, 0, 4, 0, false, true)]
#[case::right_a_nc(0x0F, R8::A, 0b0010_0000, 2, 0b0001_0000, false, false)]
#[case::right_a_c(0x0F, R8::A, 0b0010_0001, 2, 0b1001_0000, true, false)]
#[case::right_a_zero(0x0F, R8::A, 0, 2, 0, false, true)]
fn test_rlc_rrc_r8(
    #[case] opcode: u8,
    #[case] register: R8,
    #[case] value: u8,
    #[case] expected_m: u8,
    #[case] expected_value: u8,
    #[case] expected_carry: bool,
    #[case] expected_zero: bool,
) {
    let mut mmu = MMU::builder()
        .rom(0, PREFIX_INSTRUCTION_BYTE)
        .rom(1, opcode)
        .write(0xCCCC, value)
        .build();
    let mut cpu = CPU::builder()
        .hl(0xCCCC)
        .f_subtract(true)
        .f_half_carry(true)
        .r8(register, value, &mut mmu)
        .build();
    let m = cpu.step(&mut mmu);

    assert_eq!(m, expected_m);
    assert_eq!(cpu.get_pc(), 2);
    assert_eq!(cpu.get_r8(register, &mmu), expected_value);
    assert_eq!(cpu.get_f_carry(), expected_carry);
    assert_eq!(cpu.get_f_zero(), expected_zero);
    assert!(!cpu.get_f_subtract());
    assert!(!cpu.get_f_half_carry());
}

/// SLA r8, SRA r8, SRL r8
#[rstest]
#[case::left_b_nc(0x20, R8::B, 0b0000_1000, 2, 0b0001_0000, false, false)]
#[case::left_b_c(0x20, R8::B, 0b1000_1000, 2, 0b0001_0000, true, false)]
#[case::left_b_zero(0x20, R8::B, 0, 2, 0, false, true)]
#[case::left_c_nc(0x21, R8::C, 0b0000_1000, 2, 0b0001_0000, false, false)]
#[case::left_c_c(0x21, R8::C, 0b1000_1000, 2, 0b0001_0000, true, false)]
#[case::left_c_zero(0x21, R8::C, 0, 2, 0, false, true)]
#[case::left_d_nc(0x22, R8::D, 0b0000_1000, 2, 0b0001_0000, false, false)]
#[case::left_d_c(0x22, R8::D, 0b1000_1000, 2, 0b0001_0000, true, false)]
#[case::left_d_zero(0x22, R8::D, 0, 2, 0, false, true)]
#[case::left_e_nc(0x23, R8::E, 0b0000_1000, 2, 0b0001_0000, false, false)]
#[case::left_e_c(0x23, R8::E, 0b1000_1000, 2, 0b0001_0000, true, false)]
#[case::left_e_zero(0x23, R8::E, 0, 2, 0, false, true)]
#[case::left_h_nc(0x24, R8::H, 0b0000_1000, 2, 0b0001_0000, false, false)]
#[case::left_h_c(0x24, R8::H, 0b1000_1000, 2, 0b0001_0000, true, false)]
#[case::left_h_zero(0x24, R8::H, 0, 2, 0, false, true)]
#[case::left_l_nc(0x25, R8::L, 0b0000_1000, 2, 0b0001_0000, false, false)]
#[case::left_l_c(0x25, R8::L, 0b1000_1000, 2, 0b0001_0000, true, false)]
#[case::left_l_zero(0x25, R8::L, 0, 2, 0, false, true)]
#[case::left_hl_nc(0x26, R8::HL, 0b0000_1000, 4, 0b0001_0000, false, false)]
#[case::left_hl_c(0x26, R8::HL, 0b1000_1000, 4, 0b0001_0000, true, false)]
#[case::left_hl_zero(0x26, R8::HL, 0, 4, 0, false, true)]
#[case::left_a_nc(0x27, R8::A, 0b0000_1000, 2, 0b0001_0000, false, false)]
#[case::left_a_c(0x27, R8::A, 0b1000_1000, 2, 0b0001_0000, true, false)]
#[case::left_a_zero(0x27, R8::A, 0, 2, 0, false, true)]
#[case::right_b_nc(0x28, R8::B, 0b0010_0000, 2, 0b0001_0000, false, false)]
#[case::right_b_c(0x28, R8::B, 0b1010_0001, 2, 0b1101_0000, true, false)]
#[case::right_b_zero(0x28, R8::B, 0, 2, 0, false, true)]
#[case::right_c_nc(0x29, R8::C, 0b0010_0000, 2, 0b0001_0000, false, false)]
#[case::right_c_c(0x29, R8::C, 0b1010_0001, 2, 0b1101_0000, true, false)]
#[case::right_c_zero(0x29, R8::C, 0, 2, 0, false, true)]
#[case::right_d_nc(0x2A, R8::D, 0b0010_0000, 2, 0b0001_0000, false, false)]
#[case::right_d_c(0x2A, R8::D, 0b1010_0001, 2, 0b1101_0000, true, false)]
#[case::right_d_zero(0x2A, R8::D, 0, 2, 0, false, true)]
#[case::right_e_nc(0x2B, R8::E, 0b0010_0000, 2, 0b0001_0000, false, false)]
#[case::right_e_c(0x2B, R8::E, 0b1010_0001, 2, 0b1101_0000, true, false)]
#[case::right_e_zero(0x2B, R8::E, 0, 2, 0, false, true)]
#[case::right_h_nc(0x2C, R8::H, 0b0010_0000, 2, 0b0001_0000, false, false)]
#[case::right_h_c(0x2C, R8::H, 0b1010_0001, 2, 0b1101_0000, true, false)]
#[case::right_h_zero(0x2C, R8::H, 0, 2, 0, false, true)]
#[case::right_l_nc(0x2D, R8::L, 0b0010_0000, 2, 0b0001_0000, false, false)]
#[case::right_l_c(0x2D, R8::L, 0b1010_0001, 2, 0b1101_0000, true, false)]
#[case::right_l_zero(0x2D, R8::L, 0, 2, 0, false, true)]
#[case::right_hl_nc(0x2E, R8::HL, 0b0010_0000, 4, 0b0001_0000, false, false)]
#[case::right_hl_c(0x2E, R8::HL, 0b1010_0001, 4, 0b1101_0000, true, false)]
#[case::right_hl_zero(0x2E, R8::HL, 0, 4, 0, false, true)]
#[case::right_a_nc(0x2F, R8::A, 0b0010_0000, 2, 0b0001_0000, false, false)]
#[case::right_a_c(0x2F, R8::A, 0b1010_0001, 2, 0b1101_0000, true, false)]
#[case::right_a_zero(0x2F, R8::A, 0, 2, 0, false, true)]
#[case::right_logical_b_nc(0x38, R8::B, 0b0010_0000, 2, 0b0001_0000, false, false)]
#[case::right_logical_b_c(0x38, R8::B, 0b1010_0001, 2, 0b0101_0000, true, false)]
#[case::right_logical_b_zero(0x38, R8::B, 0, 2, 0, false, true)]
#[case::right_logical_c_nc(0x39, R8::C, 0b0010_0000, 2, 0b0001_0000, false, false)]
#[case::right_logical_c_c(0x39, R8::C, 0b1010_0001, 2, 0b0101_0000, true, false)]
#[case::right_logical_c_zero(0x39, R8::C, 0, 2, 0, false, true)]
#[case::right_logical_d_nc(0x3A, R8::D, 0b0010_0000, 2, 0b0001_0000, false, false)]
#[case::right_logical_d_c(0x3A, R8::D, 0b1010_0001, 2, 0b0101_0000, true, false)]
#[case::right_logical_d_zero(0x3A, R8::D, 0, 2, 0, false, true)]
#[case::right_logical_e_nc(0x3B, R8::E, 0b0010_0000, 2, 0b0001_0000, false, false)]
#[case::right_logical_e_c(0x3B, R8::E, 0b1010_0001, 2, 0b0101_0000, true, false)]
#[case::right_logical_e_zero(0x3B, R8::E, 0, 2, 0, false, true)]
#[case::right_logical_h_nc(0x3C, R8::H, 0b0010_0000, 2, 0b0001_0000, false, false)]
#[case::right_logical_h_c(0x3C, R8::H, 0b1010_0001, 2, 0b0101_0000, true, false)]
#[case::right_logical_h_zero(0x3C, R8::H, 0, 2, 0, false, true)]
#[case::right_logical_l_nc(0x3D, R8::L, 0b0010_0000, 2, 0b0001_0000, false, false)]
#[case::right_logical_l_c(0x3D, R8::L, 0b1010_0001, 2, 0b0101_0000, true, false)]
#[case::right_logical_l_zero(0x3D, R8::L, 0, 2, 0, false, true)]
#[case::right_logical_hl_nc(0x3E, R8::HL, 0b0010_0000, 4, 0b0001_0000, false, false)]
#[case::right_logical_hl_c(0x3E, R8::HL, 0b1010_0001, 4, 0b0101_0000, true, false)]
#[case::right_logical_hl_zero(0x3E, R8::HL, 0, 4, 0, false, true)]
#[case::right_logical_a_nc(0x3F, R8::A, 0b0010_0000, 2, 0b0001_0000, false, false)]
#[case::right_logical_a_c(0x3F, R8::A, 0b1010_0001, 2, 0b0101_0000, true, false)]
#[case::right_logical_a_zero(0x3F, R8::A, 0, 2, 0, false, true)]
fn test_sla_sra_srl_r8(
    #[case] opcode: u8,
    #[case] register: R8,
    #[case] value: u8,
    #[case] expected_m: u8,
    #[case] expected_value: u8,
    #[case] expected_carry: bool,
    #[case] expected_zero: bool,
) {
    let mut mmu = MMU::builder()
        .rom(0, PREFIX_INSTRUCTION_BYTE)
        .rom(1, opcode)
        .write(0xCCCC, value)
        .build();
    let mut cpu = CPU::builder()
        .hl(0xCCCC)
        .f_subtract(true)
        .f_half_carry(true)
        .r8(register, value, &mut mmu)
        .build();
    let m = cpu.step(&mut mmu);

    assert_eq!(m, expected_m);
    assert_eq!(cpu.get_pc(), 2);
    assert_eq!(cpu.get_r8(register, &mmu), expected_value);
    assert_eq!(cpu.get_f_carry(), expected_carry);
    assert_eq!(cpu.get_f_zero(), expected_zero);
    assert!(!cpu.get_f_subtract());
    assert!(!cpu.get_f_half_carry());
}

/// SWAP r8
#[rstest]
#[case::b_nz(0x30, R8::B, 0b1011_0100, 2, 0b0100_1011, false)]
#[case::b_z(0x30, R8::B, 0, 2, 0, true)]
#[case::c_nz(0x31, R8::C, 0b1011_0100, 2, 0b0100_1011, false)]
#[case::c_z(0x31, R8::C, 0, 2, 0, true)]
#[case::d_nz(0x32, R8::D, 0b1011_0100, 2, 0b0100_1011, false)]
#[case::d_z(0x32, R8::D, 0, 2, 0, true)]
#[case::e_nz(0x33, R8::E, 0b1011_0100, 2, 0b0100_1011, false)]
#[case::e_z(0x33, R8::E, 0, 2, 0, true)]
#[case::h_nz(0x34, R8::H, 0b1011_0100, 2, 0b0100_1011, false)]
#[case::h_z(0x34, R8::H, 0, 2, 0, true)]
#[case::l_nz(0x35, R8::L, 0b1011_0100, 2, 0b0100_1011, false)]
#[case::l_z(0x35, R8::L, 0, 2, 0, true)]
#[case::hl_nz(0x36, R8::HL, 0b1011_0100, 4, 0b0100_1011, false)]
#[case::hl_z(0x36, R8::HL, 0, 4, 0, true)]
#[case::a_nz(0x37, R8::A, 0b1011_0100, 2, 0b0100_1011, false)]
#[case::a_z(0x37, R8::A, 0, 2, 0, true)]
fn test_swap_r8(
    #[case] opcode: u8,
    #[case] register: R8,
    #[case] value: u8,
    #[case] expected_m: u8,
    #[case] expected_value: u8,
    #[case] expected_zero: bool,
) {
    let mut mmu = MMU::builder()
        .rom(0, PREFIX_INSTRUCTION_BYTE)
        .rom(1, opcode)
        .write(0xCCCC, value)
        .build();
    let mut cpu = CPU::builder()
        .hl(0xCCCC)
        .f_carry(true)
        .f_subtract(true)
        .f_half_carry(true)
        .r8(register, value, &mut mmu)
        .build();
    let m = cpu.step(&mut mmu);

    assert_eq!(m, expected_m);
    assert_eq!(cpu.get_pc(), 2);
    assert_eq!(cpu.get_r8(register, &mmu), expected_value);
    assert_eq!(cpu.get_f_zero(), expected_zero);
    assert!(!cpu.get_f_subtract());
    assert!(!cpu.get_f_half_carry());
    assert!(!cpu.get_f_carry());
}
