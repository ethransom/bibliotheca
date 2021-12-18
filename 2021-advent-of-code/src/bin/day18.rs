#![feature(test)]

extern crate test;

const EXAMPLE: &str = include_str!("example18.txt");
const INPUT: &str = include_str!("input18.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    let numbers: Vec<Pair> = input.lines().map(Pair::from).collect();

    dbg!(numbers);

    (0, 0)
}

#[derive(PartialEq, Debug)]
struct Pair {
    left: Arm,
    right: Arm,
}

#[derive(PartialEq, Debug)]
enum Arm {
    Num(u32),
    Branch(Box<Pair>),
}
use Arm::{Branch, Num};

impl From<&str> for Pair {
    fn from(line: &str) -> Pair {
        let mut chars = line.chars().peekable();

        use std::iter::Peekable;
        use std::str::Chars;

        fn recurse(chars: &mut Peekable<Chars>) -> Arm {
            if let Some('[') = chars.peek() {
                assert_eq!(chars.next().expect("unexpected EOF"), '[');

                let left = recurse(chars);

                assert_eq!(chars.next().expect("unexpected EOF"), ',');

                let right = recurse(chars);

                assert_eq!(chars.next().expect("unexpected EOF"), ']');

                Branch(Box::new(Pair { left, right }))
            } else {
                Num(chars
                    .next()
                    .expect("unexpected EOF")
                    .to_digit(10)
                    .expect("unexpected char"))
            }
        }

        match recurse(&mut chars) {
            Branch(pair) => {
                assert_eq!(chars.next(), None);
                *pair
            }
            Num(_) => {
                panic!("root number must be a pair")
            }
        }
    }
}

#[test]
fn test_parse() {
    assert_eq!(
        Pair::from("[1,2]"),
        Pair {
            left: Num(1),
            right: Num(2)
        }
    );
    assert_eq!(
        Pair::from("[[1,2],3]"),
        Pair {
            left: Branch(Box::new(Pair {
                left: Num(1),
                right: Num(2)
            })),
            right: Num(3)
        }
    );
    assert_eq!(
        Pair::from("[9,[8,7]]"),
        Pair {
            left: Num(9),
            right: Branch(Box::new(Pair {
                left: Num(8),
                right: Num(7)
            }))
        }
    );
    assert_eq!(
        Pair::from("[[1,9],[8,5]]"),
        Pair {
            left: Branch(Box::new(Pair {
                left: Num(1),
                right: Num(9)
            })),
            right: Branch(Box::new(Pair {
                left: Num(8),
                right: Num(5)
            }))
        }
    );

    // assert no panic
    let _ = Pair::from("[[[[1,2],[3,4]],[[5,6],[7,8]]],9]");
    let _ = Pair::from("[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]");
    let _ = Pair::from("[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]");
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (0, 0));
}

#[bench]
fn bench_solve_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (0, 0));
    });
}
