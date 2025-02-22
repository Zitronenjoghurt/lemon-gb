use crate::game_boy::components::mmu::MMU;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct MMUBuilder {
    mmu: MMU,
}

impl MMUBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> MMU {
        self.mmu
    }

    pub fn write(mut self, address: u16, value: u8) -> Self {
        self.mmu.write(address, value);
        self
    }

    pub fn rom(mut self, address: u16, value: u8) -> Self {
        self.mmu.force_write_rom(address, value);
        self
    }
}
