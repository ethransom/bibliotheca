// #![feature(test)]

// extern crate test;

use std::collections::HashMap;

const EXAMPLE: &str = include_str!("example08.txt");
const EXAMPLE2: &str = include_str!("example08_2.txt");
const INPUT: &str = include_str!("input08.txt");

fn main() {
    let (instructions, network) = parse(EXAMPLE);
    let steps = simple_follow(&network, &instructions);
    dbg!(steps);

    let (instructions, network) = parse(EXAMPLE2);
    let all_steps = ghost_follow(&network, &instructions);
    dbg!(all_steps);

    let (instructions, network) = parse(INPUT);
    let steps = simple_follow(&network, &instructions);
    let all_steps = ghost_follow(&network, &instructions);
    dbg!((steps, all_steps));
}

fn ghost_follow(network: &HashMap<&str, (&str, &str)>, instructions: &[char]) -> usize {
    let mut locations = network
        .keys()
        .cloned() // god, I love rust
        .filter(|&k| k.ends_with('A'))
        .collect::<Vec<_>>();
    let mut steps = 0;
    let mut instructions = instructions.iter().cycle();
    while locations.iter().any(|&l| !l.ends_with('Z')) {
        let instruction = *instructions.next().unwrap();
        for location in locations.iter_mut() {
            let (left, right) = network.get(location).unwrap();

            *location = if instruction == 'L' { left } else { right };
        }

        steps += 1;
    }
    steps
}

fn simple_follow(network: &HashMap<&str, (&str, &str)>, instructions: &Vec<char>) -> usize {
    let mut location = "AAA";
    let mut steps = 0;
    let mut instructions = instructions.iter().cycle();
    while location != "ZZZ" {
        let (left, right) = network.get(location).unwrap();

        location = if *instructions.next().unwrap() == 'L' {
            left
        } else {
            right
        };

        steps += 1;
    }

    steps
}

fn parse(input: &str) -> (Vec<char>, HashMap<&str, (&str, &str)>) {
    let (instructions, network) = input.split_once("\n\n").unwrap();

    let instructions = instructions.chars().collect::<Vec<_>>();

    let network = network
        .lines()
        .map(|line| {
            let (key, value) = line.split_once(" = ").unwrap();
            let (left, right) = (&value[1..=3], &value[6..=8]);

            (key, (left, right))
        })
        .collect();

    (instructions, network)
}

#[test]
fn test_example() {
    let (instructions, network) = parse(EXAMPLE);

    let steps = simple_follow(&network, &instructions);

    assert_eq!(steps, 6);
}

#[test]
fn test_example2() {
    let (instructions, network) = parse(EXAMPLE2);

    let all_steps = ghost_follow(&network, &instructions);
    assert_eq!(all_steps, 6);
}

#[test]
fn test_input() {
    let (instructions, network) = parse(INPUT);

    let steps = simple_follow(&network, &instructions);
    let all_steps = ghost_follow(&network, &instructions);

    assert_eq!((steps, all_steps), (18827, 0));
}

// #[bench]
// fn bench_solve_current(b: &mut test::Bencher) {
//     b.iter(|| {
//         assert_eq!(solve(INPUT), (0, 0));
//     });
// }
