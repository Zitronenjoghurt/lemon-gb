use crate::game_boy::components::cartridge::header::CartridgeHeader;
use crate::game_boy::components::cpu::CPU;
use crate::game_boy::components::mmu::save_state::MMUSaveState;
use crate::game_boy::components::timer::Timer;
use serde::{Deserialize, Serialize};
use std::io::{Error, ErrorKind};
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GameBoySaveState {
    pub cartridge_header: CartridgeHeader,
    pub cpu: CPU,
    pub timer: Timer,
    pub mmu_state: MMUSaveState,
}

impl GameBoySaveState {
    pub fn store_json(&self, path: &Path) -> std::io::Result<()> {
        let serialized = serde_json::to_string_pretty(&self)?;
        std::fs::write(path, serialized)?;
        Ok(())
    }

    pub fn load_json(path: &Path) -> std::io::Result<Self> {
        let serialized = std::fs::read(&path)?;
        Ok(serde_json::from_slice(&serialized)?)
    }

    pub fn store_binary(&self, path: &Path) -> std::io::Result<()> {
        let serialized = bincode::serialize(&self)
            .map_err(|e| Error::new(ErrorKind::InvalidData, e.to_string()))?;
        std::fs::write(path, serialized)?;
        Ok(())
    }

    pub fn load_binary(path: &Path) -> std::io::Result<Self> {
        let serialized = std::fs::read(&path)?;
        bincode::deserialize(&serialized)
            .map_err(|e| Error::new(ErrorKind::InvalidData, e.to_string()))
    }
}
