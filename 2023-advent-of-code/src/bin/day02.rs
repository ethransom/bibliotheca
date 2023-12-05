// #![feature(test)]

use std::collections::HashMap;

// extern crate test;

const EXAMPLE: &str = include_str!("example02.txt");
const INPUT: &str = include_str!("input02.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    let games = dbg!(parse(input));

    let example = CubeSet {
        red: 12,
        green: 13,
        blue: 14,
    };

    let possibles = games
        .iter()
        .enumerate()
        .filter(|(_i, game)| {
            game.iter().all(|cube_set| {
                cube_set.red <= example.red
                    && cube_set.green <= example.green
                    && cube_set.blue <= example.blue
            })
        })
        .map(|(i, _game)| i + 1)
        .sum();

    (possibles, 0)
}

#[derive(Debug)]
struct CubeSet {
    red: usize,
    green: usize,
    blue: usize,
}

fn parse(input: &str) -> Vec<Vec<CubeSet>> {
    input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let prefix = format!("Game {}: ", i + 1);
            let line = line
                .strip_prefix(&prefix)
                .unwrap_or_else(|| panic!("'{line}' did not have prefix '{prefix}'"));

            line.split("; ")
                .map(|set| {
                    let mut result = CubeSet {
                        red: 0,
                        green: 0,
                        blue: 0,
                    };
                    set.split(", ").for_each(|part| {
                        let (count, color) = part.split_once(' ').unwrap();
                        match (count.parse::<usize>(), color) {
                            (Err(e), _) => panic!("{e}"),
                            (Ok(count), "red") => result.red = count,
                            (Ok(count), "green") => result.green = count,
                            (Ok(count), "blue") => result.blue = count,
                            (Ok(_), color) => panic!("unknown color {color}"),
                        }
                    });
                    result
                })
                .collect()
        })
        .collect()
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (8, 0));
}

#[test]
fn test_input() {
    assert_eq!(solve(INPUT), (2771, 0));
}
//
// #[bench]
// fn bench_solve_current(b: &mut test::Bencher) {
//     b.iter(|| {
//         assert_eq!(solve(INPUT), (0, 0));
//     });
// }
