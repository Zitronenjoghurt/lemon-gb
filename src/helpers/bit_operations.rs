pub fn unsigned_16(lsb: u8, msb: u8) -> u16 {
    lsb as u16 | ((msb as u16) << 8)
}
