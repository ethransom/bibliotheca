#![feature(test)]

extern crate test;

const EXAMPLE: &str = include_str!("example21.txt");
const INPUT: &str = include_str!("input21.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (u32, u32) {
    let starts: [u32; 2] = input
        .lines()
        .map(|line| {
            let (_info, pos) = line.split_once(": ").expect("expected ':'");
            pos.parse::<u32>().expect("can't parse pos") - 1
        })
        .collect::<Vec<_>>()
        .try_into()
        .expect("not enough starting positions");

    let mut positions = starts;
    let mut scores = [0u32; 2];

    let mut rolls = 0;

    while scores.iter().filter(|&s| *s >= 1000).count() == 0 {
        dbg!(&scores, &positions);
        for (i, player) in positions.iter_mut().enumerate() {
            // print!("player {} rolls ", i + 1);
            let mut roll = 0;
            for _ in 0..3 {
                // TODO: make this iterator
                rolls += 1;
                roll += rolls % 100;
                // print!("{} + ", rolls % 100);
            }
            println!(" = {}", roll);

            *player = (*player + roll) % 10;

            // println!("player {} moves to {}", i + 1, *player + 1);

            scores[i] += *player + 1;

            if scores[i] >= 1000 {
                break;
            }
        }
    }

    let losing_score = scores.iter().min().unwrap();

    dbg!(losing_score, rolls);

    (losing_score * rolls, 0)
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (739785, 0));
}

#[bench]
fn bench_solve_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (926610, 0));
    });
}
