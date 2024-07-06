fn main() {
    // The sum of 10 and 20 can be represented as a u8.
    assert_eq!(10_u8.checked_add(20), Some(30));

    // Unfortunately, the sum of 100 and 200 cannot.
    assert_eq!(100_u8.checked_add(200), None);

    // Do the addition; panic if it overflows.
    let x = i8::MAX;
    let y = 1;
    let _ = x.checked_add(y).unwrap();

    // Oddly, signed division can overflow too, in one particular case.
    // A signed n-bit type can represent -2ⁿ⁻¹, but not 2ⁿ⁻¹.
    assert_eq!((-128_i8).checked_div(-1), None);
}
