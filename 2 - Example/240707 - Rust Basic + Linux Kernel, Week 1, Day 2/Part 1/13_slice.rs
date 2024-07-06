fn main() {
    let v: Vec<f64> = vec![0.0, 0.707, 1.0, 0.707];
    let a: [f64; 4] = [0.0, -0.707, -1.0, -0.707];

    let sv: &[f64] = &v;
    let sa: &[f64] = &a;

    fn print(n: &[f64]) {
        for elt in n {
            println!("{}", elt);
        }
    }

    print(&a);          // Works on arrays
    print(&v);          // Works on vectors

    print(&v[0..2]);    // Print the first two elements of v
    print(&a[2..]);     // Print elements of a starting with a[2]
    print(&sv[1..3]);   // Print v[1] and v[2]
    print(&sa[1..]);    // Print a[1], a[2], and a[3]
}
