use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use intcode::Computer;

fn main() {
    let mut input = String::new();

    File::open("./src/13/input.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let mut tape: Vec<i64> = input
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    let mut c = Computer::new(tape.clone());

    c.compute();

    let mut screen = HashMap::new();

    while let (Some(x), Some(y), Some(t)) = (c.pop_output(), c.pop_output(), c.pop_output()) {
        screen.insert((x, y), t);
    }

    println!(
        "Block tiles on screen: {}",
        screen.values().filter(|x| **x == 2).count()
    );

    tape[0] = 2;

    let mut c = Computer::new(tape);

    let mut paddle_x = 0;
    let mut score = 0;

    loop {
        for _ in 0..3 {
            c.compute_until_read();
        }

        let x_o = c.pop_output();
        let y_o = c.pop_output();
        let t_o = c.pop_output();

        if let (Some(x), Some(y), Some(t)) = (x_o, y_o, t_o) {
            match (x, y, t) {
                (-1, 0, s) => score = s,
                (a, _, 3) => paddle_x = a,
                (a, _, 4) if paddle_x < a => c.push_input(1),
                (a, _, 4) if paddle_x == a => c.push_input(0),
                (a, _, 4) if paddle_x > a => c.push_input(-1),
                _ => {}
            }
        } else {
            break;
        }
    }

    println!("Score: {}", score);
}
