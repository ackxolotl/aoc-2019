use std::fs::File;
use std::io::Read;

fn main() {
    let mut input = String::new();

    File::open("./src/01/input.txt").unwrap().read_to_string(&mut input).unwrap();

    let module_masses: Vec<usize> = input.lines().map(|l| l.parse::<usize>().unwrap()).collect();

    let fuel: usize = module_masses.iter().map(|x| (*x as i64 / 3 - 2) as usize).sum();

    println!("Required fuel: {}", fuel);

    let fuel: usize = module_masses.iter().map(|x| fuel_for_fuel(*x as i64)).sum();

    println!("Fuel + fuel required for fuel: {}", fuel);
}

fn fuel_for_fuel(fuel: i64) -> usize {
    match fuel / 3 - 2 {
        x if x <= 0 => 0,
        x => x as usize + fuel_for_fuel(x),
    }
}

#[test]
fn fuel_for_fuel_calculation() {
    assert_eq!(fuel_for_fuel(1969), 966);
    assert_eq!(fuel_for_fuel(100756), 50346);
}