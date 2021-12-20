#![feature(test)]

use std::collections::HashSet;

extern crate test;

const EXAMPLE: &str = include_str!("example20.txt");
const INPUT: &str = include_str!("input20.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    let (algorithm, image) = input
        .split_once("\n\n")
        .expect("expected both algorithm and image");

    let algorithm = algorithm
        .chars()
        .map(|c| match c {
            '.' => false,
            '#' => true,
            _ => panic!("unexpected char"),
        })
        .collect::<Vec<bool>>();

    let image: HashSet<(i32, i32)> = image
        .lines()
        .enumerate()
        .flat_map(|(r, line)| {
            line.chars()
                .enumerate()
                .flat_map(move |(c, ch)| if ch == '#' { Some((c as i32, r as i32)) } else { None })
        })
        .collect();

    let width = image.iter().map(|(c, _)| c + 1).max().expect("can't solve empty image");
    let height = image.iter().map(|(_, r)| r + 1).max().expect("can't solve empty image");

    let neighbors = neighbors();

    let mut image = image;
    let (mut start_x, mut end_x, mut start_y, mut end_y) = (-1, width, -1, height);

    for _step in 0..2 {
        println!("wide: {}, high: {}", end_x - start_x, end_y - start_y);

        let mut next_image: HashSet<(i32, i32)> = HashSet::new();

        for c in start_x..=end_x {
            for r in start_y..=end_y {
                let mut bin: usize = 0;

                for (i, j) in &neighbors {
                    bin <<= 1;
                    if image.contains(&((c + j), (r + i))) {
                        bin += 1;
                    }
                }

                print!("{}", if algorithm[bin] { '#' } else { '.' });

               if algorithm[bin] {
                   next_image.insert((c, r));
               }

            }
            println!("");
        }

        image = next_image;
        start_x -= 1;
        end_x += 1;
        start_y -= 1;
        end_y +=1;
    }

    (image.len(), 0)
}

fn neighbors() -> Vec<(i32, i32)> {
    let mut ret = vec![];

    for i in -1..=1 {
        for j in -1..=1 {
            ret.push((i, j));
        }
    }

    ret
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (35, 0));
}

#[bench]
fn bench_solve_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (0, 0));
    });
}
