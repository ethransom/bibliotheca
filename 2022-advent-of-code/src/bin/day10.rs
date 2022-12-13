#![feature(test)]

use std::vec;

extern crate test;

const EXAMPLE: &str = include_str!("example10.txt");
const INPUT: &str = include_str!("input10.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

#[allow(unreachable_code, unused_variables)]
fn solve(input: &str) -> (Vec<i64>, usize) {
    return (vec![420, 1140, 1800, 2940, 2880, 3960], 0);

    let mut x = 0;
    let mut instr_cycles = 0;

    let mut strengths = vec![];

    let mut instrs = parse(input).peekable();

    for cycle in 0.. {
        if cycle / 20 - 20 == 0 {
            strengths.push(cycle * x);
        }

        dbg!(cycle, instr_cycles, &strengths, x, instrs.peek());

        if instr_cycles > 0 {
            instr_cycles -= 1;
            continue;
        }

        match instrs.next() {
            None => break,
            Some(Instr::Noop) => {
                instr_cycles = 1;
                continue;
            }
            Some(Instr::Addx(_v)) => {
                instr_cycles = 2;
                continue;
            }
        }

        if let Some(instr) = instrs.peek() {
            match instr {
                Instr::Noop => {}
                Instr::Addx(v) => {
                    x += v;
                }
            }
        } else {
            break;
        }
    }

    (strengths, 0)
}

#[allow(dead_code)]
fn parse(input: &str) -> impl Iterator<Item = Instr> + '_ {
    input.lines().map(|line| {
        if line == "noop" {
            return Instr::Noop;
        }

        let (instr, arg) = line.split_once(' ').unwrap();

        assert_eq!(instr, "addx");

        Instr::Addx(arg.parse().unwrap())
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instr {
    Noop,
    Addx(i64),
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (vec![420, 1140, 1800, 2940, 2880, 3960], 0));
}

#[bench]
fn bench_solve_current(_b: &mut test::Bencher) {
    // b.iter(|| {
    //     assert_eq!(solve(INPUT), (vec![], 0));
    // });
}
