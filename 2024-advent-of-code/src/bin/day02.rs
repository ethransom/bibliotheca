// #![feature(test)]
#![feature(iter_map_windows)]

// extern crate test;

use core::iter::Iterator;

const EXAMPLE: &str = include_str!("example02.txt");
const INPUT: &str = include_str!("input02.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    let reports = parse(input);

    let num_safe = reports.iter().filter(|r| report_is_safe(r)).count();

    let num_safe_dampener = reports
        .iter()
        .filter(|r| {
            // println!("{:?}", r);

            for dampened in 0..r.len() {
                let report = r
                    .iter()
                    .enumerate()
                    .filter_map(|(i, v)| if i == dampened { None } else { Some(*v) })
                    .collect::<Vec<_>>();

                // println!("\t{:?}", report);

                if report_is_safe(&report) {
                    return true;
                }
            }

            false
        })
        .count();

    (num_safe, num_safe_dampener)
}

fn report_is_safe(report: &[i16]) -> bool {
    let diffs = report
        .iter()
        .map_windows(|&[a, b]| a - b)
        .collect::<Vec<_>>();

    let all_increasing = diffs.iter().all(|&d| d < 0);
    let all_decreasing = diffs.iter().all(|&d| d > 0);
    let gradually_changing = diffs.iter().map(|d| d.abs()).all(|d| (1..=3).contains(&d));

    (all_increasing || all_decreasing) && (gradually_changing)
}

fn parse(input: &str) -> Vec<Vec<i16>> {
    input
        .lines()
        .map(|line: &str| {
            line.split_whitespace()
                .map(|level| level.parse().unwrap())
                .collect()
        })
        .collect()
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (2, 4));
}

#[test]
fn test_input() {
    assert_eq!(solve(INPUT), (516, 561));
}

// #[bench]
// fn bench_solve_current(b: &mut test::Bencher) {
//     b.iter(|| {
//         assert_eq!(solve(INPUT), (0, 0));
//     });
// }
