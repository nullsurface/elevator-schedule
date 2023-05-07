mod model;

fn main() {
    let mut e1: model::elevator::Elevator = model::elevator::Elevator::new("Elevator One", 5);
    println!("{0}", e1.get_name());
    println!("{0}", e1.get_floor());
    e1.move_floor(2);
    println!("{0}", e1.get_name());
    println!("{0}", e1.get_floor());
    e1.move_floor(10);
    println!("{0}", e1.get_name());
    println!("{0}", e1.get_floor());
}
