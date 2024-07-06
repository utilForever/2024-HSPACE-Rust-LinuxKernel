fn main() {
    // Assignment of String "moves" the data.
    let string1 = "somnambulance".to_string();
    let string2 = string1;

    // Assignment of i32 "copies" the data.
    let num1: i32 = 36;
    let num2 = num1;

    // By default, struct and enum types are not Copy.
    // struct Label {
    //     number: u32,
    // }

    // fn print(l: Label) {
    //     println!("STAMP: {}", l.number);
    // }

    // let l = Label { number: 3 };

    // print(l);
    // println!("My label number is: {}", l.number);   // Error: value borrowed here after move

    // To make a struct or enum Copy, add the derive attribute.
    #[derive(Clone, Copy)]
    struct Label {
        number: u32,
    }

    fn print(l: Label) {
        println!("STAMP: {}", l.number);
    }

    let l = Label { number: 3 };
    
    print(l);
    println!("My label number is: {}", l.number);
}
