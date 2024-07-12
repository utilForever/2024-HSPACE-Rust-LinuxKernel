fn main() {
    let x = 10;
    let y = 10;

    let rx = &x;
    let ry = &y;

    let rrx = &rx;
    let rry = &ry;

    // Like the dot operator, the comparison operator
    // automatically dereferences multiple levels of references
    assert!(rrx <= rry);
    assert!(rrx == rry);

    assert!(rx == ry);                  // Their referents are equal
    assert!(!std::ptr::eq(rx, ry));     // but occupy different addresses

    // assert!(rx == rrx);     // Error: type mismatch: `&i32` vs `&&i32`
    assert!(rx == *rrx);    // This is okay
}
