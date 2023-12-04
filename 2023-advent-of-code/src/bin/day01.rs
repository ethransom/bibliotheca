// #![feature(test)]

// extern crate test;

const EXAMPLE: &str = include_str!("example01.txt");
const EXAMPLE2: &str = include_str!("example2_01.txt");
const INPUT: &str = include_str!("input01.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(EXAMPLE2));
    dbg!(solve(INPUT));
}

const NUMBER_WORDS: [(&str, usize); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

const NUMBER_DIGITS: [(&str, usize); 9] = [
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
];

fn find_patterns(mut input: &str, patterns: &[(&str, usize)]) -> Vec<usize> {
    let mut result = Vec::new();

    while !input.is_empty() {
        for (word, value) in patterns {
            if input.starts_with(word) {
                result.push(*value);
            }
        }

        input = &input[1..]
    }

    result
}

#[test]
fn test_find_first_pattern() {
    let patterns = (NUMBER_WORDS.into_iter().chain(NUMBER_DIGITS.into_iter())).collect::<Vec<_>>();

    assert_eq!(find_patterns("two", &patterns), vec![2]);
    assert_eq!(find_patterns("onetwothree", &patterns), vec![1, 2, 3]);
    assert_eq!(find_patterns("asdflajef", &patterns), vec![]);
    assert_eq!(find_patterns("twotwo", &patterns), vec![2, 2]);
}

fn solve(input: &str) -> (usize, usize) {
    let lines = parse(input);

    (
        sum_calibrations(&lines, &NUMBER_DIGITS),
        sum_calibrations(
            &lines,
            &NUMBER_WORDS
                .into_iter()
                .chain(NUMBER_DIGITS.into_iter())
                .collect::<Vec<_>>(),
        ),
    )
}

fn sum_calibrations(lines: &[&str], patterns: &[(&str, usize)]) -> usize {
    lines
        .iter()
        .map(|line| {
            let patterns = find_patterns(line, patterns);

            let first = patterns.first().unwrap_or(&0);
            let second = patterns.last().unwrap_or(&0);

            println!("{}:\t\t\t{:?} -> {}", line, patterns, first * 10 + second);

            first * 10 + second
        })
        .sum()
}

fn parse(input: &str) -> Vec<&str> {
    input.lines().collect::<Vec<&str>>()
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (142, 142));
}

#[test]
fn test_example2() {
    assert_eq!(solve(EXAMPLE2), (209, 281));
}

#[test]
fn test_input() {
    assert_eq!(solve(INPUT), (55834, 53221));
}

// #[bench]
// fn bench_solve_current(b: &mut test::Bencher) {
//     b.iter(|| {
//         assert_eq!(solve(INPUT), (0, 0));
//     });
// }
