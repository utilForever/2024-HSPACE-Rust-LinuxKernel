fn main() {
    // struct S<'a> {
    //     x: &'a i32,
    //     y: &'a i32,
    // }

    struct S<'a, 'b> {
        x: &'a i32,
        y: &'b i32,
    }

    let x = 10;
    let r;

    {
        let y = 20;
        {
            // Note that x and y have different lifetimes.
            let s = S { x: &x, y: &y };
            r = s.x;
        }
    }

    println!("{r}");
}
