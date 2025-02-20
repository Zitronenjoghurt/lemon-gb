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

    pub fn set(mut self, address: u16, value: u8) -> Self {
        self.mmu.write(address, value);
        self
    }
}
