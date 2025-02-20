use crate::game_boy::components::cpu::registers::flags_register::CPUFlagsRegister;
use crate::game_boy::components::cpu::registers::{CPURegisters, CpuRegistersAccessTrait};

#[test]
fn test_flag_registers() {
    let mut test1 = CPUFlagsRegister::default();
    assert!(!test1.get_zero());
    assert!(!test1.get_subtract());
    assert!(!test1.get_half_carry());
    assert!(!test1.get_carry());

    test1.set_zero(true);
    assert!(test1.get_zero());
    test1.set_subtract(true);
    assert!(test1.get_subtract());
    test1.set_half_carry(true);
    assert!(test1.get_half_carry());
    test1.set_carry(true);
    assert!(test1.get_carry());
    assert_eq!(u8::from(test1), 0b1111_0000);

    let test2 = CPUFlagsRegister::from(0b1010_0000);
    assert!(test2.get_zero());
    assert!(!test2.get_subtract());
    assert!(test2.get_half_carry());
    assert!(!test2.get_carry());
    assert_eq!(u8::from(test2), 0b1010_0000);

    let mut test3 = CPUFlagsRegister::from(0b0101_1101);
    assert!(!test3.get_zero());
    assert!(test3.get_subtract());
    assert!(!test3.get_half_carry());
    assert!(test3.get_carry());
    assert_eq!(u8::from(test3), 0b0101_0000);

    test3.set_subtract(false);
    test3.set_carry(false);
    assert_eq!(u8::from(test3), 0b0000_0000);
}

#[test]
fn test_bc() {
    let mut registers = CPURegisters::default();
    assert_eq!(registers.get_b(), 0);
    assert_eq!(registers.get_c(), 0);

    registers.set_bc(0x4FD2);
    assert_eq!(registers.get_bc(), 0x4FD2);
    assert_eq!(registers.get_b(), 0x4F);
    assert_eq!(registers.get_c(), 0xD2);
}
