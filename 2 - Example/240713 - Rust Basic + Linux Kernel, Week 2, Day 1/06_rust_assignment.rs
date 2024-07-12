fn main() {
    let x = 10;
    let y = 20;
    let b = true;
    let mut r = &x;

    if b {
        r = &y;     // Now, r refers to y instead of x
    }

    assert!(*r == 10 || *r == 20);
}
