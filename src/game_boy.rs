use crate::enums::interrupts::Interrupt;
use crate::game_boy::components::cartridge::Cartridge;
use crate::game_boy::components::cpu::CPU;
use crate::game_boy::components::mmu::{IF_ADDRESS, MMU};
use crate::game_boy::components::ppu::PPU;
use crate::game_boy::components::timer::Timer;
use crate::game_boy::save_state::GameBoySaveState;
use crate::helpers::bit_operations::set_bit_u8;
use std::error::Error;

pub mod components;
pub mod save_state;

#[derive(Debug, Clone, PartialEq)]
pub struct GameBoy {
    /// Central Processing Unit
    cpu: CPU,
    /// Memory Management Unit
    /// Handles all memory storage and access
    mmu: MMU,
    timer: Timer,
    ppu: PPU,
}

impl GameBoy {
    pub fn initialize(cartridge: &Cartridge) -> Self {
        Self {
            cpu: CPU::initialize(),
            mmu: MMU::initialize(cartridge),
            timer: Timer::initialize(),
            ppu: PPU::new(),
        }
    }

    pub fn step(&mut self) -> bool {
        let m = self.cpu.step(&mut self.mmu);
        let timer_interrupt = self.timer.step(m, &mut self.mmu);
        let (vblank_interrupt, stat_interrupt, frame_finished) = self.ppu.step(m, &mut self.mmu);

        self.write_interrupts(timer_interrupt, vblank_interrupt, stat_interrupt);
        frame_finished
    }

    pub fn finish_frame(&mut self) {
        while !self.step() {}
    }

    fn write_interrupts(&mut self, timer: bool, vblank: bool, stat: bool) {
        let mut i_flag = self.mmu.read(IF_ADDRESS);
        if timer {
            i_flag = set_bit_u8(i_flag, Interrupt::Timer.get_if_index(), true);
        }
        if vblank {
            i_flag = set_bit_u8(i_flag, Interrupt::Vblank.get_if_index(), true);
        }
        if stat {
            i_flag = set_bit_u8(i_flag, Interrupt::Lcd.get_if_index(), true);
        }
        self.mmu.write(IF_ADDRESS, i_flag);
    }

    pub fn save(&self) -> GameBoySaveState {
        GameBoySaveState {
            cartridge_header: self.mmu.cartridge_header.clone(),
            cpu: self.cpu.clone(),
            timer: self.timer.clone(),
            mmu_state: self.mmu.save(),
        }
    }

    pub fn load(state: GameBoySaveState, cartridge: &Cartridge) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            cpu: state.cpu,
            mmu: MMU::load(state.mmu_state, cartridge)?,
            timer: state.timer,
            ppu: PPU::new(), // ToDO: Save/Load PPU
        })
    }

    pub fn get_frame_buffer(&self) -> &[u8] {
        self.ppu.get_frame_buffer()
    }
}

impl Default for GameBoy {
    fn default() -> Self {
        Self {
            cpu: CPU::default(),
            mmu: MMU::default(),
            timer: Timer::default(),
            ppu: PPU::new(),
        }
    }
}
