fn main() {
    // This does not compile.
    // struct S {
    //     r: &i32,            // Error: Missing lifetime specifier
    // }

    struct S<'a> {
        r: &'a i32,
    }

    let s;

    {
        let x = 10;
        s = S { r: &x };
    }

    assert_eq!(*s.r, 10);   // Bad: Reads from dropped `x`
}
