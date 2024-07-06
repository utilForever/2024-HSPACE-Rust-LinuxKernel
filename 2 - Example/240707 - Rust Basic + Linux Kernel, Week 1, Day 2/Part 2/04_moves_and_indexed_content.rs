fn main() {
    // Build a vector of the strings "101", "102", ... "105"
    let mut v = Vec::new();

    for i in 101..106 {
        v.push(i.to_string());
    }

    // Pull out random elements from the vector.
    // let third = v[2];   // Error: Cannot move out of index of Vec
    // let fifth = v[4];   // Here too

    // 1. Pop a value off the end of the vector:
    let fifth = v.pop().expect("vector empty!");
    assert_eq!(fifth, "105");

    // 2. Move a value out of a given index in the vector,
    // and move the last element into its spot:
    let second = v.swap_remove(1);
    assert_eq!(second, "102");

    // 3. Swap in another value for the one we're taking out:
    let third = std::mem::replace(&mut v[2], "substitute".to_string());
    assert_eq!(third, "103");

    // Let's see what's left of our vector.
    assert_eq!(v, vec!["101", "104", "substitute"]);

    let v = vec![
        "liberté".to_string(),
        "égalité".to_string(),
        "fraternité".to_string(),
    ];

    // This code consumes all their elements in a loop.
    for mut s in v {
        s.push('!');
        println!("{s}");
    }
}
