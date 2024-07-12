fn main() {
    // Part 1
    let x = 10;
    let r = &x;         // &x is a shared reference to x

    assert!(*r == 10);  // Explicitly dereference r

    // Part 2
    let mut y = 32;
    let m = &mut y;     // &mut y is a mutable reference to y
    *m += 32;           // Explicitly dereference m to set y's value

    assert!(*m == 64);  // and to see y's new value

    // Part 3
    struct Anime {
        name: &'static str,
        bechdel_pass: bool,
    }
    let aria = Anime {
        name: "Aria: The Animation",
        bechdel_pass: true,
    };
    let anime_ref = &aria;

    assert_eq!(anime_ref.name, "Aria: The Animation");
    // Equivalent to the above, but with the dereference written out:
    assert_eq!((*anime_ref).name, "Aria: The Animation");

    // Part 4
    let mut v = vec![1973, 1968];
    v.sort();           // Implicitly borrows a mutable reference to v
    (&mut v).sort();    // Equivalent, but more verbose
}
