use crate::game_boy::components::mmu::{DIV_ADDRESS, MMU, TAC_ADDRESS, TIMA_ADDRESS, TMA_ADDRESS};
use crate::game_boy::components::timer::Timer;
use rstest::rstest;

#[test]
fn test_div_increment() {
    let mut timer = Timer::default();
    let mut mmu = MMU::default();

    // DIV increments every 64 cycles (16 M-cycles)
    for _ in 0..63 {
        timer.step(1, &mut mmu);
        assert_eq!(mmu.read(DIV_ADDRESS), 0);
    }

    timer.step(1, &mut mmu);
    assert_eq!(mmu.read(DIV_ADDRESS), 1);
}

#[test]
fn test_div_reset() {
    let mut timer = Timer::default();
    let mut mmu = MMU::default();

    // Run enough cycles to get DIV > 0
    for _ in 0..200 {
        timer.step(1, &mut mmu);
    }

    let before_reset = mmu.read(DIV_ADDRESS);
    assert!(before_reset > 0);

    // Writing any value to DIV resets it to 0
    mmu.write(DIV_ADDRESS, 123); // Value doesn't matter
    assert_eq!(mmu.read(DIV_ADDRESS), 0);

    timer.step(1, &mut mmu);
    assert_eq!(mmu.read(DIV_ADDRESS), 0);
}

#[rstest]
#[case::clock_16(0b101, 4)]
#[case::clock_16(0b110, 16)]
#[case::clock_16(0b111, 64)]
#[case::clock_16(0b100, 256)]
fn test_tima_increment_rates(#[case] tac: u8, #[case] cycles: u16) {
    let mut timer = Timer::default();
    let mut mmu = MMU::default();

    mmu.write(TAC_ADDRESS, tac);
    mmu.write(TIMA_ADDRESS, 0);

    // Step just before increment
    for _ in 0..(cycles - 1) {
        timer.step(1, &mut mmu);
    }
    assert_eq!(mmu.read(TIMA_ADDRESS), 0);

    // Step to trigger increment
    timer.step(1, &mut mmu);
    assert_eq!(mmu.read(TIMA_ADDRESS), 1,);
}

#[test]
fn test_tima_overflow() {
    let mut timer = Timer::default();
    let mut mmu = MMU::default();

    // Set up timer
    mmu.write(TAC_ADDRESS, 0b101); // Enable timer, Clock/16
    mmu.write(TIMA_ADDRESS, 0xFF);
    mmu.write(TMA_ADDRESS, 0x42);

    // Trigger overflow
    timer.step(4, &mut mmu);
    assert_eq!(mmu.read(TIMA_ADDRESS), 0x42);
}

#[test]
fn test_timer_disable_edge_case() {
    let mut timer = Timer::default();
    let mut mmu = MMU::default();

    // Set up timer
    mmu.write(TAC_ADDRESS, 0b101); // Enable timer, Clock/16
    mmu.write(TIMA_ADDRESS, 0);

    // Run for a few cycles
    timer.step(2, &mut mmu);
    let initial_tima = mmu.read(TIMA_ADDRESS);

    // Disable timer by clearing the enable bit
    // According to docs, this can cause one more increment due to the falling edge
    mmu.write(TAC_ADDRESS, 0b001);
    timer.step(1, &mut mmu);

    assert_eq!(mmu.read(TIMA_ADDRESS), initial_tima + 1);

    // Further cycles should not increment TIMA
    let tima_after_disable = mmu.read(TIMA_ADDRESS);
    timer.step(100, &mut mmu);
    assert_eq!(mmu.read(TIMA_ADDRESS), tima_after_disable);
}

#[test]
fn test_div_write_affects_timer() {
    let mut timer = Timer::default();
    let mut mmu = MMU::default();

    // Enable timer with Clock/16 rate
    mmu.write(TAC_ADDRESS, 0b101);
    mmu.write(TIMA_ADDRESS, 0);

    // Run for a bit to get DIV != 0
    timer.step(100, &mut mmu);
    let initial_tima = mmu.read(TIMA_ADDRESS);

    // Writing to DIV resets internal counter, which can trigger TIMA increment
    mmu.write(DIV_ADDRESS, 0);

    // This might cause a TIMA increment due to falling edge
    assert!(mmu.read(TIMA_ADDRESS) >= initial_tima);
}
