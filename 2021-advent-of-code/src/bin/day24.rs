#![feature(test)]
#![feature(derive_default_enum)]

extern crate test;

use std::mem;

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
        let (instr, args) = line.split_once(' ').unwrap();

        match instr {
            "inp" => {
                Instr::Inp(Operand::from(args))
            }
            "add" => {
                let (a, b) = args.split_once(' ').unwrap();
                Instr::Add(Operand::from(a), Operand::from(b))
            }
            "mul" => {
                let (a, b) = args.split_once(' ').unwrap();
                Instr::Mul(Operand::from(a), Operand::from(b))
            }
            "div" => {
                let (a, b) = args.split_once(' ').unwrap();
                Instr::Div(Operand::from(a), Operand::from(b))
            }
            "mod" => {
                let (a, b) = args.split_once(' ').unwrap();
                Instr::Mod(Operand::from(a), Operand::from(b))
            }
            "eql" => {
                let (a, b) = args.split_once(' ').unwrap();
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

    let regs = simplify(&instrs);

    dbg!(regs[3].node_count());

    (0, 0)
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Type {
    Add,
    Mul,
    Div,
    Mod,
    Eql,
}
#[derive(Debug, Default, Clone, Eq, PartialEq)]
enum Value {
    #[default]
    None,

    Imm(i64),
    Input(usize),
    Op(Type, Box<Value>, Box<Value>),
}

impl Value {
    fn node_count(&self) -> usize {
        let mut count = 1;

        if let Value::Op(_type, left, right) = self {
            count += left.node_count();
            count += right.node_count();
        }

        count
    }

    fn input_is_solution(&self, input: u64) -> bool {
        let input = input.to_string();

        if input.chars().any(|c| c == '0') {
            return false;
        }

        let inputs: Vec<i64> = input
            .chars()
            .map(|d| d.to_digit(10).unwrap() as i64)
            .collect();

        self.eval(&inputs) == 0
    }

    fn eval(&self, inputs: &[i64]) -> i64 {
        match self {
            Value::None => panic!("cannot evaluate None"),
            &Value::Imm(v) => v,
            &Value::Input(i) => inputs[i],
            Value::Op(kind, left, right) => {
                let left = left.eval(inputs);
                let right = right.eval(inputs);
                match kind {
                    Type::Add => left + right,
                    Type::Mul => left * right,
                    Type::Div => left / right,
                    Type::Mod => left % right,
                    Type::Eql => (left == right) as i64,
                }
            }
        }
    }
}

#[test]
fn test_node_count() {
    let instrs = EXAMPLE.lines().map(Instr::from).collect::<Vec<Instr>>();
    let regs = simplify(&instrs);
    assert_eq!(regs[0].node_count(), 9);
    assert_eq!(regs[1].node_count(), 9);
    assert_eq!(regs[2].node_count(), 7);
    assert_eq!(regs[3].node_count(), 5);
}

fn simplify(instrs: &[Instr]) -> [Value; 4] {
    let mut regs = [Value::Imm(0), Value::Imm(0), Value::Imm(0), Value::Imm(0)];

    let mut input = 0;

    for (i, instr) in instrs.iter().enumerate() {
        dbg!(i, instr, regs[3].node_count());

        match &instr {
            Instr::Inp(Operand::Reg(reg)) => {
                regs[*reg] = Value::Input(input);
                input += 1;
            }

            Instr::Add(Operand::Reg(left), right) => {
                let right = match right {
                    Operand::Imm(v) => Value::Imm(*v),
                    Operand::Reg(r) => regs[*r].clone(),
                };
                regs[*left] = Value::Op(
                    Type::Add,
                    Box::new(mem::take(&mut regs[*left])),
                    Box::new(right),
                );
            }

            Instr::Mul(Operand::Reg(left), right) => {
                let right = match right {
                    Operand::Imm(v) => Value::Imm(*v),
                    Operand::Reg(r) => regs[*r].clone(),
                };
                regs[*left] = Value::Op(
                    Type::Mul,
                    Box::new(mem::take(&mut regs[*left])),
                    Box::new(right),
                );
            }

            Instr::Div(Operand::Reg(left), right) => {
                let right = match right {
                    Operand::Imm(v) => Value::Imm(*v),
                    Operand::Reg(r) => regs[*r].clone(),
                };
                regs[*left] = Value::Op(
                    Type::Div,
                    Box::new(mem::take(&mut regs[*left])),
                    Box::new(right),
                );
            }

            Instr::Mod(Operand::Reg(left), right) => {
                let right = match right {
                    Operand::Imm(v) => Value::Imm(*v),
                    Operand::Reg(r) => regs[*r].clone(),
                };
                regs[*left] = Value::Op(
                    Type::Mod,
                    Box::new(mem::take(&mut regs[*left])),
                    Box::new(right),
                );
            }

            Instr::Eql(Operand::Reg(left), right) => {
                let right = match right {
                    Operand::Imm(v) => Value::Imm(*v),
                    Operand::Reg(r) => regs[*r].clone(),
                };
                regs[*left] = Value::Op(
                    Type::Eql,
                    Box::new(mem::take(&mut regs[*left])),
                    Box::new(right),
                );
            }

            _ => unimplemented!("can't handle instr: {:?}", instr),
        }
    }
    regs
}

#[test]
fn test_simplify() {
    let instrs = EXAMPLE.lines().map(Instr::from).collect::<Vec<Instr>>();

    let regs = simplify(&instrs);

    assert_eq!(
        regs[3],
        Value::Op(
            Type::Mod,
            Box::new(Value::Op(
                Type::Add,
                Box::new(Value::Imm(0)),
                Box::new(Value::Input(0))
            )),
            Box::new(Value::Imm(2))
        )
    );

    // // Too slow :/
    // let instrs = INPUT.lines().map(Instr::from).collect::<Vec<Instr>>();
    //
    // let regs = simplify(&instrs);
}

#[bench]
fn bench_simplify_small_input(b: &mut test::Bencher) {
    let instrs = EXAMPLE.lines().map(Instr::from).collect::<Vec<Instr>>();

    b.iter(|| simplify(&instrs))
}

#[bench]
fn bench_simplify_large_input(b: &mut test::Bencher) {
    let instrs = INPUT.lines().map(Instr::from).collect::<Vec<Instr>>();

    b.iter(|| simplify(&instrs))
}

#[test]
fn test_input_is_solution() {
    let instrs = EXAMPLE.lines().map(Instr::from).collect::<Vec<Instr>>();

    let regs = simplify(&instrs);

    for i in 0..=9 {
        if i == 0 {
            continue;
        }
        assert_eq!(regs[3].input_is_solution(i), (i % 2) == 0);
    }

    let instrs = INPUT.lines().map(Instr::from).collect::<Vec<Instr>>();

    let _regs = simplify(&instrs);

    // assert_eq!(regs[3].input_is_solution(13579246899999), false);
}

#[bench]
fn bench_input_is_solution(b: &mut test::Bencher) {
    let instrs = EXAMPLE.lines().map(Instr::from).collect::<Vec<Instr>>();

    let regs = simplify(&instrs);

    b.iter(|| regs[3].input_is_solution(8))
    //
    // let instrs = INPUT.lines().map(Instr::from).collect::<Vec<Instr>>();
    //
    // let regs = simplify(&instrs);

    // assert_eq!(regs[3].input_is_solution(13579246899999), false);
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (8, 0));
}

#[bench]
fn bench_run_alu(b: &mut test::Bencher) {
    ///
    /// An extremely ill-advised naive attempt to search all 14 digit numbers starting with the
    /// largest. However! Runs in something like ~0.5µs (500ns!!) to check a single number on my
    /// machine? I was considering simply parallelizing this to the extreme.
    ///
    /// ## Example:
    /// ```
    /// fn solve(input: &str) -> (i64, i64) {
    ///     let instrs = input.lines().map(Instr::from).collect::<Vec<Instr>>();
    ///
    ///     let start = (1..14).fold(9 as i64, |n, _| n * 10 + 9);
    ///
    ///     let end = start / 10;
    ///
    ///     let highest_monad = (end..start)
    ///         .rev()
    ///         .find(|&i| {
    ///             if i % 1_000_000 == 0 {
    ///                 println!("{}", i);
    ///             }
    ///             run_alu(&instrs, i as u64)
    ///         })
    ///         .expect("NO SOLUTION");
    ///
    ///     (highest_monad, 0)
    /// }
    /// ```
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
                    regs[dst] += regs[src];
                }

                Instr::Add(Operand::Reg(dst), Operand::Imm(src)) => {
                    regs[dst] += src;
                }

                Instr::Mul(Operand::Reg(dst), Operand::Reg(src)) => {
                    regs[dst] *= regs[src];
                }

                Instr::Mul(Operand::Reg(dst), Operand::Imm(src)) => {
                    regs[dst] *= src;
                }

                Instr::Div(Operand::Reg(dst), Operand::Reg(src)) => {
                    regs[dst] /= regs[src];
                }

                Instr::Div(Operand::Reg(dst), Operand::Imm(src)) => {
                    regs[dst] /= src;
                }

                Instr::Mod(Operand::Reg(dst), Operand::Reg(src)) => {
                    regs[dst] %= regs[src];
                }

                Instr::Mod(Operand::Reg(dst), Operand::Imm(src)) => {
                    regs[dst] %= src;
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
