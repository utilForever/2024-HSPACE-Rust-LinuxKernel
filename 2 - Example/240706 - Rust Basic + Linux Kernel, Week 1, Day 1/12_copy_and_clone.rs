fn hello(name: String) {
    println!("Hello, {name}");
}

fn square(num: i32) -> i32 {
    num * num
}

fn main() {
    // Only implement Clone trait, not Copy
    let name = String::from("Chris");
    hello(name.clone());
    hello(name);

    // Implement Copy and Clone traits
    let num = 5;
    println!("square({num}): {}", square(num));
    println!("square({}) : {}", num + 5, square(num + 5));
}
