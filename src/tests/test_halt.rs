use crate::enums::interrupts::Interrupt;
use crate::game_boy::components::cpu::registers::builder::CPURegistersBuilderTrait;
use crate::game_boy::components::cpu::registers::CpuRegistersAccessTrait;
use crate::game_boy::components::cpu::CPU;
use crate::game_boy::components::mmu::{IE_ADDRESS, IF_ADDRESS, MMU};

#[test]
fn test_halt_no_ime() {
    let mut mmu = MMU::builder()
        .rom(0, 0x76)
        .rom(1, 0x80) // Adds the register B to register A
        .write(IE_ADDRESS, Interrupt::Vblank.get_mask()) // Enable VBlank interrupt
        .build();
    let mut cpu = CPU::builder().b(1).build();

    // CPU will be in low power mode and won't execute instructions
    for _ in 0..5 {
        let m = cpu.step(&mut mmu);
        assert_eq!(m, 1);
        assert_eq!(cpu.get_pc(), 1);
        assert_eq!(cpu.get_a(), 0);
    }

    // Trigger interrupt
    mmu.write(IF_ADDRESS, Interrupt::Vblank.get_mask());

    // Interrupt will be detected but IME is disabled => wake from sleep and continue
    let m = cpu.step(&mut mmu);
    assert_eq!(m, 1);
    assert_eq!(cpu.get_pc(), 2);
    assert_eq!(cpu.get_a(), 1);
}

#[test]
fn test_halt_ime() {
    let mut mmu = MMU::builder()
        .rom(0, 0x76)
        .rom(1, 0x80)
        .write(IE_ADDRESS, Interrupt::Vblank.get_mask())
        .build();
    let mut cpu = CPU::builder().b(1).ime(true).build();

    // CPU will be in low power mode and won't execute instructions
    for _ in 0..5 {
        let m = cpu.step(&mut mmu);
        assert_eq!(m, 1);
        assert_eq!(cpu.get_pc(), 1);
        assert_eq!(cpu.get_a(), 0);
    }

    // Trigger interrupt
    mmu.write(IF_ADDRESS, Interrupt::Vblank.get_mask());

    // Interrupt will be detected and IME is enabled => jumping to interrupt handler
    let m = cpu.step(&mut mmu);
    assert_eq!(m, 5);
    assert_eq!(cpu.get_pc(), Interrupt::Vblank.get_target_address());
    assert_eq!(cpu.get_a(), 0);
}

#[test]
fn test_halt_bug() {
    let mut mmu = MMU::builder()
        .rom(0, 0x76)
        .rom(1, 0x80)
        .write(IE_ADDRESS, Interrupt::Vblank.get_mask())
        .write(IF_ADDRESS, Interrupt::Vblank.get_mask())
        .build();
    let mut cpu = CPU::builder().b(1).build();

    // We halt while there's an interrupt scheduled but IME is disabled => HALT bug
    // The next instruction will be executed twice
    let m = cpu.step(&mut mmu);
    assert_eq!(m, 1);
    assert_eq!(cpu.get_pc(), 1);
    assert_eq!(cpu.get_a(), 1);

    // After the 0x80 we had a NOP, but because of the halting bug, 0x80 was executed twice
    let m = cpu.step(&mut mmu);
    assert_eq!(m, 1);
    assert_eq!(cpu.get_pc(), 2);
    assert_eq!(cpu.get_a(), 2);
}
