#![feature(test)]

use std::num::ParseIntError;

extern crate test;

const EXAMPLE: &str = include_str!("example01.txt");
const INPUT: &str = include_str!("input01.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (u64, u64) {
    let masses = parse(input);

    (
        masses.iter().cloned().map(module_fuel).sum::<u64>(),
        masses.iter().cloned().map(rocket_equation).sum::<u64>(),
    )
}

fn parse(input: &str) -> Vec<u64> {
    input
        .lines()
        .map(str::parse::<u64>)
        .collect::<Result<Vec<u64>, ParseIntError>>()
        .expect("couldn't parse input file")
}

fn module_fuel(mass: u64) -> u64 {
    mass / 3 - 2
}

fn rocket_equation(initial_mass: u64) -> u64 {
    let mut total_fuel = 0;

    let mut fuel: i64 = initial_mass as i64 / 3 - 2;
    loop {
        dbg!(initial_mass, total_fuel, fuel);

        if fuel <= 0 {
            break;
        }

        total_fuel += match u64::try_from(fuel) {
            Ok(fuel) => fuel,
            Err(_) => return total_fuel,
        };

        fuel = fuel / 3 - 2;
    }

    total_fuel
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (34_241, 51_316));
}

#[test]
fn test_module_fuel() {
    assert_eq!(module_fuel(12), 2);
    assert_eq!(module_fuel(14), 2);
    assert_eq!(module_fuel(1969), 654);
    assert_eq!(module_fuel(100_756), 33_583);
}

#[test]
fn test_rocket_equation() {
    assert_eq!(rocket_equation(12), 2);
    assert_eq!(rocket_equation(14), 2);
    assert_eq!(rocket_equation(1969), 966);
    assert_eq!(rocket_equation(100_756), 50_346);
}

#[bench]
fn bench_solve_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (3_167_282, 4_748_063));
    });
}

#[bench]
fn bench_solve_part1(b: &mut test::Bencher) {
    let masses = parse(INPUT);

    fn module_fuel(mass: u64) -> u64 {
        mass / 3 - 2
    }

    // see also: https://rust.godbolt.org/z/vz4cYKscd
    b.iter(|| {
        assert_eq!(
            masses.iter().cloned().map(module_fuel).sum::<u64>(),
            3_167_282
        );
    });
}
