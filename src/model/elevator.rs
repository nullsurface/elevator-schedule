struct Elevator {
    name: String,
    curr_floor: int,
    dir: Direction,
    max_floor: int,
    min_floor: int,
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

    // Move elevator X floors in a Direction
    fn move(&self, target_floor: int) {
        if target_floor > max_floor || target_floor < 0 {
            // Throw exception
        }

        if target_floor > start_floor {
            self.dir = Direciton.Up;
        } else if target_floor < start_floor {
            self.dir = Direciton.Down;
        }

        ratchet(target_floor - start_floor);
        self.dir = Direction.Stop;
    }

    // Go up each floor with a consistent speed
    fn rachet(&self, disp: int) {
        while () {

        }
    }
}
