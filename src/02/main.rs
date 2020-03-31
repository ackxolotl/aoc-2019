use std::fs::File;
use std::io::Read;

use intcode::Computer;

fn main() {
    let mut input = String::new();

    File::open("./src/02/input.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let mut tape: Vec<i64> = input
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    tape[1] = 12;
    tape[2] = 2;

    let mut computer = Computer::new(tape);

    computer.compute();

    println!("Output: {}", computer.get_from_memory(0).unwrap());

    tape = input
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    for i in 0..=99 {
        for j in 0..=99 {
            let mut new_tape = tape.clone();

            new_tape[1] = i;
            new_tape[2] = j;

            let mut computer = Computer::new(new_tape);

            computer.compute();

            if computer.get_from_memory(0).unwrap() == 19_690_720 {
                println!("Noun: {}, verb: {}, solution: {}", i, j, 100 * i + j);
                return;
            }
        }
    }
}
