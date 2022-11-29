#![feature(test)]

extern crate test;

const EXAMPLE: &str = include_str!("example02.txt");
const INPUT: &str = include_str!("input02.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (u64, u64) {
    let intcode = parse(input);

    let after_example_inputs = run(&intcode, 12, 2);

    (after_example_inputs[0], 0)
}

fn parse(input: &str) -> Vec<u64> {
    input
        .split(',')
        .map(str::parse::<u64>)
        .collect::<Result<Vec<u64>, std::num::ParseIntError>>()
        .expect("couldn't parse input file")
}

fn run(intcode: &[u64], input1: u64, input2: u64) -> Vec<u64> {
    let mut intcode = intcode.to_owned();

    intcode[1] = input1;
    intcode[2] = input2;

    let mut ip = 0;

    while let Some(instr) = intcode.get(ip..(ip + 4)) {
        let [opcode, input1, input2, output]: [u64; 4] = instr.try_into().unwrap();

        let op = match opcode {
            1 => u64::checked_add,
            2 => u64::checked_mul,
            _ => break,
        };

        intcode[output as usize] =
            op(intcode[input1 as usize], intcode[input2 as usize]).expect("arithmetic overflow");

        ip += 4;
    }

    intcode.to_owned()
}

#[test]
fn test_run() {
    fn test(initial_state: &[u64], end_state: &[u64]) {
        let state = initial_state.to_owned();
        let (input1, input2) = (state[1], state[2]);
        let state = run(&state, input1, input2);
        assert_eq!(state, end_state);
    }
    test(
        &parse(EXAMPLE),
        &[3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
    );
    test(&[1, 0, 0, 0, 99], &[2, 0, 0, 0, 99]);
    test(&[2, 3, 0, 3, 99], &[2, 3, 0, 6, 99]);
    test(&[2, 4, 4, 5, 99, 0], &[2, 4, 4, 5, 99, 9801]);
    test(
        &[1, 1, 1, 4, 99, 5, 6, 0, 99],
        &[30, 1, 1, 4, 2, 5, 6, 0, 99],
    );
}

#[test]
fn test_example() {
    let intcode = parse(EXAMPLE);

    let (input1, input2) = (intcode[1], intcode[2]);

    assert_eq!(run(&intcode, input1, input2)[0], 3_500);
}

#[bench]
fn bench_solve_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (5_290_681, 0));
    });
}
