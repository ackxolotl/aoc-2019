use std::fs::File;
use std::io::Read;

fn main() {
    let mut input = String::new();

    File::open("./src/02/input.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let mut tape: Vec<usize> = input
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    tape[1] = 12;
    tape[2] = 2;

    compute(&mut tape);

    println!("Value at position 0: {}", tape[0]);

    tape = input
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    for i in 0..=99 {
        for j in 0..=99 {
            let mut new_tape = tape.clone();

            new_tape[1] = i;
            new_tape[2] = j;

            compute(&mut new_tape);

            if new_tape[0] == 19_690_720 {
                println!("Noun: {}, verb: {}, solution: {}", i, j, 100 * i + j);
                break;
            }
        }
    }
}

fn compute(tape: &mut Vec<usize>) {
    let mut current = 0;

    loop {
        let opcode = tape[current];

        if opcode == 99 {
            break;
        }

        let out = tape[current + 3];
        let in1 = tape[current + 1];
        let in2 = tape[current + 2];

        match opcode {
            1 => tape[out] = tape[in1] + tape[in2],
            2 => tape[out] = tape[in1] * tape[in2],
            _ => panic!("wrong opcode"),
        };

        current += 4;
    }
}

#[test]
fn compute_test() {
    let mut vec = vec![1, 0, 0, 0, 99];
    compute(&mut vec);
    assert_eq!(vec![2, 0, 0, 0, 99], vec);
}
