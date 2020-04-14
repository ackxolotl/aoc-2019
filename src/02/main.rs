use std::fs::File;
use std::io::Read;

use intcode::Computer;

fn main() {
    let mut input = String::new();

    File::open("./src/02/input.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let computer = Computer::from_tape(&input);

    let mut c = computer.clone();

    c.set(1, 12);
    c.set(2, 2);

    c.compute();

    println!("Output: {}", c.get(0).unwrap());

    for i in 0..=99 {
        for j in 0..=99 {
            let mut c = computer.clone();

            c.set(1, i);
            c.set(2, j);

            c.compute();

            if c.get(0).unwrap() == 19_690_720 {
                println!("Noun: {}, verb: {}, solution: {}", i, j, 100 * i + j);
                return;
            }
        }
    }
}
