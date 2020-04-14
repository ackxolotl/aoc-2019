use std::fs::File;
use std::io::Read;

use intcode::Computer;

fn main() {
    let mut input = String::new();

    File::open("./src/05/input.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let computer = Computer::from_tape(&input);

    let mut c = computer.clone();

    c.push_input(1);

    c.compute();

    while let Some(out) = c.pop_output() {
        println!("Output: {}", out);
    }

    let mut c = computer;

    c.push_input(5);

    c.compute();

    while let Some(out) = c.pop_output() {
        println!("Output: {}", out);
    }
}
