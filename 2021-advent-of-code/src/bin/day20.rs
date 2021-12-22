#![feature(test)]

use std::collections::HashSet;

extern crate test;

const EXAMPLE: &str = include_str!("example20.txt");
const INPUT: &str = include_str!("input20.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

#[derive(Default, Clone)]
struct Image {
    background: bool,
    pixels: HashSet<(i32, i32)>,
}

impl Image {
    fn new(background: bool) -> Self {
        Image {
            background,
            pixels: HashSet::<(i32, i32)>::default(),
        }
    }

    fn set(&mut self, pos: (i32, i32), val: bool) {
        if val != self.background {
            self.pixels.insert(pos);
        }
    }

    fn get(&self, pos: &(i32, i32)) -> bool {
        if self.background {
            !self.pixels.contains(pos)
        } else {
            self.pixels.contains(pos)
        }
    }

    fn lit(&self) -> usize {
        self.pixels.len()
    }
}

fn solve(input: &str) -> (usize, usize) {
    let (algorithm, pixels) = input
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

    let mut image = Image::new(false);

    pixels.lines().enumerate().for_each(|(r, line)| {
        line.chars().enumerate().for_each(|(c, ch)| {
            if ch == '#' {
                image.set((c as i32, r as i32), true)
            }
        })
    });

    let width = image
        .pixels
        .iter()
        .map(|(c, _)| c + 1)
        .max()
        .expect("can't solve empty image");
    let height = image
        .pixels
        .iter()
        .map(|(_, r)| r + 1)
        .max()
        .expect("can't solve empty image");

    let twice_enhanced = enhance(&image, width, height, &algorithm, 2);

    let fifty_enhanced = enhance(&image, width, height, &algorithm, 50);

    (twice_enhanced.lit(), fifty_enhanced.lit())
}

fn enhance(image: &Image, width: i32, height: i32, algorithm: &[bool], steps: usize) -> Image {
    let neighbors = neighbors();

    let mut image: Image = image.clone();

    let (mut start_x, mut end_x, mut start_y, mut end_y) = (-1, width + 1, -1, height + 1);

    for _step in 0..steps {
        let next_background = if algorithm[0] {
            !image.background
        } else {
            false
        };

        let mut next_image = Image::new(next_background);

        for c in start_x..=end_x {
            for r in start_y..=end_y {
                let mut bin: usize = 0;

                for (i, j) in &neighbors {
                    bin <<= 1;
                    if image.get(&((c + j), (r + i))) {
                        bin += 1;
                    }
                }

                print!("{}", if algorithm[bin] { '#' } else { '.' });

                next_image.set((c, r), algorithm[bin]);
            }
            println!();
        }

        image = next_image;
        start_x -= 1;
        end_x += 1;
        start_y -= 1;
        end_y += 1;
    }

    image
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
    assert_eq!(solve(EXAMPLE), (35, 3351));
}

#[bench]
fn bench_solve_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (5359, 12333));
    });
}
