use crate::game_boy::components::cpu::registers::builder::CPURegistersBuilderTrait;
use crate::game_boy::components::cpu::registers::CpuRegistersAccessTrait;
use crate::game_boy::components::cpu::CPU;
use crate::game_boy::components::mmu::MMU;
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
    let mut mmu = MMU::builder().set(0, opcode).build();
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

    let mut mmu = MMU::builder().set(0, 0x86).set(ADDRESS, value).build();
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
    let mut mmu = MMU::builder().set(0, 0x87).build();
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

/// JUMP IMM (0xC3)
#[rstest]
#[case::basic_jump(0x11, 0x22, 0x2211)]
#[case::jump_to_start(0, 0, 0)]
#[case::jump_to_end(0xFF, 0xFF, 0xFFFF)]
fn test_jump_imm(#[case] imm1: u8, #[case] imm2: u8, #[case] expected_pc: u16) {
    let mut mmu = MMU::builder()
        .set(0, 0xC3)
        .set(1, imm1)
        .set(2, imm2)
        .build();
    let mut cpu = CPU::default();

    let m = cpu.step(&mut mmu);
    assert_eq!(cpu.get_pc(), expected_pc);
    assert_eq!(m, 4);
}

/// JUMP COND IMM
#[rstest]
#[case::nz_jump(0xC2, 0x34, 0x12, false, false, 0x1234, 4)]
#[case::nz_no_jump(0xC2, 0x34, 0x12, true, false, 3, 3)]
#[case::z_jump(0xCA, 0x34, 0x12, true, false, 0x1234, 4)]
#[case::z_no_jump(0xCA, 0x34, 0x12, false, false, 3, 3)]
#[case::nc_jump(0xD2, 0x34, 0x12, false, false, 0x1234, 4)]
#[case::nc_no_jump(0xD2, 0x34, 0x12, false, true, 3, 3)]
#[case::c_jump(0xDA, 0x34, 0x12, false, true, 0x1234, 4)]
#[case::c_no_jump(0xDA, 0x34, 0x12, false, false, 3, 3)]
fn test_jump_cond_imm(
    #[case] opcode: u8,
    #[case] imm1: u8,
    #[case] imm2: u8,
    #[case] f_zero: bool,
    #[case] f_carry: bool,
    #[case] expected_pc: u16,
    #[case] expected_m: u8,
) {
    let mut mmu = MMU::builder()
        .set(0, opcode)
        .set(1, imm1)
        .set(2, imm2)
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
    let mut mmu = MMU::builder().set(0, 0xE9).build();
    let mut cpu = CPU::builder().hl(target_address).build();
    let m = cpu.step(&mut mmu);

    assert_eq!(cpu.get_pc(), target_address);
    assert_eq!(m, 1);
}
