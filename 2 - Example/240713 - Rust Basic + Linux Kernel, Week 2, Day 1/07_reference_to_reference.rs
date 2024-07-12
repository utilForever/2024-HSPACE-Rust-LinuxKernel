fn main() {
    struct Point {
        x: i32,
        y: i32,
    }

    let point = Point { x: 1000, y: 729 };
    // Create a reference to `point` named `r`
    let r: &Point = &point;
    // Create a reference to the reference `r` named `rr`
    let rr: &&Point = &r;
    // Create a reference to the reference `rr` named `rrr`
    let rrr: &&&Point = &rr;

    // The dot operator automatically dereferences multiple levels of references
    // to access the `y` field of `point`. This line checks that `rrr.y` equals 729.
    assert_eq!(rrr.y, 729);
}
