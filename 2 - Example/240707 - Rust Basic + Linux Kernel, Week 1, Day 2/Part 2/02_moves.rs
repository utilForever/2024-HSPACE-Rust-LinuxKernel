fn main() {
    // This code will not compile because the variable `s` is moved to `t`.`
    // let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
    // let t = s;
    // let u = s;

    let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
    let t = s.clone();
    let u = s.clone();

    // More operations that move
    let mut s = "Govinda".to_string();
    s = "Siddhartha".to_string(); // Value "Govinda" dropped here

    let mut s = "Govinda".to_string();
    let t = s;
    s = "Siddhartha".to_string(); // Nothing is dropped here

    struct Person {
        name: String,
        birth: i32,
    }

    // Person instance is moved to the vector
    let mut composers = Vec::new();
    composers.push(Person {
        name: "Palestrina".to_string(),
        birth: 1525,
    });
}
