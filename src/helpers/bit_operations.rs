/// Creates an u16 from 2 bytes.
/// Example: LSB 0xAA, MSB 0xBB => 0xBBAA
///
/// # Arguments
///
/// * `lsb`: The least significant byte
/// * `msb`: The most significant byte
///
/// # Returns
///
/// u16
pub fn construct_u16(lsb: u8, msb: u8) -> u16 {
    lsb as u16 | ((msb as u16) << 8)
}

/// Deconstructs a given u16 into 2 bytes.
/// Example: 0xBBAA => (0xAA, 0xBB)
///
/// # Returns
///
/// (LSB, MSB)
/// (u8, u8)
pub fn deconstruct_u16(value: u16) -> (u8, u8) {
    (value as u8, (value >> 8) as u8)
}
