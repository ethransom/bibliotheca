// #![feature(test)]

// extern crate test;

const EXAMPLE: &str = include_str!("example13.txt");
const INPUT: &str = include_str!("input13.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    let images = parse(input);

    let mut sum = 0;

    for image in &images {
        let height = image.len();
        let width = image[0].len();

        // println!("trying vertically");
        if let Some(c) = reflects_vertically(image, height, width) {
            sum += c;
            continue;
        }

        // println!("\ntrying horizontally");

        if let Some(r) = reflects_horizontally(image, height, width) {
            sum += r * 100;
            continue;
        }
    }

    (sum, 0)
}

fn reflects_horizontally(image: &[&str], height: usize, width: usize) -> Option<usize> {
    'reflection: for r in 1..height {
        for c in 0..width {
            let top = (0..r).map(|r| image[r].as_bytes()[c]);
            let bottom = (r..image.len()).map(|r| image[r].as_bytes()[c]);

            // print!(
            //     "{:?} {:?}",
            //     top.clone().map(|c| c as char).collect::<String>(),
            //     bottom.clone().map(|c| c as char).collect::<String>()
            // );
            let reflects = top.rev().zip(bottom).all(|(a, b)| a == b);
            // println!("{mirreflect}");
            if !reflects {
                continue 'reflection;
            }
        }

        // winner winner chicken dinner
        // println!(
        //     "did reflect left to right around line between {} {}",
        //     (r - 1) + 1,
        //     r + 1
        // );

        return Some(r);
    }

    None
}

fn reflects_vertically(image: &[&str], height: usize, width: usize) -> Option<usize> {
    'reflection: for c in 1..width {
        for r in 0..height {
            let left = &image[r][0..c];
            let right = &image[r][c..];

            let reflects = left.chars().rev().zip(right.chars()).all(|(a, b)| a == b);
            // println!("{left:?} {right:?} {}", mirreflect);
            if !reflects {
                continue 'reflection;
            }
        }

        // winner winner chicken dinner
        // println!(
        //     "did reflect top to bottom around line between {} {}",
        //     (c - 1) + 1,
        //     c + 1
        // );

        return Some(c);
    }

    None
}

fn parse(input: &str) -> Vec<Vec<&str>> {
    input
        .split("\n\n")
        .map(|image| image.lines().collect())
        .collect()
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (405, 0));
}

#[test]
fn test_input() {
    assert_eq!(solve(INPUT), (35232, 0));
}

// #[bench]
// fn bench_solve_current(b: &mut test::Bencher) {
//     b.iter(|| {
//         assert_eq!(solve(INPUT), (0, 0));
//     });
// }
