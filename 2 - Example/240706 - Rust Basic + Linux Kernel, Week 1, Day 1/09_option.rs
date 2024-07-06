// enum Option<T> {
//     Some(T),
//     None,
// }

fn main() {
    let x: Option<i32> = Some(5);
    let y: Option<i32> = None;

    println!("{:?}", x);
    println!("{:?}", y);

    println!("{}", x.unwrap_or(0));

    match y {
        Option::Some(v) => println!("Value: {}", v),
        Option::None => println!("No value"),
    }
}
