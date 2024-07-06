struct Player {
    name: String,
    level: i32,
    hp: i32,
    mp: i32,
}

impl Player {
    fn increase_level(&mut self) {
        self.level += 1;
        self.hp += 10;
        self.mp += 5;
    }
}

fn main() {
    let mut player = Player {
        name: String::from("Chris"),
        level: 1,
        hp: 100,
        mp: 50,
    };

    player.increase_level();

    println!("Player: {}", player.name);
    println!("Level: {}", player.level);
    println!("HP: {}", player.hp);
    println!("MP: {}", player.mp);
}
