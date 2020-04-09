#![allow(clippy::float_cmp)]

use std::cmp::Ordering;
use std::collections::VecDeque;
use std::convert::TryInto;
use std::fs::File;
use std::io::Read;

struct Space {
    asteroid_belt: Vec<Vec<Option<Asteroid>>>,
}

impl Space {
    fn update_neighbors(&mut self) {
        for y_1 in 0..self.asteroid_belt.len() {
            for x_1 in 0..self.asteroid_belt[0].len() {
                if self.asteroid_belt[y_1][x_1].is_some() {
                    for y_2 in y_1..self.asteroid_belt.len() {
                        for x_2 in 0..self.asteroid_belt[0].len() {
                            if y_2 == y_1 && x_2 <= x_1 {
                                continue;
                            }

                            if self.asteroid_belt[y_2][x_2].is_some()
                                && self.in_sight(x_1, y_1, x_2, y_2)
                            {
                                self.asteroid_belt[y_1][x_1].as_mut().unwrap().neighbors += 1;
                                self.asteroid_belt[y_2][x_2].as_mut().unwrap().neighbors += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    fn in_sight(&self, x_1: usize, y_1: usize, x_2: usize, y_2: usize) -> bool {
        let delta_x = x_2 as i64 - x_1 as i64;
        let delta_y = y_2 as i64 - y_1 as i64;

        let gcd = gcd(delta_x.abs() as usize, delta_y.abs() as usize);

        let step_x = delta_x / gcd as i64;
        let step_y = delta_y / gcd as i64;

        let steps = match (step_x, step_y) {
            (0, 0) => panic!("oh no"),
            (0, _) => (delta_y / step_y) as usize,
            (_, _) => (delta_x / step_x) as usize,
        };

        let mut visible = true;

        for i in 1..steps {
            let x = (x_1 as i64 + i as i64 * step_x) as usize;
            let y = (y_1 as i64 + i as i64 * step_y) as usize;

            if self.asteroid_belt[y][x].is_some() {
                visible = false;
                break;
            }
        }

        visible
    }

    fn get_best_location(&self) -> (usize, usize) {
        let max = self
            .asteroid_belt
            .iter()
            .flatten()
            .flatten()
            .map(|x| x.neighbors)
            .max()
            .unwrap();

        for y in 0..self.asteroid_belt.len() {
            for x in 0..self.asteroid_belt[0].len() {
                if self.asteroid_belt[y][x].is_some()
                    && self.asteroid_belt[y][x].as_ref().unwrap().neighbors == max
                {
                    return (x, y);
                }
            }
        }

        panic!("no best location");
    }

    fn get_shooting_order(&self, my_x: usize, my_y: usize) -> VecDeque<(usize, usize, f64)> {
        let mut coordinates: Vec<(usize, usize, f64)> = Vec::new();

        for y in 0..self.asteroid_belt.len() {
            for x in 0..self.asteroid_belt[0].len() {
                if x == my_x && y == my_y {
                    continue;
                }

                let order =
                    ((x as i64 - my_x as i64) as f64).atan2((y as i64 - my_y as i64) as f64);

                coordinates.push((x, y, order));
            }
        }

        coordinates.sort_by(|(a, b, c), (d, e, f)| {
            if *c == *f {
                if (*a as i64 - my_x as i64).abs() + (*b as i64 - my_y as i64).abs()
                    < (*d as i64 - my_x as i64).abs() + (*e as i64 - my_y as i64).abs()
                {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            } else {
                f.partial_cmp(c).unwrap()
            }
        });

        coordinates.try_into().unwrap()
    }
}

#[derive(Debug)]
struct Asteroid {
    neighbors: usize,
}

fn gcd(a: usize, b: usize) -> usize {
    match b {
        0 => a,
        _ => gcd(b, a % b),
    }
}

fn main() {
    let mut input = String::new();

    File::open("./src/10/input.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let asteroid_belt: Vec<Vec<Option<Asteroid>>> = input
        .lines()
        .map(|x| {
            x.chars()
                .map(|y| {
                    if y == '#' {
                        Some(Asteroid { neighbors: 0 })
                    } else {
                        None
                    }
                })
                .collect()
        })
        .collect();

    let mut space = Space { asteroid_belt };

    space.update_neighbors();

    let best = space.get_best_location();

    println!(
        "Best asteroid: {}",
        space.asteroid_belt[best.1][best.0]
            .as_ref()
            .unwrap()
            .neighbors
    );

    let mut order = space.get_shooting_order(best.0, best.1);

    let mut vaporized = 0;
    let mut rotation = -0.0;

    while let Some((x, y, r)) = order.pop_front() {
        if space.asteroid_belt[y][x].is_some() {
            if r == rotation {
                order.push_back((x, y, r));
            } else if space.in_sight(best.0, best.1, x, y) {
                rotation = r;
                vaporized += 1;

                if vaporized == 200 {
                    println!("200th vaporized asteroid: {}", x * 100 + y);
                }
            }
        }
    }
}

#[test]
fn gcd_test() {
    assert_eq!(gcd(3, 7), 1);
    assert_eq!(gcd(2, 4), 2);
    assert_eq!(gcd(1071, 462), 21);
}
