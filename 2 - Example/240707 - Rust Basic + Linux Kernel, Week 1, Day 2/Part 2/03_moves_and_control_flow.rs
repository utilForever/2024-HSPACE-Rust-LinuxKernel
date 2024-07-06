fn main() {
    // fn f(_v: Vec<i32>) {
    //     // Do something
    // }

    // fn g(_v: Vec<i32>) {
    //     // Do something
    // }

    // fn h(_v: Vec<i32>) {
    //     // Do something
    // }

    // let x = vec![10, 20, 30];
    // let c = true;

    // if c {
    //     f(x); // ... OK to move from x here
    // } else {
    //     g(x); // ... OK to also move from x here
    // }

    // h(x); // Bad: x is uninitialized here if either path uses it

    // fn cond() -> bool {
    //     // Do something
    //     true
    // }

    // let x = vec![10, 20, 30];

    // while cond() {
    //     g(x); // Bad: x would be moved in first iteration, uninitialized in second
    // }

    fn f() -> bool {
        false
    }

    fn g(_v: Vec<i32>) {
        // Do something
    }

    fn h() -> Vec<i32> {
        vec![20, 30, 40]
    }

    fn e(_v: Vec<i32>) {
        // Do something
    }

    let mut x = vec![10, 20, 30];

    while f() {
        g(x);       // Move from x
        x = h();    // Give x a fresh value
    }

    e(x);
}
