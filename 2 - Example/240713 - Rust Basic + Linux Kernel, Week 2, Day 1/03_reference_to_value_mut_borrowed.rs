use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

fn sort_works(table: &mut Table) {
    for (_artist, works) in table {
        works.sort();
    }
}

fn show(table: &Table) {
    for (artist, works) in table {
        println!("works by {artist}:");

        for work in works {
            println!("  {work}");
        }
    }
}

fn main() {
    let mut table = Table::new();

    table.insert(
        "Gesualdo".to_string(),
        vec![
            "many madrigals".to_string(),
            "Tenebrae Responsoria".to_string(),
        ],
    );
    table.insert(
        "Caravaggio".to_string(),
        vec![
            "The Musicians".to_string(),
            "The Calling of St. Matthew".to_string(),
        ],
    );
    table.insert(
        "Cellini".to_string(),
        vec![
            "Persion with the head of Medusa".to_string(),
            "a salt cellar".to_string(),
        ],
    );

    sort_works(&mut table);     // Case 1: Borrow table mutably

    // After sort_works function is returned, the ownership of table is returned to the main function

    show(&table);               // Case 2: Borrow table immutably
}
