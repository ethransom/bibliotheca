#![feature(test)]

use itertools::Itertools;

extern crate test;

const INPUT: &str = include_str!("input06.txt");

fn main() {
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    (
        first_distinct_window(input, 4).unwrap(),
        first_distinct_window(input, 14).unwrap(),
    )
}

#[test]
fn test_examples() {
    assert_eq!(solve("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), (7, 19));
    assert_eq!(solve("bvwbjplbgvbhsrlpgdmjqwftvncz"), (5, 23));
    assert_eq!(solve("nppdvjthqldpwncqszvftbrmjlhg"), (6, 23));
    assert_eq!(solve("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), (10, 29));
    assert_eq!(solve("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), (11, 26));
}

fn first_distinct_window(input: &str, size: usize) -> Option<usize> {
    // OTHER, NON-ITERTOOLS IDEAS:

    // if input.len() < 4 {
    //     return None;
    // }

    // let chars = input.chars();

    // let window: Vec<char> = chars.take(3).collect();

    // for char in chars {
    //     if window.iter().all(|c| c != char);
    //     window.rotate_left(mid);
    // }

    input
        .chars()
        .collect::<Vec<char>>()
        .windows(size)
        .position(|window| window.iter().unique().count() == window.len())
        .map(|pos| pos + size)
}

#[bench]
fn bench_solve_00_original(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (1953, 2301));
    });
}

#[bench]
fn bench_solve_01_better(b: &mut test::Bencher) {
    fn solve(input: &str) -> (usize, usize) {
        (
            first_distinct_window(input, 4).unwrap(),
            first_distinct_window(input, 14).unwrap(),
        )
    }

    fn first_distinct_window(input: &str, size: usize) -> Option<usize> {
        input
            .chars()
            .collect::<Vec<char>>()
            .windows(size)
            .position(|window| {
                window
                    .iter()
                    .enumerate()
                    .all(|(i, _)| !window[i + 1..].contains(&window[i]))
            })
            .map(|pos| pos + size)
    }

    assert_eq!(solve("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), (7, 19));
    assert_eq!(solve("bvwbjplbgvbhsrlpgdmjqwftvncz"), (5, 23));
    assert_eq!(solve("nppdvjthqldpwncqszvftbrmjlhg"), (6, 23));
    assert_eq!(solve("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), (10, 29));
    assert_eq!(solve("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), (11, 26));

    b.iter(|| {
        assert_eq!(solve(INPUT), (1953, 2301));
    });
}
