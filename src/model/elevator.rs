struct Elevator {
    name: String,
    curr_floor: int,
    dir: Direction,
}

enum Direction {
    Up,
    Down,
    Stop,
}


impl Elevator {
    // Constructor
    fn new(name: String) -> Elevator {
        return Elevator {name: start_floor, curr_floor: 0, dir: Direction::Stop}
    }
}
