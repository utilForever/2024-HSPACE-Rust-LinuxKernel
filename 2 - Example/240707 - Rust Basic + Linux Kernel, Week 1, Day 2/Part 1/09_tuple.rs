fn main() {
    let text = "I see the eigenvalue in thine eye";
    let (head, tail) = text.split_at(21);

    assert_eq!(head, "I see the eigenvalue ");
    assert_eq!(tail, "in thine eye");

    // Above code is equivalent to below code, but is more legible
    let text = "I see the eigenvalue in thine eye";
    let temp = text.split_at(21);
    let head = temp.0;
    let tail = temp.1;

    assert_eq!(head, "I see the eigenvalue ");
    assert_eq!(tail, "in thine eye");
}
