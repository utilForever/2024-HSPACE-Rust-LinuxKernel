fn main() {
    fn print_padovan() {
        let mut padovan = vec![1, 1, 1];  // Allocated here

        for i in 3..10 {
            let next = padovan[i - 3] + padovan[i - 2];
            padovan.push(next);
        }

        println!("P(1..10) = {:?}", padovan);
    }  // Dropped here

    print_padovan();

    {
        let point = Box::new((0.625, 0.5)); // Point allocated here
        let label = format!("{:?}", point);          // Label allocated here

        assert_eq!(label, "(0.625, 0.5)");
    } // Both dropped here

    // The ownershop of complex types
    struct Person {
        name: String,
        birth: i32,
    }

    let mut composers = Vec::new();
    composers.push(Person {
        name: "Palestrina".to_string(),
        birth: 1525,
    });
    composers.push(Person {
        name: "Dowland".to_string(),
        birth: 1563,
    });
    composers.push(Person {
        name: "Lully".to_string(),
        birth: 1632,
    });

    for composer in &composers {
        println!("{}, born {}", composer.name, composer.birth);
    }
}
