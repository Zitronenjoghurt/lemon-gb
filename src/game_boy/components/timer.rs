//! https://hacktix.github.io/GBEDG/timers/

use crate::game_boy::components::mmu::{
    DIV_ADDRESS, INITIAL_DIV, MMU, TAC_ADDRESS, TIMA_ADDRESS, TMA_ADDRESS,
};
use crate::helpers::bit_operations::{get_bit_u16, get_bit_u8};
use serde::{Deserialize, Serialize};

// ToDo: Maybe add more accurate TIMA overflow timing, its 0 for 1 M-Cycle before getting reset to TMA and triggering the interrupt
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Timer {
    pub counter: u16,
    last_and_result: bool,
}

impl Timer {
    pub fn initialize() -> Self {
        Self {
            counter: (INITIAL_DIV as u16) << 8,
            last_and_result: false,
        }
    }

    /// Returns true if a Timer Interrupt was triggered
    pub fn step(&mut self, cycles: u8, mmu: &mut MMU) -> bool {
        let mut interrupt_triggered = false;

        for _ in 0..cycles {
            self.update_counter(1, mmu);
            self.update_div(mmu);
            if self.update_tima(mmu) {
                interrupt_triggered = true;
            }
        }

        interrupt_triggered
    }

    fn update_counter(&mut self, cycles: u8, mmu: &MMU) {
        let div = mmu.read(DIV_ADDRESS);
        // If DIV is 0 but our counter's high byte isn't, DIV must have been reset
        if div == 0 && (self.counter >> 8) != 0 {
            self.counter = 0;
        }
        self.counter += cycles as u16 * 4;
    }

    fn update_div(&self, mmu: &mut MMU) {
        mmu.timer_update_div((self.counter >> 8) as u8);
    }

    /// Returns true if a Timer Interrupt should be requested
    fn update_tima(&mut self, mmu: &mut MMU) -> bool {
        let tac = mmu.read(TAC_ADDRESS);
        let timer_enabled = get_bit_u8(tac, 2);
        let and_value = match tac & 0b0000_0011 {
            0b00 => get_bit_u16(self.counter, 9),
            0b01 => get_bit_u16(self.counter, 3),
            0b10 => get_bit_u16(self.counter, 5),
            0b11 => get_bit_u16(self.counter, 7),
            _ => unreachable!(),
        };

        let and_result = and_value && timer_enabled;

        // No falling edge detected
        if !self.last_and_result || and_result {
            self.last_and_result = and_result;
            return false;
        } else {
            self.last_and_result = false;
        }

        let last_tima = mmu.read(TIMA_ADDRESS);
        if last_tima != 0xFF {
            mmu.write(TIMA_ADDRESS, last_tima + 1);
            false
        } else {
            let tma = mmu.read(TMA_ADDRESS);
            mmu.write(TIMA_ADDRESS, tma);
            true
        }
    }
}
