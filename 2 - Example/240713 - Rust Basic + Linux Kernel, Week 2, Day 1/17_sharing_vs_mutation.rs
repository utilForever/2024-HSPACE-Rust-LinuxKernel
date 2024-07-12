fn main() {
    // Part 1
    let mut x = 10;
    let r1 = &x;
    let r2 = &x;                    // OK: Multiple shared borrows permitted
    x += 10;                        // Error: Cannot assign to `x` because it is borrowed
    let m = &mut x;                 // Error: Cannot borrow `x` as mutable because it is also borrowed as immutable
    println!("{r1}, {r2}, {m}");    // The references are used here, so their lifetimes must last at least this long

    let mut y = 20;
    let m1 = &mut y;
    let m2 = &mut y;                // Error: Cannot borrow as mutable more than once
    let z = y;                      // Error: Cannot use `y` because it was mutably borrowed
    println!("{m1}, {m2}, {z}");    // References are used here

    // Part 2
    let mut w = (107, 109);
    let r = &w;
    let r0 = &r.0;      // OK: Reborrowing shared as shared
    let m1 = &mut r.1;  // Error: Can't reborrow shared as mutable
    println!("{r0}");   // r0 gets used here

    // Part 3
    let mut v = (136, 139);
    let m = &mut v;
    let m0 = &mut m.0;  // OK: reborrowing mutable from mutable
    *m0 = 137;
    let r1 = &m.1;      // OK: Reborrowing shared from mutable, and doesn't overlap with m0
    v.1;                // Error: Access through other paths still forbidden
    println!("{r1}");   // r1 gets used here
}
