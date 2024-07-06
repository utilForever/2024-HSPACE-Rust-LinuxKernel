fn main() {
    let mut arr = vec![1.2, 4.5, 3.1, -5.7, 6.3];

    // Can't use arr.sort() because f64 doesn't implement Ord
    // arr.sort();

    // Instead, use sort_by
    arr.sort_by(|a, b| a.partial_cmp(b).unwrap());

    println!("{:?}", arr);
}
