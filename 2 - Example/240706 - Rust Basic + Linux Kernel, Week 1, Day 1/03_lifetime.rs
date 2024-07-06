#[derive(Debug)]
struct Number(i32);

// fn min(a: &Number, b: &Number) -> &Number {
//     if a.0 < b.0 {
//         a
//     } else {
//         b
//     }
// }

fn min<'a>(a: &'a Number, b: &'a Number) -> &'a Number {
    if a.0 < b.0 {
        a
    } else {
        b
    }
}

fn main() {
    let num1 = Number(5);
    let num2 = Number(24);
    let num3 = min(&num1, &num2);

    println!("num3: {num3:?}");
}
