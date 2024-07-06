// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

fn square_if_even(num: i32) -> Result<i32, String> {
    if num % 2 == 0 {
        Ok(num * num)
    } else {
        Err(String::from("Not even"))
    }
}

fn main() {
    let num1 = 4;
    let num2 = 5;

    match square_if_even(num1) {
        Ok(v) => println!("Result: {v}"),
        Err(e) => println!("Error: {e}"),
    }

    match square_if_even(num2) {
        Ok(v) => println!("Result: {v}"),
        Err(e) => println!("Error: {e}"),
    }
}
