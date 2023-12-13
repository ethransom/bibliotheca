// #![feature(test)]

// extern crate test;

const EXAMPLE: &str = include_str!("example09.txt");
const INPUT: &str = include_str!("input09.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (i32, i32) {
    let histories = parse(input).expect("couldn't parse input");

    let sum_next_values = histories
        .clone()
        .into_iter()
        .map(|history| {
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

            for d in &derivatives {
                println!("{:?}", d);
            }
            println!(
                "solved {history:?} after {steps} derivations\n",
                steps = derivatives.len()
            );

            // hallucinate last item
            // for part 2 I assume we will need to store these off somewhere 🙈
            derivatives
                .iter()
                .rev()
                .fold(0, |delta, prev| delta + prev.last().unwrap())

            // let iter = derivatives.iter_mut().rev().last().unwrap();
            // let c = *iter.last().unwrap();
            // iter.push(c);
            //
            // for d in derivatives.iter_mut().windows(2) {
            //     let prev = d.last().unwrap();
            // }
        })
        .sum();

    (sum_next_values, 0)
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
    assert_eq!(solve(EXAMPLE), (114, 0));
}

#[test]
fn test_input() {
    assert_eq!(solve(INPUT), (1939607039, 0));
}

// #[bench]
// fn bench_solve_current(b: &mut test::Bencher) {
//     b.iter(|| {
//         assert_eq!(solve(INPUT), (0, 0));
//     });
// }
