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

/// Bits are indexed right to left starting from 0
pub fn get_bit_u8(value: u8, bit_index: usize) -> bool {
    (value >> bit_index) & 1 == 1
}

/// Bits are indexed right to left starting from 0
pub fn get_bit_u16(value: u16, bit_index: usize) -> bool {
    (value >> bit_index) & 1 == 1
}

/// Adds a and b and returns (result, half_carry, carry)
pub fn add_carry_u8(a: u8, b: u8) -> (u8, bool, bool) {
    let (result, carry) = a.overflowing_add(b);

    // Check half carry (bit 3)
    let h_carry = ((a & 0x0F) + (b & 0x0F)) > 0x0F;

    (result, h_carry, carry)
}

/// Adds a and b and returns (result, half_carry, carry)
pub fn add_carry_u16(a: u16, b: u16) -> (u16, bool, bool) {
    let (result, carry) = a.overflowing_add(b);

    // Check half carry (bit 11)
    let h_carry = ((a & 0x0FFF) + (b & 0x0FFF)) > 0x0FFF;

    (result, h_carry, carry)
}
