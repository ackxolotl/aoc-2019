use std::fs::File;
use std::io::Read;

use intcode::Computer;

fn main() {
    let mut input = String::new();

    File::open("./src/05/input.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let tape: Vec<i64> = input
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    let mut computer = Computer::new(tape.clone());

    computer.push_input(1);

    computer.compute();

    while let Some(out) = computer.pop_output() {
        println!("Output: {}", out);
    }

    let mut computer = Computer::new(tape);

    computer.push_input(5);

    computer.compute();

    while let Some(out) = computer.pop_output() {
        println!("Output: {}", out);
    }
}
