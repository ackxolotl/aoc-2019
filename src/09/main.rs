use std::fs::File;
use std::io::Read;

use intcode::Computer;

fn main() {
    let mut input = String::new();

    File::open("./src/09/input.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let computer = Computer::from_tape(&input);

    let mut c = computer.clone();

    c.push_input(1);

    c.compute();

    println!("BOOST keycode: {}", c.pop_output().unwrap());

    let mut c = computer;

    c.push_input(2);

    c.compute();

    println!("Coordinates of signal: {}", c.pop_output().unwrap());
}
