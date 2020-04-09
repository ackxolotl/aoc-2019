use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use intcode::Computer;

struct Robot {
    position: (i32, i32),
    rotation: u8,
}

fn main() {
    let mut input = String::new();

    File::open("./src/11/input.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let tape: Vec<i64> = input
        .split(',')
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    let painted = paint(tape.clone(), 0);

    println!("Painted tiles: {}", painted.iter().count());

    let painted = paint(tape, 1);

    let x_min = painted.keys().map(|x| x.0).min().unwrap();
    let x_max = painted.keys().map(|x| x.0).max().unwrap();
    let y_min = painted.keys().map(|x| x.1).min().unwrap();
    let y_max = painted.keys().map(|x| x.1).max().unwrap();

    for y in y_min..=y_max {
        for x in x_min..=x_max {
            match painted.get(&(x, y)) {
                Some(1) => print!("\x1B[0m█"),
                _ => print!("\x1B[30m█"),
            }
        }

        println!();
    }
}

fn paint(tape: Vec<i64>, input: i64) -> HashMap<(i32, i32), u8> {
    let mut c = Computer::new(tape);

    let mut painted: HashMap<(i32, i32), u8> = HashMap::new();

    let mut robot = Robot {
        position: (0, 0),
        rotation: 0,
    };

    c.push_input(input);

    c.compute_until_read();

    while let Some(out) = c.pop_output() {
        if out == 1 || painted.get(&robot.position).is_some() {
            painted.insert(robot.position, out as u8);
        }

        c.compute_until_io();

        robot.rotation = match c.pop_output().unwrap() {
            0 => (robot.rotation + 3) % 4,
            1 => (robot.rotation + 1) % 4,
            _ => panic!("illegal direction"),
        };

        robot.position = match robot.rotation {
            0 => (robot.position.0, robot.position.1 + 1),
            1 => (robot.position.0 + 1, robot.position.1),
            2 => (robot.position.0, robot.position.1 - 1),
            3 => (robot.position.0 - 1, robot.position.1),
            _ => panic!("illegal robot rotation"),
        };

        let color = painted.get(&robot.position).unwrap_or(&0);

        c.push_input(*color as i64);

        c.compute_until_read();
    }

    painted
}
