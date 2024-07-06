trait Moveable {
    fn move_to(&mut self, x: i32, y: i32);
}

struct Player {
    name: String,
    level: i32,
    hp: i32,
    mp: i32,
    stamina: i32,
}

struct Pet {
    name: String,
    level: i32,
    exp: i32,
}

impl Moveable for Player {
    fn move_to(&mut self, x: i32, y: i32) {
        if self.stamina <= 0 {
            println!("Not enough stamina to move");
            return;
        }

        self.stamina -= 1;
        println!("Moving player to ({x}, {y})");
    }
}

impl Moveable for Pet {
    fn move_to(&mut self, x: i32, y: i32) {
        self.exp += 1;
        println!("Moving pet to ({x}, {y})");
    }
}

fn main() {
    let mut objects = vec![
        Box::new(Player {
            name: String::from("Chris"),
            level: 1,
            hp: 100,
            mp: 50,
            stamina: 100,
        }) as Box<dyn Moveable>,
        Box::new(Pet {
            name: String::from("Dog"),
            level: 1,
            exp: 0,
        }) as Box<dyn Moveable>,
    ];

    for object in objects.iter_mut() {
        object.move_to(10, 20);
    }
}
