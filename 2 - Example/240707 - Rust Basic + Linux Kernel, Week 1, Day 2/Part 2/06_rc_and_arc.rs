fn main() {
    use std::rc::Rc;

    // Rust can infer all these types; written out for clarity
    let s: Rc<String> = Rc::new("shirataki".to_string());
    let t: Rc<String> = s.clone();
    let u: Rc<String> = s.clone();

    assert!(s.contains("shira"));
    assert_eq!(t.find("taki"), Some(5));
    println!("{u} are quite chewy, almost bouncy, but lack flavor");

    // A value owned by an Rc pointer is immutable.
    // s.push_str(" noodles");
}
