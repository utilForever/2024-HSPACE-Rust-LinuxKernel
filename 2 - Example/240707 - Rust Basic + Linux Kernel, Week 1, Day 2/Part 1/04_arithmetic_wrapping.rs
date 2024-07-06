fn main() {
    // The first product can be represented as a u16;
    // the second cannot, so we get 250000 modulo 2¹⁶.
    assert_eq!(100_u16.wrapping_mul(200), 20000);
    assert_eq!(500_u16.wrapping_mul(500), 53392);
    
    // Operations on signed types may wrap to negative values.
    assert_eq!(500_i16.wrapping_mul(500), -12144);

    // In bitwise shift operations, the shift distance is wrapped to fall within the size of the value.
    // So a shift of 17 bits in a 16-bit type is a shift of 1.
    assert_eq!(5_i16.wrapping_shl(17), 10);
}
