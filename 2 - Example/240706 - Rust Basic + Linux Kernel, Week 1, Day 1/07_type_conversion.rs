fn main() {
    let a = 10;
    let b = 30.4;

    // Error: mismatched types
    // let c = a + b;

    // Use exlicit type conversion
    let c = a as f64 + b;

    println!("{c}");
}
