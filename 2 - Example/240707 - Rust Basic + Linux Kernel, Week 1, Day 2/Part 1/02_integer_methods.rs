fn main() {
    assert_eq!(2_u16.pow(4), 16);               // Exponentiation
    assert_eq!((-4_i32).abs(), 4);              // Absolute value
    assert_eq!(0b101101_u8.count_ones(), 4);    // Population count

    // println!("{}", (-4).abs());              // Compile error: Ambiguous numeric type
    println!("{}", (-4_i32).abs());
    println!("{}", i32::abs(-4));
}
