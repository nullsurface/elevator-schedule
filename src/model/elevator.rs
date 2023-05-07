use std::thread;
use std::time::Duration;
use std::fmt;
use crate::model::constants;

pub struct Elevator<'name> {
    name: &'name str,
    curr_floor: i32,
    dir: Direction,
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Stop,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let direction = match *self {
            Direction::Up => "Up",
            Direction::Down => "Down",
            Direction::Stop => "Stop",
        };
        write!(f, "{}", direction)
    }
}


impl<'name> Elevator<'name> {
    // Constructor
    pub fn new(name: &'name str, start_floor: i32) -> Elevator<'name> {
        return Elevator {name:name, curr_floor:start_floor, dir: Direction::Stop}
    }

    // Move elevator X floors in a Direction
    pub fn move_floor(&mut self, target_floor: i32) {
        let disp = target_floor - self.curr_floor;
        if disp > 0 {
            self.dir = Direction::Up;
        } else {
            self.dir = Direction::Down;
        }

        println!("Direction: {0}", self.dir);

        self.rachet(disp.abs());
        self.dir = Direction::Stop;
    }

    // Go up each floor with a consistent speed
    fn rachet(&mut self, disp: i32) {
        let opt = match self.dir {
            Direction::Up => 1,
            Direction::Down => -1,
            Direction::Stop => 0,
        };

        println!("Moving {0}, {1}", self.dir, opt);
        
        for floor in 0..disp {
            thread::sleep(Duration::from_secs(constants::WAIT_TIME_BETWEEN_FLOORS));
            self.curr_floor += opt;
            println!("On Floor: {0}", self.get_floor());
        }
        println!("Done!");
    }

    pub fn get_name(&self) -> &str {
        return self.name;
    }

    pub fn get_floor(&self) -> i32 {
        return self.curr_floor;
    }

    pub fn get_dir(&self) -> Direction {
        return self.dir;
    }
}
