use std::fs::File;
use std::io::Read;

mod permutation;

use intcode::Computer;
use permutation::*;
use std::cmp::max;

fn main() {
    let mut input = String::new();

    File::open("./src/07/input.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let tape: Vec<i64> = input
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    let input = 0;

    let phases = Permutation::new([0, 1, 2, 3, 4]);

    let mut max_out = 0;

    for p in phases {
        #[rustfmt::skip]
        let out = amp(&tape, p[4], amp(&tape, p[3], amp(&tape, p[2], amp(&tape, p[1], amp(&tape, p[0], input)))));

        max_out = max(out, max_out);
    }

    println!("Maximum signal: {}", max_out);
}

fn amp(tape: &Vec<i64>, phase: u8, input: i64) -> i64 {
    let mut c = Computer::new(tape.clone());

    c.push_input(phase as i64);
    c.push_input(input);

    c.compute();

    c.pop_output().expect("no output from the amplifier")
}
