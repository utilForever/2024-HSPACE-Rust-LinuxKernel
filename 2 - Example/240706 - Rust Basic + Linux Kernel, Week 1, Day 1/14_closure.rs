fn call_twice<F>(closure: F) where F: Fn() {
    closure();
    closure();
}

fn main() {
    let name = String::from("Chris");
    let hello = || {
        println!("Hello, {name}");
        drop(name);
    };

    call_twice(hello);
}
