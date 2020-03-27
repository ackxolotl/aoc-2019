use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut input = String::new();

    File::open("./src/03/input.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let mut l1: HashSet<(i64, i64)> = HashSet::new();
    let mut l2: HashSet<(i64, i64)> = HashSet::new();

    for (i, l) in [&mut l1, &mut l2].iter_mut().enumerate() {
        let mut current = (0, 0);

        input.lines().nth(i).unwrap().split(',').for_each(|x| {
            let (d, n) = x.split_at(1);

            match (d, n.parse::<i64>().unwrap()) {
                ("U", y) => {
                    (0..y).for_each(|z| {
                        l.insert((current.0, current.1 - z));
                    });

                    current.1 -= y
                }
                ("D", y) => {
                    (0..y).for_each(|z| {
                        l.insert((current.0, current.1 + z));
                    });

                    current.1 += y
                }
                ("L", y) => {
                    (0..y).for_each(|z| {
                        l.insert((current.0 - z, current.1));
                    });

                    current.0 -= y
                }
                ("R", y) => {
                    (0..y).for_each(|z| {
                        l.insert((current.0 + z, current.1));
                    });

                    current.0 += y
                }
                _ => panic!("wrong direction"),
            }
        });
    }

    let distance = l1
        .intersection(&l2)
        .map(|x| x.0.abs() + x.1.abs())
        .filter(|x| *x > 0)
        .min()
        .unwrap();

    println!("Distance: {:?}", distance);
}
