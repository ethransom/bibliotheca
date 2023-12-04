// #![feature(test)]

// extern crate test;

const EXAMPLE: &str = include_str!("example01.txt");
const INPUT: &str = include_str!("input01.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    let lines = parse(input);

    (
        lines
            .iter()
            .map(|line| {
                let first = line
                    .chars()
                    .find(|&c| c.is_ascii_digit())
                    .expect("No digit found")
                    .to_digit(10)
                    .unwrap() as usize;
                let last = line
                    .chars()
                    .rev()
                    .find(|&c| c.is_ascii_digit())
                    .expect("No digit found")
                    .to_digit(10)
                    .unwrap() as usize;

                println!("{}:\t\t\t{} + {}", line, first, last);

                first * 10 + last
            })
            .sum(),
        0,
    )
}

fn parse(input: &str) -> Vec<&str> {
    input.lines().collect::<Vec<&str>>()
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (142, 0));
}

#[test]
fn test_input() {
    assert_eq!(solve(INPUT), (55834, 0));
}

// #[bench]
// fn bench_solve_current(b: &mut test::Bencher) {
//     b.iter(|| {
//         assert_eq!(solve(INPUT), (0, 0));
//     });
// }
