fn main() {
    let mut a = 10;
    let b = &a;

    // Error: cannot borrow `a` as mutable because it is also borrowed as immutable
    {
        let c = &mut a;
        *c = 20;
    }

    println!("a : {a}");
    println!("b : {b}");
}
