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

    let phases = Permutation::new([0, 1, 2, 3, 4]);

    let mut in_out = 0;
    let mut max_thrust = 0;

    for phase in phases {
        let mut computers = vec![
            Computer::new(tape.clone()),
            Computer::new(tape.clone()),
            Computer::new(tape.clone()),
            Computer::new(tape.clone()),
            Computer::new(tape.clone()),
        ];

        for (i, c) in computers.iter_mut().enumerate() {
            c.push_input(phase[i] as i64);
            c.push_input(in_out);

            c.compute();

            in_out = c.pop_output().expect("no output from the amplifier");
        }

        max_thrust = max(in_out, max_thrust);

        in_out = 0;
    }

    println!("Maximum thrust: {}", max_thrust);

    let phases = Permutation::new([5, 6, 7, 8, 9]);

    let mut in_out = 0;
    let mut max_thrust = 0;

    for phase in phases {
        let mut computers = vec![
            Computer::new(tape.clone()),
            Computer::new(tape.clone()),
            Computer::new(tape.clone()),
            Computer::new(tape.clone()),
            Computer::new(tape.clone()),
        ];

        for (c, p) in computers.iter_mut().zip(phase.iter()) {
            c.push_input(*p as i64);
        }

        while computers.first().unwrap().is_running() {
            for c in computers.iter_mut() {
                c.push_input(in_out);

                c.compute_until_read();

                if let Some(x) = c.pop_output() {
                    in_out = x;
                } else {
                    break;
                }
            }
        }

        max_thrust = max(in_out, max_thrust);

        in_out = 0;
    }

    println!("Maximum thrust: {}", max_thrust);
}
