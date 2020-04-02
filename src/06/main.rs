use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::Read;
use std::iter::FromIterator;

fn main() {
    let mut input = String::new();

    File::open("./src/06/input.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let mut orbits: HashMap<&str, HashSet<&str>> = HashMap::new();

    input.lines().for_each(|l| {
        let split = l.split(')').collect::<Vec<&str>>();

        orbits
            .entry(split[0])
            .or_insert_with(HashSet::new)
            .insert(split[1]);
    });

    let orbit_closure = orbits
        .keys()
        .map(|k| (*k, get_closure(&orbits, *k)))
        .collect::<HashMap<&str, HashSet<&str>>>();

    println!(
        "Orbits: {}",
        orbit_closure.values().map(|v| v.len()).sum::<usize>()
    );

    let root = orbit_closure
        .iter()
        .filter(|(_, v)| v.contains("YOU") && v.contains("SAN"))
        .min_by_key(|(_, v)| v.len())
        .map(|(k, _)| *k)
        .expect("there is no connection between YOU and SAN");

    let dist_root_san = get_min_distance(&orbits, root, "SAN").unwrap() - 1;
    let dist_root_you = get_min_distance(&orbits, root, "YOU").unwrap() - 1;

    println!("Distance: {}", dist_root_san + dist_root_you)
}

fn get_closure<'a>(orbits: &HashMap<&'a str, HashSet<&'a str>>, current: &str) -> HashSet<&'a str> {
    let mut tmp: HashSet<&str> = HashSet::new();

    let mut todo = orbits
        .get(current)
        .map_or(VecDeque::new(), |k| VecDeque::from_iter(k.iter()));

    while let Some(p) = todo.pop_front() {
        tmp.insert(p);

        if let Some(k) = orbits.get(p) {
            k.iter().for_each(|q| todo.push_back(q));
        }
    }

    tmp
}

fn get_min_distance(orbits: &HashMap<&str, HashSet<&str>>, from: &str, to: &str) -> Option<usize> {
    let mut todo = orbits.get(from).map_or(VecDeque::new(), |k| {
        VecDeque::from_iter(k.iter().map(|v| (*v, 1)))
    });

    while let Some((p, dist)) = todo.pop_front() {
        if p == to {
            return Some(dist);
        }

        if let Some(k) = orbits.get(p) {
            k.iter().for_each(|q| todo.push_back((q, dist + 1)));
        }
    }

    None
}
