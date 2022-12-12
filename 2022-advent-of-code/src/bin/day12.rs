#![feature(test)]

extern crate test;

const EXAMPLE: &str = include_str!("example12.txt");
const INPUT: &str = include_str!("input12.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    let (heightmap, start, end) = parse(input);

    dbg!(&heightmap, start, end);

    (0, 0)
}

fn parse(input: &str) -> (Vec<Vec<u8>>, (usize, usize), (usize, usize)) {
    let mut start = None;
    let mut end = None;
    let heightmap = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.bytes()
                .enumerate()
                .map(|(x, b)| {
                    let b = match b {
                        b'S' => {
                            start = Some((x, y));
                            b'a'
                        }
                        b'E' => {
                            end = Some((x, y));
                            b'z'
                        }
                        b => b,
                    };
                    match b {
                        b'a'..=b'z' => b - b'a',
                        b => panic!("invalid byte: {}", b),
                    }
                })
                .collect()
        })
        .collect();

    (heightmap, start.unwrap(), end.unwrap())
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (31, 0));
}

#[bench]
fn bench_solve_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (0, 0));
    });
}
