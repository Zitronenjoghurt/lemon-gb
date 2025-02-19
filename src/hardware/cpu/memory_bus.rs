#[derive(Debug, Copy, Clone, PartialEq)]
pub struct MemoryBus {
    memory: [u8; 0xFFFF],
}

impl MemoryBus {
    pub fn read(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn write(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }
}

impl Default for MemoryBus {
    fn default() -> Self {
        Self {
            memory: [0; 0xFFFF],
        }
    }
}
