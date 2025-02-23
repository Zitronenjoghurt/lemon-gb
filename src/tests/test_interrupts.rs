use crate::enums::interrupts::Interrupt;
use crate::game_boy::components::cpu::registers::CpuRegistersAccessTrait;
use crate::game_boy::components::cpu::CPU;
use crate::game_boy::components::mmu::{IE_ADDRESS, IF_ADDRESS, MMU};

#[test]
fn test_vblank_interrupt() {
    let mut mmu = MMU::builder()
        .write(IF_ADDRESS, 0b0000_0001)
        .write(IE_ADDRESS, 0b0000_0001)
        .build();
    let mut cpu = CPU::builder().ime(true).build();

    let m = cpu.step(&mut mmu);
    assert_eq!(m, 5);
    assert_eq!(cpu.get_pc(), Interrupt::Vblank.get_target_address());
    assert!(!cpu.get_ime());
}
