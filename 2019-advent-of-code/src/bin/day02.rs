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

    let (found_noun, found_verb) = (0..100)
        .find_map(|noun| {
            (0..100).find_map(|verb| {
                if 19690720 == run(&intcode, noun, verb)[0] {
                    Some((noun, verb))
                } else {
                    None
                }
            })
        })
        .expect("could not find inputs resulting in solution");

    (after_example_inputs[0], 100 * found_noun + found_verb)
}

fn parse(input: &str) -> Vec<u64> {
    input
        .split(',')
        .map(str::parse::<u64>)
        .collect::<Result<Vec<u64>, std::num::ParseIntError>>()
        .expect("couldn't parse input file")
}

fn run(intcode: &[u64], input1: u64, input2: u64) -> Vec<u64> {
    let mut memory = intcode.to_owned();

    memory[1] = input1;
    memory[2] = input2;

    let mut ip = 0;

    while let Some(instr) = memory.get(ip..(ip + 4)) {
        let [opcode, input1, input2, output]: [u64; 4] = instr.try_into().unwrap();

        let op = match opcode {
            1 => u64::checked_add,
            2 => u64::checked_mul,
            _ => break,
        };

        memory[output as usize] =
            op(memory[input1 as usize], memory[input2 as usize]).expect("arithmetic overflow");

        ip += 4;
    }

    memory
}

#[test]
fn test_run() {
    fn test(intcode: &[u64], end_state: &[u64]) {
        assert_eq!(run(&intcode, intcode[1], intcode[2]), end_state);
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

    assert_eq!(run(&intcode, intcode[1], intcode[2])[0], 3_500);
}

#[bench]
fn bench_solve_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (5_290_681, 5_741));
    });
}
