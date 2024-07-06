fn main() {
    let num = 100;

    match num {
        0 => println!("Zero");
        1 | 3 | 5 | 7 | 9 => println!("1-digit Odd");
        2..=8 => println!("1-digit Even");
        num @ 10..=99 => println!("2-digit: {num}");
        _ => println!("3-digit or more");
    }
}
