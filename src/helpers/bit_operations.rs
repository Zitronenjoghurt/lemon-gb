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
pub fn set_bit_u8(value: u8, bit_index: usize, bit: bool) -> u8 {
    if bit {
        value | (1 << bit_index)
    } else {
        value & !(1 << bit_index)
    }
}

/// Bits are indexed right to left starting from 0
pub fn get_bit_u16(value: u16, bit_index: usize) -> bool {
    (value >> bit_index) & 1 == 1
}

/// Adds a and b and returns (result, half_carry, carry)
pub fn add_u8(a: u8, b: u8) -> (u8, bool, bool) {
    let (result, carry) = a.overflowing_add(b);

    // Check half carry (bit 3)
    let h_carry = ((a & 0x0F) + (b & 0x0F)) > 0x0F;

    (result, h_carry, carry)
}

/// Adds a, b and the given carry; returns (result, half_carry, carry)
pub fn add_carry_u8(a: u8, b: u8, carry: bool) -> (u8, bool, bool) {
    let (result, carry1) = a.overflowing_add(b);
    let (result, carry2) = result.overflowing_add(carry as u8);

    // Check half carry (bit 3)
    let h_carry = ((a & 0x0F) + (b & 0x0F) + (carry as u8)) > 0x0F;

    (result, h_carry, carry1 || carry2)
}

/// Adds a and b and returns (result, half_carry, carry)
pub fn add_u16(a: u16, b: u16) -> (u16, bool, bool) {
    let (result, carry) = a.overflowing_add(b);

    // Check half carry (bit 11)
    let h_carry = ((a & 0x0FFF) + (b & 0x0FFF)) > 0x0FFF;

    (result, h_carry, carry)
}

/// Subtracts b from a and returns (result, half_carry, carry)
pub fn sub_u8(a: u8, b: u8) -> (u8, bool, bool) {
    let (result, carry) = a.overflowing_sub(b);

    // Check half carry (bit 3)
    let h_carry = (a & 0x0F) < (b & 0x0F);

    (result, h_carry, carry)
}

/// Subtracts b and the given carry from a; returns (result, half_carry, carry)
pub fn sub_carry_u8(a: u8, b: u8, carry: bool) -> (u8, bool, bool) {
    let (result1, carry1) = a.overflowing_sub(b);
    let (result, carry2) = result1.overflowing_sub(carry as u8);

    // Check half carry (bit 3)
    let h_carry = (a & 0x0F) < ((b & 0x0F) + (carry as u8));

    (result, h_carry, carry1 || carry2)
}

/// Rotates the value left by 1, returning (result, carry)
/// ```
/// ┏━ Carry ━┓   ┏━━━━━━ u8 ━━━━━━━┓
/// ┃    C   ←╂─┬─╂─ b7 ← ... ← b0 ←╂─┐
/// ┗━━━━━━━━━┛ │ ┗━━━━━━━━━━━━━━━━━┛ │
///             └─────────────────────┘
/// ```
pub fn rotate_left_get_carry_u8(value: u8) -> (u8, bool) {
    let result = value.rotate_left(1);
    let carry = get_bit_u8(value, 7);
    (result, carry)
}

/// Rotates the value right by 1, returning (result, carry)
/// ```
///   ┏━━━━━━━ u8 ━━━━━━┓   ┏━ Carry ━┓
/// ┌─╂→ b7 → ... → b0 ─╂─┬─╂→   C    ┃
/// │ ┗━━━━━━━━━━━━━━━━━┛ │ ┗━━━━━━━━━┛
/// └─────────────────────┘
/// ```
pub fn rotate_right_get_carry_u8(value: u8) -> (u8, bool) {
    let result = value.rotate_right(1);
    let carry = get_bit_u8(value, 0);
    (result, carry)
}

/// Rotates the value right by 1 THROUGH the given carry, returning (result, new_carry)
/// ```
///   ┏━━━━━━━ u8 ━━━━━━┓ ┏━ Carry ━┓
/// ┌─╂→ b7 → ... → b0 ─╂─╂→   C   ─╂─┐
/// │ ┗━━━━━━━━━━━━━━━━━┛ ┗━━━━━━━━━┛ │
/// └─────────────────────────────────┘
/// ```
pub fn rotate_right_through_carry_u8(value: u8, carry: bool) -> (u8, bool) {
    let new_carry = get_bit_u8(value, 0);
    let result = set_bit_u8(value >> 1, 7, carry);
    (result, new_carry)
}

/// Rotates the value left by 1 THROUGH the given carry, returning (result, new_carry)
/// ```
///   ┏━ Carry ━┓ ┏━━━━━━ u8 ━━━━━━━┓
/// ┌─╂─   C   ←╂─╂─ b7 ← ... ← b0 ←╂─┐
/// │ ┗━━━━━━━━━┛ ┗━━━━━━━━━━━━━━━━━┛ │
/// └─────────────────────────────────┘
/// ```
pub fn rotate_left_through_carry_u8(value: u8, carry: bool) -> (u8, bool) {
    let new_carry = get_bit_u8(value, 7);
    let result = set_bit_u8(value << 1, 0, carry);
    (result, new_carry)
}
