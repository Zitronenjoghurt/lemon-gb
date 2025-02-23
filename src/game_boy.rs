use crate::enums::interrupts::Interrupt;
use crate::game_boy::components::cartridge::Cartridge;
use crate::game_boy::components::cpu::CPU;
use crate::game_boy::components::mmu::{IF_ADDRESS, MMU};
use crate::game_boy::components::timer::Timer;
use crate::helpers::bit_operations::set_bit_u8;

pub mod components;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct GameBoy {
    /// Central Processing Unit
    cpu: CPU,
    /// Memory Management Unit
    /// Handles all memory storage and access
    mmu: MMU,
    timer: Timer,
}

impl GameBoy {
    pub fn initialize(cartridge: &Cartridge) -> Self {
        Self {
            cpu: CPU::initialize(),
            mmu: MMU::initialize(cartridge),
            timer: Timer::initialize(),
        }
    }

    pub fn step(&mut self) -> u8 {
        let m = self.cpu.step(&mut self.mmu);
        let timer_interrupt = self.timer.step(m, &mut self.mmu);

        self.write_interrupts(timer_interrupt);
        m
    }

    fn write_interrupts(&mut self, timer: bool) {
        let i_flag = self.mmu.read(IF_ADDRESS);
        if timer {
            set_bit_u8(i_flag, Interrupt::Timer.get_if_index(), true);
        }
        self.mmu.write(IF_ADDRESS, i_flag);
    }
}
