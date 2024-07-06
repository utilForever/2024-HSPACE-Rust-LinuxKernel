fn f(mut s: String) {
    s.push_str(", world!");
}

fn main() {
    let mut s = String::from("Hello");

    // s is moved to f
    f(s);

    // Error: value used here after move
    // println!("{s}");
}
