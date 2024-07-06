fn main() {
    assert_eq!(5f32.sqrt() * 5f32.sqrt(), 5.); // Exactly 5.0, per IEEE
    assert_eq!((-1.01f64).floor(), -2.0);
    assert_eq!((-1.01f64).ceil(), -1.0);
}
