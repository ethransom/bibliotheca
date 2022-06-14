#![feature(test)]

extern crate test;

const EXAMPLE: &[u8] = include_bytes!("example07.txt");
const INPUT: &[u8] = include_bytes!("input07.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn minimize_cost(crabs: &[u32], cost: fn(u32) -> u32) -> u32 {
    let mut min_fuel = None;
    for i in *crabs.iter().min().unwrap()..*crabs.iter().max().unwrap() {
        let mut fuel = 0;
        for &crab in crabs {
            let dist = (crab as i64 - i as i64).abs();
            fuel += cost(dist as u32);
        }
        // println!("for pos {} fuel of {}", i, fuel);
        match min_fuel {
            None => min_fuel = Some(fuel),
            Some(old_min) => {
                if fuel < old_min {
                    min_fuel = Some(fuel)
                }
            }
        }
    }
    min_fuel.expect("NO SOLUTION")
}

fn solve(input: &[u8]) -> (u32, u32) {
    let crabs = std::str::from_utf8(input)
        .expect("input was not utf8")
        .split(',')
        .map(|n| n.parse().expect("not a number"))
        .collect::<Vec<u32>>();

    fn linear_cost(dist: u32) -> u32 {
        dist
    }
    fn exp_cost(dist: u32) -> u32 {
        (1..=dist).sum()
    }

    (
        minimize_cost(&crabs, linear_cost),
        minimize_cost(&crabs, exp_cost),
    )
}

#[bench]
fn bench_solve_example(b: &mut test::Bencher) {
    b.iter(|| solve(EXAMPLE));
}

#[bench]
fn bench_solve_example_str(b: &mut test::Bencher) {
    const EXAMPLE_STR: &str = include_str!("example07.txt");

    pub fn minimize_cost(crabs: &[u32], cost: fn(u32) -> u32) -> u32 {
        let mut min_fuel = None;
        for i in *crabs.iter().min().unwrap()..*crabs.iter().max().unwrap() {
            let mut fuel = 0;
            for &crab in crabs {
                let dist = (crab as i64 - i as i64).abs();
                fuel += cost(dist as u32);
            }
            // println!("for pos {} fuel of {}", i, fuel);
            match min_fuel {
                None => min_fuel = Some(fuel),
                Some(old_min) => {
                    if fuel < old_min {
                        min_fuel = Some(fuel)
                    }
                }
            }
        }
        min_fuel.expect("NO SOLUTION")
    }

    fn solve(input: &str) -> (u32, u32) {
        let crabs = input
            .split(",")
            .map(|n| n.parse().expect("not a number"))
            .collect::<Vec<u32>>();

        fn linear_cost(dist: u32) -> u32 {
            dist
        }
        fn exp_cost(dist: u32) -> u32 {
            (1..=dist).sum()
        }

        (
            minimize_cost(&crabs, linear_cost),
            minimize_cost(&crabs, exp_cost),
        )
    }

    b.iter(|| solve(EXAMPLE_STR));
}

#[bench]
fn bench_solve_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (342534, 94004208));
    });
}
