use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

struct Line {
    start: (i64, i64),
    end: (i64, i64),
    len: usize,
}

fn is_between(a: i64, b: i64, between: i64) -> bool {
    (a <= between && between <= b) || (b <= between && between <= a)
}

impl Line {
    fn crosses(&self, line: &Line) -> Option<(i64, i64)> {
        if is_between(self.start.0, self.end.0, line.start.0)
            && is_between(line.start.1, line.end.1, self.start.1)
        {
            return Some((line.start.0, self.start.1));
        }

        if is_between(line.start.0, line.end.0, self.start.0)
            && is_between(self.start.1, self.end.1, line.start.1)
        {
            return Some((self.start.0, line.start.1));
        }

        None
    }

    fn distance_to(&self, point: &(i64, i64)) -> Option<usize> {
        if self.start.0 == point.0 && is_between(self.start.1, self.end.1, point.1) {
            return Some((point.1 - self.start.1).abs() as usize);
        }

        if self.start.1 == point.1 && is_between(self.start.0, self.end.0, point.0) {
            return Some((point.0 - self.start.0).abs() as usize);
        }

        None
    }
}

fn main() {
    let mut input = String::new();

    File::open("./src/03/input.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let mut l1: Vec<Line> = Vec::new();
    let mut l2: Vec<Line> = Vec::new();

    for (i, l) in [&mut l1, &mut l2].iter_mut().enumerate() {
        let mut current = (0, 0);

        input.lines().nth(i).unwrap().split(',').for_each(|x| {
            let (d, n) = x.split_at(1);

            let len = n.parse::<usize>().unwrap();

            let end = match d {
                "U" => (current.0, current.1 - len as i64),
                "D" => (current.0, current.1 + len as i64),
                "L" => (current.0 - len as i64, current.1),
                "R" => (current.0 + len as i64, current.1),
                _ => panic!("wrong direction"),
            };

            let line = Line {
                start: current,
                end,
                len,
            };

            current = line.end;

            l.push(line);
        });
    }

    let mut intersections = HashSet::new();

    for line1 in &l1 {
        for line2 in &l2 {
            if let Some(x) = line1.crosses(&line2) {
                intersections.insert(x);
            }
        }
    }

    let distance = intersections
        .iter()
        .map(|x| x.0.abs() + x.1.abs())
        .filter(|x| *x > 0)
        .min()
        .unwrap();

    println!("Distance: {}", distance);

    let min_distance = intersections
        .iter()
        .map(|inter| {
            let mut distance = 0;

            for l in [&l1, &l2].iter() {
                for line in *l {
                    if let Some(d) = line.distance_to(inter) {
                        distance += d;
                        break;
                    } else {
                        distance += line.len;
                    }
                }
            }

            distance
        })
        .filter(|x| *x > 0)
        .min()
        .unwrap();

    println!("Minimum distance: {}", min_distance);
}

#[test]
fn is_between_test() {
    assert!(is_between(0, 0, 0));

    assert!(is_between(1, 3, 1));
    assert!(is_between(1, 3, 2));
    assert!(is_between(1, 3, 3));
    assert!(is_between(3, 1, 2));

    assert!(!is_between(1, 3, 4));
}

#[test]
fn cross_test() {
    let line1 = Line {
        start: (2, 0),
        end: (2, 10),
        len: 10,
    };

    let line2 = Line {
        start: (0, 3),
        end: (10, 3),
        len: 10,
    };

    assert_eq!(line1.crosses(&line2), Some((2, 3)));
}

#[test]
fn distance_to_test() {
    let line = Line {
        start: (2, 0),
        end: (2, 8),
        len: 8,
    };

    assert_eq!(line.distance_to(&(2, 3)), Some(3));
    assert_eq!(line.distance_to(&(2, 9)), None);
    assert_eq!(line.distance_to(&(4, 0)), None);
}
