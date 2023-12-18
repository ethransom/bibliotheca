// #![feature(test)]

// extern crate test;

const EXAMPLE: &str = include_str!("example15.txt");
const INPUT: &str = include_str!("input15.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    let sum = parse(input)
        .map(hash)
        .sum();

    (sum, 0)
}

fn hash(input: &[u8]) -> usize {
    input.iter().fold(0, |acc, &val| {
        ((acc + val as usize) * 17) % 256
    })
}

fn parse(input: &str) -> impl Iterator<Item=&[u8]> {
    input.split(',')
        .map(|step| step.as_bytes())
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (1320, 0));
}

#[test]
fn test_input() {
    assert_eq!(solve(INPUT), (516469, 0));
}

// #[bench]
// fn bench_solve_current(b: &mut test::Bencher) {
//     b.iter(|| {
//         assert_eq!(solve(INPUT), (0, 0));
//     });
// }
