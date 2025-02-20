use crate::game_boy::components::mmu::builder::MMUBuilder;
use crate::helpers::bit_operations::construct_u16;

mod builder;

#[derive(Debug, Clone, PartialEq)]
pub struct MMU {
    memory: [u8; 0xFFFF],
}

impl MMU {
    pub fn builder() -> MMUBuilder {
        MMUBuilder::new()
    }

    pub fn read(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn read_16(&self, address: u16) -> u16 {
        let lsb = self.read(address);
        let msb = self.read(address.wrapping_add(1));
        construct_u16(lsb, msb)
    }

    pub fn write(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }
}

impl Default for MMU {
    fn default() -> Self {
        Self {
            memory: [0; 0xFFFF],
        }
    }
}
