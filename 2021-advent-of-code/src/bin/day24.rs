#![feature(test)]

use rayon::prelude::*;

extern crate test;

const EXAMPLE: &str = include_str!("example24.txt");
const INPUT: &str = include_str!("input24.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

#[derive(Debug)]
enum Operand {
    Reg(usize),
    Imm(i64),
}

#[derive(Debug)]
enum Instr {
    /// Read an input value and write it to variable a.
    Inp(Operand),

    /// Add the value of a to the value of b, then store the result in variable a.
    Add(Operand, Operand),

    /// Multiply the value of a by the value of b, then store the result in variable a.
    Mul(Operand, Operand),

    /// Divide the value of a by the value of b, truncate the result to an integer, then store the result in variable a. (Here, "truncate" means to round the value toward zero.)
    Div(Operand, Operand),

    /// Divide the value of a by the value of b, then store the remainder in variable a. (This is also called the modulo operation.)
    Mod(Operand, Operand),

    /// If the value of a and b are equal, then store the value 1 in variable a. Otherwise, store the value 0 in variable a.
    Eql(Operand, Operand),
}

impl From<&str> for Instr {
    fn from(line: &str) -> Instr {
        let (instr, args) = line.split_once(" ").unwrap();

        match instr {
            "inp" => {
                Instr::Inp(Operand::from(args))
            }
            "add" => {
                let (a, b) = args.split_once(" ").unwrap();
                Instr::Add(Operand::from(a), Operand::from(b))
            }
            "mul" => {
                let (a, b) = args.split_once(" ").unwrap();
                Instr::Mul(Operand::from(a), Operand::from(b))
            }
            "div" => {
                let (a, b) = args.split_once(" ").unwrap();
                Instr::Div(Operand::from(a), Operand::from(b))
            }
            "mod" => {
                let (a, b) = args.split_once(" ").unwrap();
                Instr::Mod(Operand::from(a), Operand::from(b))
            }
            "eql" => {
                let (a, b) = args.split_once(" ").unwrap();
                Instr::Eql(Operand::from(a), Operand::from(b))
            }
            _ => panic!("in C this would have corrupted my hard drive and also unplugged my mother's ventilator")
        }
    }
}

impl From<&str> for Operand {
    fn from(word: &str) -> Operand {
        if let Ok(n) = word.parse() {
            Operand::Imm(n)
        } else {
            Operand::Reg(match word {
                "w" => 0,
                "x" => 1,
                "y" => 2,
                "z" => 3,
                _ => panic!("unknown register"),
            })
        }
    }
}

fn solve(input: &str) -> (i64, i64) {
    let instrs = input.lines().map(Instr::from).collect::<Vec<Instr>>();

    let start = (1..14).fold(9 as i64, |n, _| n * 10 + 9);

    let end = start / 10;

    let highest_monad = (end..start)
        .rev()
        .find(|&i| {
            if i % 1_000_000 == 0 {
                println!("{}", i);
            }
            run_alu(&instrs, i as u64)
        })
        .expect("NO SOLUTION");

    (highest_monad, 0)
}

#[inline]
fn run_alu(instrs: &[Instr], input: u64) -> bool {
    let mut regs = [0; 4];

    let input = input.to_string();

    if input.chars().any(|c| c == '0') {
        return false;
    }

    let mut inputs = input.chars().map(|d| d.to_digit(10).unwrap() as i64);

    for instr in instrs {
        match *instr {
            Instr::Inp(Operand::Reg(dst)) => {
                let i = inputs.next().expect("asked for too many inputs");
                regs[dst] = i;
            }

            Instr::Add(Operand::Reg(dst), Operand::Reg(src)) => {
                regs[dst] = regs[dst] + regs[src];
            }

            Instr::Add(Operand::Reg(dst), Operand::Imm(src)) => {
                regs[dst] = regs[dst] + src;
            }

            Instr::Mul(Operand::Reg(dst), Operand::Reg(src)) => {
                regs[dst] = regs[dst] * regs[src];
            }

            Instr::Mul(Operand::Reg(dst), Operand::Imm(src)) => {
                regs[dst] = regs[dst] * src;
            }

            Instr::Div(Operand::Reg(dst), Operand::Reg(src)) => {
                regs[dst] = regs[dst] / regs[src];
            }

            Instr::Div(Operand::Reg(dst), Operand::Imm(src)) => {
                regs[dst] = regs[dst] / src;
            }

            Instr::Mod(Operand::Reg(dst), Operand::Reg(src)) => {
                regs[dst] = regs[dst] % regs[src];
            }

            Instr::Mod(Operand::Reg(dst), Operand::Imm(src)) => {
                regs[dst] = regs[dst] % src;
            }

            Instr::Eql(Operand::Reg(dst), Operand::Reg(src)) => {
                regs[dst] = (regs[dst] == regs[src]) as i64;
            }

            Instr::Eql(Operand::Reg(dst), Operand::Imm(src)) => {
                regs[dst] = (regs[dst] == src) as i64;
            }

            _ => {
                // dbg!(bad);
                panic!();
            }
        }
    }

    regs[3] == 0
}

#[test]
fn test_run_alu() {
    let instrs = EXAMPLE.lines().map(Instr::from).collect::<Vec<Instr>>();
    for i in 0..=9 {
        if i == 0 {
            continue;
        }
        assert_eq!(run_alu(&instrs, i), (i % 2) == 0);
    }

    let instrs = INPUT.lines().map(Instr::from).collect::<Vec<Instr>>();

    assert_eq!(run_alu(&instrs, 13579246899999), false);
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (0, 0));
}

#[bench]
fn bench_run_alu(b: &mut test::Bencher) {
    let instrs = INPUT.lines().map(Instr::from).collect::<Vec<Instr>>();

    b.iter(|| {
        assert_eq!(run_alu(&instrs, 13579246899999), false);
    });
}

#[bench]
fn bench_solve_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (0, 0));
    });
}
