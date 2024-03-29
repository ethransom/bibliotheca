#![feature(test)]

extern crate test;

const EXAMPLE: &str = include_str!("example09.txt");
const INPUT: &str = include_str!("input09.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (i32, i32) {
    let histories = parse(input).expect("couldn't parse input");

    histories
        .clone()
        .into_iter()
        .fold((0, 0), |(next, prev), history| {
            let mut derivatives = vec![history.clone()];

            loop {
                let derivative = derivatives
                    .last()
                    .unwrap()
                    .iter()
                    .as_slice()
                    .windows(2)
                    .map(|pair| pair[1] - pair[0])
                    .collect::<Vec<_>>();

                derivatives.push(derivative);

                if derivatives
                    .last()
                    .unwrap()
                    .iter()
                    .as_slice()
                    .windows(2)
                    .all(|w| w[0] == w[1])
                {
                    break;
                }
            }

            #[cfg(debug_assertions)]
            {
                for d in &derivatives {
                    println!("{:?}", d);
                }
                println!(
                    "solved {history:?} after {steps} derivations",
                    steps = derivatives.len()
                );
            }

            let new_next = derivatives
                .iter()
                .rev()
                .fold(0, |delta, prev| delta + prev.last().unwrap());

            let new_prev = derivatives
                .iter()
                .rev()
                .fold(0, |delta, prev| prev.first().unwrap() - delta);

            #[cfg(debug_assertions)]
            {
                println!("previous value was {new_prev}, next value is {new_next}");
                println!();
            }

            (next + new_next, prev + new_prev)
        })
}

fn parse(input: &str) -> Option<Vec<Vec<i32>>> {
    input
        .lines()
        .map(|line| line.split_whitespace().map(str::parse::<i32>).collect())
        .collect::<Result<_, _>>()
        .ok()
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (114, 2));
}

#[test]
fn test_input() {
    assert_eq!(solve(INPUT), (1939607039, 1041));
}

#[bench]
fn bench_solve_current(b: &mut test::Bencher) {
    b.iter(|| {
        test_input();
    });
}

#[bench]
fn bench_solve_without_original_clones(b: &mut test::Bencher) {
    fn parse(input: &str) -> Option<Vec<Vec<i32>>> {
        input
            .lines()
            .map(|line| line.split_whitespace().map(str::parse::<i32>).collect())
            .collect::<Result<_, _>>()
            .ok()
    }

    #[allow(dead_code)]
    fn solve(input: &str) -> (i32, i32) {
        let histories = parse(input).expect("couldn't parse input");

        histories.into_iter().fold((0, 0), |(next, prev), history| {
            let mut derivatives = vec![history];

            loop {
                let derivative = derivatives
                    .last()
                    .unwrap()
                    .iter()
                    .as_slice()
                    .windows(2)
                    .map(|pair| pair[1] - pair[0])
                    .collect::<Vec<_>>();

                let brk = derivative
                    .iter()
                    .as_slice()
                    .windows(2)
                    .all(|w| w[0] == w[1]);

                derivatives.push(derivative);

                if brk {
                    break;
                }
            }

            #[cfg(debug_assertions)]
            {
                for d in &derivatives {
                    println!("{:?}", d);
                }
                println!(
                    "solved {history:?} after {steps} derivations",
                    history = derivatives.first().unwrap(),
                    steps = derivatives.len()
                );
            }

            let new_next = derivatives
                .iter()
                .rev()
                .fold(0, |delta, prev| delta + prev.last().unwrap());

            let new_prev = derivatives
                .iter()
                .rev()
                .fold(0, |delta, prev| prev.first().unwrap() - delta);

            #[cfg(debug_assertions)]
            {
                println!("previous value was {new_prev}, next value is {new_next}");
                println!();
            }

            (next + new_next, prev + new_prev)
        })
    }

    b.iter(|| {
        assert_eq!(solve(INPUT), (1939607039, 1041));
    });
}

#[bench]
fn bench_solve_without_original_clones_and_lazy_parse(b: &mut test::Bencher) {
    #[allow(dead_code)]
    fn solve(input: &str) -> (i32, i32) {
        input
            .lines()
            .enumerate()
            .map(|(line_no, line)| {
                line.split_whitespace()
                    .map(str::parse::<i32>)
                    .collect::<Result<Vec<i32>, _>>()
                    .unwrap_or_else(|e| panic!("error on line {}: {}", line_no + 1, e))
            })
            .fold((0, 0), |(next, prev), history| {
                let mut derivatives = vec![history];

                loop {
                    let derivative = derivatives
                        .last()
                        .unwrap()
                        .iter()
                        .as_slice()
                        .windows(2)
                        .map(|pair| pair[1] - pair[0])
                        .collect::<Vec<_>>();

                    let brk = derivative
                        .iter()
                        .as_slice()
                        .windows(2)
                        .all(|w| w[0] == w[1]);

                    derivatives.push(derivative);

                    if brk {
                        break;
                    }
                }

                #[cfg(debug_assertions)]
                {
                    for d in &derivatives {
                        println!("{:?}", d);
                    }
                    println!(
                        "solved {history:?} after {steps} derivations",
                        history = derivatives.first().unwrap(),
                        steps = derivatives.len()
                    );
                }

                let new_next = derivatives
                    .iter()
                    .rev()
                    .fold(0, |delta, prev| delta + prev.last().unwrap());

                let new_prev = derivatives
                    .iter()
                    .rev()
                    .fold(0, |delta, prev| prev.first().unwrap() - delta);

                #[cfg(debug_assertions)]
                {
                    println!("previous value was {new_prev}, next value is {new_next}");
                    println!();
                }

                (next + new_next, prev + new_prev)
            })
    }

    b.iter(|| {
        assert_eq!(solve(INPUT), (1939607039, 1041));
    });
}

#[bench]
fn bench_solve_without_parse(b: &mut test::Bencher) {
    #[allow(dead_code)]
    fn solve(histories: &[Vec<i32>]) -> (i32, i32) {
        histories.iter().fold((0, 0), |(next, prev), history| {
            let mut derivatives = vec![history.clone()];

            loop {
                let derivative = derivatives
                    .last()
                    .unwrap()
                    .iter()
                    .as_slice()
                    .windows(2)
                    .map(|pair| pair[1] - pair[0])
                    .collect::<Vec<_>>();

                let brk = derivative
                    .iter()
                    .as_slice()
                    .windows(2)
                    .all(|w| w[0] == w[1]);

                derivatives.push(derivative);

                if brk {
                    break;
                }
            }

            #[cfg(debug_assertions)]
            {
                for d in &derivatives {
                    println!("{:?}", d);
                }
                println!(
                    "solved {history:?} after {steps} derivations",
                    history = derivatives.first().unwrap(),
                    steps = derivatives.len()
                );
            }

            let new_next = derivatives
                .iter()
                .rev()
                .fold(0, |delta, prev| delta + prev.last().unwrap());

            let new_prev = derivatives
                .iter()
                .rev()
                .fold(0, |delta, prev| prev.first().unwrap() - delta);

            #[cfg(debug_assertions)]
            {
                println!("previous value was {new_prev}, next value is {new_next}");
                println!();
            }

            (next + new_next, prev + new_prev)
        })
    }

    let histories = parse(INPUT).expect("couldn't parse input");

    b.iter(|| {
        assert_eq!(solve(&histories), (1939607039, 1041));
    });
}
