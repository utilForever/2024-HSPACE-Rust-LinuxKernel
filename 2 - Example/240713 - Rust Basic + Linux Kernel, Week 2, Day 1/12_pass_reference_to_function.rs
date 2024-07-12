fn main() {
    // This could be written more briefly: fn g(p: &i32),
    // but let's write out the lifetimes for now.
    fn g<'a>(p: &'a i32) {
        // ...
    }

    let x = 10;
    g(&x);
}
