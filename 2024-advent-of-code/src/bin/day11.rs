// #![feature(test)]

// extern crate test;

const EXAMPLE: &str = include_str!("example11.txt");
const INPUT: &str = include_str!("input11.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    let mut stones = parse(input);

    for i in 0..25 {
        stones = blink(&stones);
        println!("did {i}");
    }

    let after_25 = stones.len();

    for i in 0..25 {
        stones = blink(&stones);
        println!("did {i}", i = i + 25);
    }

    let after_75 = stones.len();

    (after_25, after_75)
}

fn blink(stones: &[u64]) -> Vec<u64> {
    let mut next = vec![];

    for stone in stones {
        match stone {
            0 => next.push(1),
            n if n.ilog10() % 2 == 1 => {
                let (left, right) = split(*n);
                next.push(left);
                next.push(right);
            }
            n => next.push(n * 2024),
        }
    }

    next
}

fn split(n: u64) -> (u64, u64) {
    let split = 1 + n.ilog10() / 2;
    (n / 10u64.pow(split), n % 10u64.pow(split))
    // let s = format!("{n}");
    // let (left, right) = s.split_at(s.len() / 2);
    // (left.parse().unwrap(), right.parse().unwrap())
}

#[test]
fn test_split() {
    assert_eq!(split(2024), (20, 24));

    assert_eq!(split(17), (1, 7));
}

fn parse(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|line| line.parse().unwrap())
        .collect()
}

#[test]
fn test_blink() {
    assert_eq!(blink(&[125, 17]), [253000, 1, 7]);
    assert_eq!(blink(&[253000, 1, 7]), [253, 0, 2024, 14168]);
    assert_eq!(blink(&[253, 0, 2024, 14168]), [512072, 1, 20, 24, 28676032]);
    assert_eq!(
        blink(&[512072, 1, 20, 24, 28676032]),
        [512, 72, 2024, 2, 0, 2, 4, 2867, 6032]
    );
    assert_eq!(
        blink(&[512, 72, 2024, 2, 0, 2, 4, 2867, 6032]),
        [1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32]
    );
    assert_eq!(
        blink(&[1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32]),
        [
            2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6, 0, 3,
            2
        ]
    );
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (55312, 0));
}

#[test]
fn test_input() {
    assert_eq!(solve(INPUT), (0, 0));
}

// #[bench]
// fn bench_solve_current(b: &mut test::Bencher) {
//     b.iter(|| {
//         assert_eq!(solve(INPUT), (0, 0));
//     });
// }
