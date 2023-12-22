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

    'images: for image in &images {
        let height = image.len();
        let width = image[0].len();

        // println!("trying vertically");
        'vertical_mirror: for c in 1..width {
            for r in 0..height {
                let left = &image[r][0..c];
                let right = &image[r][c..];

                let mirreflect = left.chars().rev().zip(right.chars()).all(|(a, b)| a == b);
                // println!("{left:?} {right:?} {}", mirreflect);
                if !mirreflect {
                    continue 'vertical_mirror;
                }
            }

            // winner winner chicken dinner
            // println!(
            //     "did reflect top to bottom around line between {} {}",
            //     (c - 1) + 1,
            //     c + 1
            // );

            sum += (c - 1) + 1;

            continue 'images;
        }

        // println!("\ntrying horizontally");

        'horizontal_mirror: for r in 1..height {
            for c in 0..width {
                let top = (0..r).map(|r| image[r].as_bytes()[c]);
                let bottom = (r..image.len()).map(|r| image[r].as_bytes()[c]);

                // print!(
                //     "{:?} {:?}",
                //     top.clone().map(|c| c as char).collect::<String>(),
                //     bottom.clone().map(|c| c as char).collect::<String>()
                // );
                let mirreflect = top.rev().zip(bottom).all(|(a, b)| a == b);
                // println!("{mirreflect}");
                if !mirreflect {
                    continue 'horizontal_mirror;
                }
            }

            // winner winner chicken dinner
            // println!(
            //     "did reflect left to right around line between {} {}",
            //     (r - 1) + 1,
            //     r + 1
            // );

            sum += r * 100;

            continue 'images;
        }
    }

    (sum, 0)
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
