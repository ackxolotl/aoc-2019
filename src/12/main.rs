use std::cmp::Ordering;
use std::convert::TryInto;
use std::fs::File;
use std::io::Read;

#[derive(Copy, Clone, Debug)]
struct Moon {
    position: [i32; 3],
    velocity: [i32; 3],
}

impl Moon {
    fn compute_velocity(&self, moons: &[Moon]) -> [i32; 3] {
        let mut velocity = self.velocity;

        for moon in moons {
            for (i, p) in moon.position.iter().enumerate() {
                velocity[i] += match self.position[i].cmp(p) {
                    Ordering::Greater => -1,
                    Ordering::Less => 1,
                    _ => 0,
                }
            }
        }

        velocity
    }

    fn compute_energy(&self) -> usize {
        self.position
            .iter()
            .map(|x| x.abs() as usize)
            .sum::<usize>()
            * self
                .velocity
                .iter()
                .map(|x| x.abs() as usize)
                .sum::<usize>()
    }
}

fn main() {
    let mut input = String::new();

    File::open("./src/12/input.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let moons: Vec<Moon> = input
        .split(&['=', ',', '>'][..])
        .filter_map(|x| x.parse::<i32>().ok())
        .collect::<Vec<i32>>()
        .chunks(3)
        .map(|x| Moon {
            position: x.try_into().unwrap(),
            velocity: [0, 0, 0],
        })
        .collect();

    let mut moons_one = moons.clone();

    for _ in 0..1000 {
        simulate_step(&mut moons_one);
    }

    println!(
        "Total energy: {}",
        moons_one.iter().map(|m| m.compute_energy()).sum::<usize>()
    );

    let x_s = [
        moons[0].position[0],
        moons[1].position[0],
        moons[2].position[0],
        moons[3].position[0],
    ];
    let y_s = [
        moons[0].position[1],
        moons[1].position[1],
        moons[2].position[1],
        moons[3].position[1],
    ];
    let z_s = [
        moons[0].position[2],
        moons[1].position[2],
        moons[2].position[2],
        moons[3].position[2],
    ];

    let x_cycle = cycle_length(&x_s);
    let y_cycle = cycle_length(&y_s);
    let z_cycle = cycle_length(&z_s);

    println!(
        "Universe repeats after {} steps",
        lcm(x_cycle, lcm(y_cycle, z_cycle))
    );
}

fn simulate_step(moons: &mut Vec<Moon>) {
    *moons = moons
        .iter()
        .map(|m| {
            let velocity = m.compute_velocity(&moons);
            let position = [
                m.position[0] + velocity[0],
                m.position[1] + velocity[1],
                m.position[2] + velocity[2],
            ];

            Moon { position, velocity }
        })
        .collect();
}

fn cycle_length(coordinates: &[i32; 4]) -> usize {
    let mut c = *coordinates;
    let mut v = [0; 4];

    let mut cycle = 0;

    loop {
        cycle += 1;

        // update velocity
        for i in 0..4 {
            v[i] += ((c[0] > c[i]) as u8
                + (c[1] > c[i]) as u8
                + (c[2] > c[i]) as u8
                + (c[3] > c[i]) as u8) as i32
                - ((c[0] < c[i]) as u8
                    + (c[1] < c[i]) as u8
                    + (c[2] < c[i]) as u8
                    + (c[3] < c[i]) as u8) as i32;
        }

        if (v[0] | v[1] | v[2] | v[3]) == 0 && c == *coordinates {
            break;
        }

        // update position
        c[0] += v[0];
        c[1] += v[1];
        c[2] += v[2];
        c[3] += v[3];
    }

    cycle
}

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

fn gcd(a: usize, b: usize) -> usize {
    match b {
        0 => a,
        _ => gcd(b, a % b),
    }
}
