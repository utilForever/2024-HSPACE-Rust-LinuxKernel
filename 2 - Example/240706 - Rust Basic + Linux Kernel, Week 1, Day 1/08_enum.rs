enum Status {
    Idle,
    Run(i32),
    Attack { damage: i32 },
    Defend { defense: i32 },
}

fn main() {
    let mut status = Status::Idle;
    status = Status::Run(10);
    status = Status::Attack { damage: 20 };
    status = Status::Defend { defense: 5 };
}
