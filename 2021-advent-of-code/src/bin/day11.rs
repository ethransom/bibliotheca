#![feature(test)]

extern crate test;

use std::collections::{HashSet, VecDeque};

const EXAMPLE: &str = include_str!("example11.txt");
const INPUT: &str = include_str!("input11.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    let mut grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("not a number"))
                .collect()
        })
        .collect::<Vec<Vec<u32>>>();

    let octopuses_len = grid.iter().map(|row| row.len()).sum::<usize>();

    let flashed: Vec<usize> = (0..1000).map(|_s| step(&mut grid)).collect();

    let first_100_flashes = flashed.iter().take(100).sum();

    let first_flash = 1 + flashed
        .into_iter()
        .position(|f| f == octopuses_len)
        .expect("did not synchronize");

    (first_100_flashes, first_flash)
}

fn step(grid: &mut Vec<Vec<u32>>) -> usize {
    let mut to_flash = VecDeque::<(usize, usize)>::new();
    // take my energy!
    for (r, row) in grid.iter_mut().enumerate() {
        for (c, cell) in row.iter_mut().enumerate() {
            *cell += 1;
            if *cell > 9 {
                to_flash.push_back((c, r));
            }
        }
    }

    let mut flashed = HashSet::<(usize, usize)>::new();
    while let Some((c, r)) = to_flash.pop_front() {
        if flashed.contains(&(c, r)) {
            continue;
        }
        flashed.insert((c, r));

        for rr in -1..=1 {
            for cc in -1..=1 {
                if rr == 0 && cc == 0 {
                    continue;
                }

                let r = (r as i64 + rr) as usize;
                let c = (c as i64 + cc) as usize;

                if let Some(row) = grid.get_mut(r) {
                    if let Some(val) = row.get_mut(c) {
                        *val += 1;
                        if *val > 9 {
                            to_flash.push_back((c, r));
                        }
                    }
                }
            }
        }
    }

    for &(c, r) in &flashed {
        if grid[r][c] > 9 {
            grid[r][c] = 0;
        }
    }

    flashed.len()
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (1656, 195));
}

#[bench]
fn bench_solve_00_original(b: &mut test::Bencher) {
    fn solve(input: &str) -> (usize, usize) {
        let mut grid = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).expect("not a number"))
                    .collect()
            })
            .collect::<Vec<Vec<u32>>>();

        let octopuses_len = grid.iter().map(|row| row.len()).sum::<usize>();

        let flashed: Vec<usize> = (0..1000).map(|_s| step(&mut grid)).collect();

        let first_100_flashes = flashed.iter().take(100).sum();

        let first_flash = 1 + flashed
            .into_iter()
            .position(|f| f == octopuses_len)
            .expect("did not synchronize");

        (first_100_flashes, first_flash)
    }

    fn step(grid: &mut Vec<Vec<u32>>) -> usize {
        let mut to_flash = VecDeque::<(usize, usize)>::new();
        // take my energy!
        for r in 0..grid.len() {
            for c in 0..grid[r].len() {
                grid[r][c] += 1;
                if grid[r][c] > 9 {
                    to_flash.push_back((c, r));
                }
            }
        }

        let mut flashed = HashSet::<(usize, usize)>::new();
        while let Some((c, r)) = to_flash.pop_front() {
            if flashed.contains(&(c, r)) {
                continue;
            }
            flashed.insert((c, r));

            for rr in -1..=1 {
                for cc in -1..=1 {
                    if rr == 0 && cc == 0 {
                        continue;
                    }

                    let r = (r as i64 + rr) as usize;
                    let c = (c as i64 + cc) as usize;

                    if let Some(row) = grid.get_mut(r) {
                        if let Some(val) = row.get_mut(c) {
                            *val += 1;
                            if *val > 9 {
                                to_flash.push_back((c, r));
                            }
                        }
                    }
                }
            }
        }

        // could also loop through `flashed` for this
        for r in 0..grid.len() {
            for c in 0..grid[r].len() {
                if grid[r][c] > 9 {
                    grid[r][c] = 0;
                }
            }
        }

        flashed.len()
    }

    b.iter(|| {
        assert_eq!(solve(INPUT), (1591, 314));
    });
}

#[bench]
fn bench_solve_01_no_range_loops(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (1591, 314));
    });
}

#[bench]
fn bench_solve_02_with_capacity(b: &mut test::Bencher) {
    fn solve(input: &str) -> (usize, usize) {
        let mut grid = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).expect("not a number"))
                    .collect()
            })
            .collect::<Vec<Vec<u32>>>();

        let octopuses_len = grid.iter().map(|row| row.len()).sum::<usize>();

        let flashed: Vec<usize> = (0..1000).map(|_s| step(&mut grid)).collect();

        let first_100_flashes = flashed.iter().take(100).sum();

        let first_flash = 1 + flashed
            .into_iter()
            .position(|f| f == octopuses_len)
            .expect("did not synchronize");

        (first_100_flashes, first_flash)
    }

    fn step(grid: &mut Vec<Vec<u32>>) -> usize {
        let mut to_flash = VecDeque::<(usize, usize)>::with_capacity(100);
        // take my energy!
        for r in 0..grid.len() {
            for c in 0..grid[r].len() {
                grid[r][c] += 1;
                if grid[r][c] > 9 {
                    to_flash.push_back((c, r));
                }
            }
        }

        let mut flashed = HashSet::<(usize, usize)>::with_capacity(100);
        while let Some((c, r)) = to_flash.pop_front() {
            if flashed.contains(&(c, r)) {
                continue;
            }
            flashed.insert((c, r));

            for rr in -1..=1 {
                for cc in -1..=1 {
                    if rr == 0 && cc == 0 {
                        continue;
                    }

                    let r = (r as i64 + rr) as usize;
                    let c = (c as i64 + cc) as usize;

                    if let Some(row) = grid.get_mut(r) {
                        if let Some(val) = row.get_mut(c) {
                            *val += 1;
                            if *val > 9 {
                                to_flash.push_back((c, r));
                            }
                        }
                    }
                }
            }
        }

        // could also loop through `flashed` for this
        for r in 0..grid.len() {
            for c in 0..grid[r].len() {
                if grid[r][c] > 9 {
                    grid[r][c] = 0;
                }
            }
        }

        flashed.len()
    }

    b.iter(|| {
        assert_eq!(solve(INPUT), (1591, 314));
    });
}

#[bench]
fn bench_solve_03_reuse_collections(b: &mut test::Bencher) {
    fn solve(input: &str) -> (usize, usize) {
        let mut grid = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).expect("not a number"))
                    .collect()
            })
            .collect::<Vec<Vec<u32>>>();

        let octopuses_len = grid.iter().map(|row| row.len()).sum::<usize>();

        let flashed: Vec<usize> = step(&mut grid, 1000);

        let first_100_flashes = flashed.iter().take(100).sum();

        let first_flash = 1 + flashed
            .into_iter()
            .position(|f| f == octopuses_len)
            .expect("did not synchronize");

        (first_100_flashes, first_flash)
    }

    fn step(grid: &mut Vec<Vec<u32>>, steps: usize) -> Vec<usize> {
        let mut flash_counts = Vec::with_capacity(steps);

        let mut to_flash = VecDeque::<(usize, usize)>::with_capacity(100);
        let mut flashed = HashSet::<(usize, usize)>::with_capacity(100);

        for _s in 0..steps {
            // take my energy!
            for r in 0..grid.len() {
                for c in 0..grid[r].len() {
                    grid[r][c] += 1;
                    if grid[r][c] > 9 {
                        to_flash.push_back((c, r));
                    }
                }
            }

            while let Some((c, r)) = to_flash.pop_front() {
                if flashed.contains(&(c, r)) {
                    continue;
                }
                flashed.insert((c, r));

                for rr in -1..=1 {
                    for cc in -1..=1 {
                        if rr == 0 && cc == 0 {
                            continue;
                        }

                        let r = (r as i64 + rr) as usize;
                        let c = (c as i64 + cc) as usize;

                        if let Some(row) = grid.get_mut(r) {
                            if let Some(val) = row.get_mut(c) {
                                *val += 1;
                                if *val > 9 {
                                    to_flash.push_back((c, r));
                                }
                            }
                        }
                    }
                }
            }

            // could also loop through `flashed` for this
            for r in 0..grid.len() {
                for c in 0..grid[r].len() {
                    if grid[r][c] > 9 {
                        grid[r][c] = 0;
                    }
                }
            }

            flash_counts.push(flashed.len());
            to_flash.clear();
            flashed.clear();
        }
        flash_counts
    }

    b.iter(|| {
        assert_eq!(solve(INPUT), (1591, 314));
    });
}

#[bench]
fn bench_solve_04_better_hashfn(b: &mut test::Bencher) {
    use fxhash::FxHashSet;

    fn solve(input: &str) -> (usize, usize) {
        let mut grid = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).expect("not a number"))
                    .collect()
            })
            .collect::<Vec<Vec<u32>>>();

        let octopuses_len = grid.iter().map(|row| row.len()).sum::<usize>();

        let flashed: Vec<usize> = step(&mut grid, 1000);

        let first_100_flashes = flashed.iter().take(100).sum();

        let first_flash = 1 + flashed
            .into_iter()
            .position(|f| f == octopuses_len)
            .expect("did not synchronize");

        (first_100_flashes, first_flash)
    }

    fn step(grid: &mut Vec<Vec<u32>>, steps: usize) -> Vec<usize> {
        let mut flash_counts = Vec::with_capacity(steps);

        let mut to_flash = VecDeque::<(usize, usize)>::with_capacity(100);
        let mut flashed = FxHashSet::<(usize, usize)>::default();
        flashed.reserve(100);

        for _s in 0..steps {
            // take my energy!
            for r in 0..grid.len() {
                for c in 0..grid[r].len() {
                    grid[r][c] += 1;
                    if grid[r][c] > 9 {
                        to_flash.push_back((c, r));
                    }
                }
            }

            while let Some((c, r)) = to_flash.pop_front() {
                if flashed.contains(&(c, r)) {
                    continue;
                }
                flashed.insert((c, r));

                for rr in -1..=1 {
                    for cc in -1..=1 {
                        if rr == 0 && cc == 0 {
                            continue;
                        }

                        let r = (r as i64 + rr) as usize;
                        let c = (c as i64 + cc) as usize;

                        if let Some(row) = grid.get_mut(r) {
                            if let Some(val) = row.get_mut(c) {
                                *val += 1;
                                if *val > 9 {
                                    to_flash.push_back((c, r));
                                }
                            }
                        }
                    }
                }
            }

            // could also loop through `flashed` for this
            for r in 0..grid.len() {
                for c in 0..grid[r].len() {
                    if grid[r][c] > 9 {
                        grid[r][c] = 0;
                    }
                }
            }

            flash_counts.push(flashed.len());
            to_flash.clear();
            flashed.clear();
        }
        flash_counts
    }

    b.iter(|| {
        assert_eq!(solve(INPUT), (1591, 314));
    });
}

#[bench]
fn bench_solve_05_stdhash_short_circuit(b: &mut test::Bencher) {
    fn solve(input: &str) -> (usize, usize) {
        let mut grid = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).expect("not a number"))
                    .collect()
            })
            .collect::<Vec<Vec<u32>>>();

        let octopuses_len = grid.iter().map(|row| row.len()).sum::<usize>();

        let flashed: Vec<usize> = step(&mut grid, 1000);

        let first_100_flashes = flashed.iter().take(100).sum();

        let first_flash = 1 + flashed
            .into_iter()
            .position(|f| f == octopuses_len)
            .expect("did not synchronize");

        (first_100_flashes, first_flash)
    }

    fn step(grid: &mut Vec<Vec<u32>>, steps: usize) -> Vec<usize> {
        let octopuses_len = grid.iter().map(|row| row.len()).sum::<usize>();

        let mut flash_counts = Vec::with_capacity(steps);

        let mut to_flash = VecDeque::<(usize, usize)>::with_capacity(100);
        let mut flashed = HashSet::<(usize, usize)>::with_capacity(100);

        for _s in 0..steps {
            // take my energy!
            for r in 0..grid.len() {
                for c in 0..grid[r].len() {
                    grid[r][c] += 1;
                    if grid[r][c] > 9 {
                        to_flash.push_back((c, r));
                    }
                }
            }

            while let Some((c, r)) = to_flash.pop_front() {
                if flashed.contains(&(c, r)) {
                    continue;
                }
                flashed.insert((c, r));

                for rr in -1..=1 {
                    for cc in -1..=1 {
                        if rr == 0 && cc == 0 {
                            continue;
                        }

                        let r = (r as i64 + rr) as usize;
                        let c = (c as i64 + cc) as usize;

                        if let Some(row) = grid.get_mut(r) {
                            if let Some(val) = row.get_mut(c) {
                                *val += 1;
                                if *val > 9 {
                                    to_flash.push_back((c, r));
                                }
                            }
                        }
                    }
                }
            }

            // could also loop through `flashed` for this
            for r in 0..grid.len() {
                for c in 0..grid[r].len() {
                    if grid[r][c] > 9 {
                        grid[r][c] = 0;
                    }
                }
            }

            flash_counts.push(flashed.len());

            if flashed.len() == octopuses_len {
                break;
            }

            to_flash.clear();
            flashed.clear();
        }
        flash_counts
    }

    b.iter(|| {
        assert_eq!(solve(INPUT), (1591, 314));
    });
}

#[bench]
fn bench_solve_06_no_memoize_shortcircuit(b: &mut test::Bencher) {
    fn solve(input: &str) -> (usize, usize) {
        let mut grid = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).expect("not a number"))
                    .collect()
            })
            .collect::<Vec<Vec<u32>>>();

        step(&mut grid)
    }

    fn step(grid: &mut Vec<Vec<u32>>) -> (usize, usize) {
        let octopuses_len = grid.iter().map(|row| row.len()).sum::<usize>();

        let mut first_100_flashes = 0;

        let mut to_flash = VecDeque::<(usize, usize)>::with_capacity(100);
        let mut flashed = HashSet::<(usize, usize)>::with_capacity(100);

        for step in 0.. {
            // take my energy!
            for r in 0..grid.len() {
                for c in 0..grid[r].len() {
                    grid[r][c] += 1;
                    if grid[r][c] > 9 {
                        to_flash.push_back((c, r));
                    }
                }
            }

            while let Some((c, r)) = to_flash.pop_front() {
                if flashed.contains(&(c, r)) {
                    continue;
                }
                flashed.insert((c, r));

                for rr in -1..=1 {
                    for cc in -1..=1 {
                        if rr == 0 && cc == 0 {
                            continue;
                        }

                        let r = (r as i64 + rr) as usize;
                        let c = (c as i64 + cc) as usize;

                        if let Some(row) = grid.get_mut(r) {
                            if let Some(val) = row.get_mut(c) {
                                *val += 1;
                                if *val > 9 {
                                    to_flash.push_back((c, r));
                                }
                            }
                        }
                    }
                }
            }

            // could also loop through `flashed` for this
            for r in 0..grid.len() {
                for c in 0..grid[r].len() {
                    if grid[r][c] > 9 {
                        grid[r][c] = 0;
                    }
                }
            }

            if step < 100 {
                first_100_flashes += flashed.len();
            }

            if flashed.len() == octopuses_len {
                return (first_100_flashes, step + 1);
            }

            to_flash.clear();
            flashed.clear();
        }

        unreachable!();
    }

    b.iter(|| {
        assert_eq!(solve(INPUT), (1591, 314));
    });
}

#[bench]
fn bench_solve_07_better_hashfn_shortcircuit(b: &mut test::Bencher) {
    use fxhash::FxHashSet;

    fn solve(input: &str) -> (usize, usize) {
        let mut grid = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).expect("not a number"))
                    .collect()
            })
            .collect::<Vec<Vec<u32>>>();

        step(&mut grid)
    }

    fn step(grid: &mut Vec<Vec<u32>>) -> (usize, usize) {
        let octopuses_len = grid.iter().map(|row| row.len()).sum::<usize>();

        let mut first_100_flashes = 0;

        let mut to_flash = VecDeque::<(usize, usize)>::with_capacity(100);
        let mut flashed = FxHashSet::<(usize, usize)>::default();
        flashed.reserve(100);

        for step in 0.. {
            // take my energy!
            for r in 0..grid.len() {
                for c in 0..grid[r].len() {
                    grid[r][c] += 1;
                    if grid[r][c] > 9 {
                        to_flash.push_back((c, r));
                    }
                }
            }

            while let Some((c, r)) = to_flash.pop_front() {
                if flashed.contains(&(c, r)) {
                    continue;
                }
                flashed.insert((c, r));

                for rr in -1..=1 {
                    for cc in -1..=1 {
                        if rr == 0 && cc == 0 {
                            continue;
                        }

                        let r = (r as i64 + rr) as usize;
                        let c = (c as i64 + cc) as usize;

                        if let Some(row) = grid.get_mut(r) {
                            if let Some(val) = row.get_mut(c) {
                                *val += 1;
                                if *val > 9 {
                                    to_flash.push_back((c, r));
                                }
                            }
                        }
                    }
                }
            }

            // could also loop through `flashed` for this
            for r in 0..grid.len() {
                for c in 0..grid[r].len() {
                    if grid[r][c] > 9 {
                        grid[r][c] = 0;
                    }
                }
            }

            if step < 100 {
                first_100_flashes += flashed.len();
            }

            if flashed.len() == octopuses_len {
                return (first_100_flashes, step + 1);
            }

            to_flash.clear();
            flashed.clear();
        }

        unreachable!();
    }

    b.iter(|| {
        assert_eq!(solve(INPUT), (1591, 314));
    });
}

#[bench]
fn bench_solve_08_visited_table_shortcircuit(b: &mut test::Bencher) {
    fn solve(input: &str) -> (usize, usize) {
        let mut grid = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).expect("not a number"))
                    .collect()
            })
            .collect::<Vec<Vec<u32>>>();

        let octopuses_len = grid.iter().map(|row| row.len()).sum::<usize>();

        let flashed: Vec<usize> = step(&mut grid, 1000);

        let first_100_flashes = flashed.iter().take(100).sum();

        let first_flash = 1 + flashed
            .into_iter()
            .position(|f| f == octopuses_len)
            .expect("did not synchronize");

        (first_100_flashes, first_flash)
    }

    fn step(grid: &mut Vec<Vec<u32>>, steps: usize) -> Vec<usize> {
        let grid_size = grid.iter().map(|row| row.len()).sum::<usize>();
        let grid_rows = grid[0].len();

        let mut flash_counts = Vec::with_capacity(steps);

        let mut to_flash = VecDeque::<(usize, usize)>::with_capacity(100);
        let mut flashed = vec![-1_isize; grid_size];

        for step in 0..steps {
            // take my energy!
            for r in 0..grid.len() {
                for c in 0..grid[r].len() {
                    grid[r][c] += 1;
                    if grid[r][c] > 9 {
                        to_flash.push_back((c, r));
                    }
                }
            }

            while let Some((c, r)) = to_flash.pop_front() {
                if flashed[r * grid_rows + c] == step as isize {
                    continue;
                }
                flashed[r * grid_rows + c] = step as isize;

                for rr in -1..=1 {
                    for cc in -1..=1 {
                        if rr == 0 && cc == 0 {
                            continue;
                        }

                        let r = (r as i64 + rr) as usize;
                        let c = (c as i64 + cc) as usize;

                        if let Some(row) = grid.get_mut(r) {
                            if let Some(val) = row.get_mut(c) {
                                *val += 1;
                                if *val > 9 {
                                    to_flash.push_back((c, r));
                                }
                            }
                        }
                    }
                }
            }

            let mut flash_count = 0;
            for r in 0..grid.len() {
                for c in 0..grid[r].len() {
                    if grid[r][c] > 9 {
                        grid[r][c] = 0;
                        flash_count += 1;
                    }
                }
            }

            flash_counts.push(flash_count);

            if flash_count == grid_size {
                break;
            }

            to_flash.clear();
        }
        flash_counts
    }

    b.iter(|| {
        assert_eq!(solve(INPUT), (1591, 314));
    });
}

#[bench]
fn bench_solve_09_vistd_tbl_shrtcrct_n_rng_lps(b: &mut test::Bencher) {
    fn solve(input: &str) -> (usize, usize) {
        let mut grid = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).expect("not a number"))
                    .collect()
            })
            .collect::<Vec<Vec<u32>>>();

        let octopuses_len = grid.iter().map(|row| row.len()).sum::<usize>();

        let flashed: Vec<usize> = step(&mut grid, 1000);

        let first_100_flashes = flashed.iter().take(100).sum();

        let first_flash = 1 + flashed
            .into_iter()
            .position(|f| f == octopuses_len)
            .expect("did not synchronize");

        (first_100_flashes, first_flash)
    }

    fn step(grid: &mut Vec<Vec<u32>>, steps: usize) -> Vec<usize> {
        let grid_size = grid.iter().map(|row| row.len()).sum::<usize>();
        let grid_rows = grid[0].len();

        let mut flash_counts = Vec::with_capacity(steps);

        let mut to_flash = VecDeque::<(usize, usize)>::with_capacity(100);
        let mut flashed = vec![-1_isize; grid_size];

        for step in 0..steps {
            // take my energy!
            for (r, row) in grid.iter_mut().enumerate() {
                for (c, cell) in row.iter_mut().enumerate() {
                    *cell += 1;
                    if *cell > 9 {
                        to_flash.push_back((c, r));
                    }
                }
            }

            while let Some((c, r)) = to_flash.pop_front() {
                if flashed[r * grid_rows + c] == step as isize {
                    continue;
                }
                flashed[r * grid_rows + c] = step as isize;

                for rr in -1..=1 {
                    for cc in -1..=1 {
                        if rr == 0 && cc == 0 {
                            continue;
                        }

                        let r = (r as i64 + rr) as usize;
                        let c = (c as i64 + cc) as usize;

                        if let Some(row) = grid.get_mut(r) {
                            if let Some(val) = row.get_mut(c) {
                                *val += 1;
                                if *val > 9 {
                                    to_flash.push_back((c, r));
                                }
                            }
                        }
                    }
                }
            }

            let mut flash_count = 0;
            for row in grid.iter_mut() {
                for cell in row.iter_mut() {
                    if *cell > 9 {
                        *cell = 0;
                        flash_count += 1;
                    }
                }
            }

            flash_counts.push(flash_count);

            if flash_count == grid_size {
                break;
            }

            to_flash.clear();
        }
        flash_counts
    }

    b.iter(|| {
        assert_eq!(solve(INPUT), (1591, 314));
    });
}

#[bench]
fn bench_solve_10_vistd_tbl_shrtcrct_n_rng_lps_n_mmze(b: &mut test::Bencher) {
    fn solve(input: &str) -> (usize, usize) {
        let mut grid = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).expect("not a number"))
                    .collect()
            })
            .collect::<Vec<Vec<u32>>>();

        step(&mut grid)
    }

    fn step(grid: &mut Vec<Vec<u32>>) -> (usize, usize) {
        let grid_size = grid.iter().map(|row| row.len()).sum::<usize>();
        let grid_rows = grid[0].len();

        let mut first_100_flashes = 0;

        let mut to_flash = VecDeque::<(usize, usize)>::with_capacity(100);
        let mut flashed = vec![-1_isize; grid_size];

        for step in 0.. {
            // take my energy!
            for (r, row) in grid.iter_mut().enumerate() {
                for (c, cell) in row.iter_mut().enumerate() {
                    *cell += 1;
                    if *cell > 9 {
                        to_flash.push_back((c, r));
                    }
                }
            }

            while let Some((c, r)) = to_flash.pop_front() {
                if flashed[r * grid_rows + c] == step as isize {
                    continue;
                }
                flashed[r * grid_rows + c] = step as isize;

                for rr in -1..=1 {
                    for cc in -1..=1 {
                        if rr == 0 && cc == 0 {
                            continue;
                        }

                        let r = (r as i64 + rr) as usize;
                        let c = (c as i64 + cc) as usize;

                        if let Some(row) = grid.get_mut(r) {
                            if let Some(val) = row.get_mut(c) {
                                *val += 1;
                                if *val > 9 {
                                    to_flash.push_back((c, r));
                                }
                            }
                        }
                    }
                }
            }

            let mut flash_count = 0;
            for row in grid.iter_mut() {
                for cell in row.iter_mut() {
                    if *cell > 9 {
                        *cell = 0;
                        flash_count += 1;
                    }
                }
            }

            if step < 100 {
                first_100_flashes += flash_count;
            }

            if flash_count == grid_size {
                return (first_100_flashes, step + 1);
            }

            to_flash.clear();
        }

        unreachable!();
    }

    b.iter(|| {
        assert_eq!(solve(INPUT), (1591, 314));
    });
}

#[bench]
fn bench_solve_11_all_vecs_shrtcrct_n_rng_lps_n_mmze(b: &mut test::Bencher) {
    fn solve(input: &str) -> (usize, usize) {
        let mut grid = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).expect("not a number"))
                    .collect()
            })
            .collect::<Vec<Vec<u32>>>();

        step(&mut grid)
    }

    fn step(grid: &mut Vec<Vec<u32>>) -> (usize, usize) {
        let grid_size = grid.iter().map(|row| row.len()).sum::<usize>();
        let grid_rows = grid[0].len();

        let mut first_100_flashes = 0;

        let mut to_flash = Vec::<(usize, usize)>::with_capacity(100);
        let mut flashed = vec![-1_isize; grid_size];

        for step in 0.. {
            // take my energy!
            for (r, row) in grid.iter_mut().enumerate() {
                for (c, cell) in row.iter_mut().enumerate() {
                    *cell += 1;
                    if *cell > 9 {
                        to_flash.push((c, r));
                    }
                }
            }

            while let Some((c, r)) = to_flash.pop() {
                if flashed[r * grid_rows + c] == step as isize {
                    continue;
                }
                flashed[r * grid_rows + c] = step as isize;

                for rr in -1..=1 {
                    for cc in -1..=1 {
                        if rr == 0 && cc == 0 {
                            continue;
                        }

                        let r = (r as i64 + rr) as usize;
                        let c = (c as i64 + cc) as usize;

                        if let Some(row) = grid.get_mut(r) {
                            if let Some(val) = row.get_mut(c) {
                                *val += 1;
                                if *val > 9 {
                                    to_flash.push((c, r));
                                }
                            }
                        }
                    }
                }
            }

            let mut flash_count = 0;
            for row in grid.iter_mut() {
                for cell in row.iter_mut() {
                    if *cell > 9 {
                        *cell = 0;
                        flash_count += 1;
                    }
                }
            }

            if step < 100 {
                first_100_flashes += flash_count;
            }

            if flash_count == grid_size {
                return (first_100_flashes, step + 1);
            }

            to_flash.clear();
        }

        unreachable!();
    }

    b.iter(|| {
        assert_eq!(solve(INPUT), (1591, 314));
    });
}

#[bench]
fn bench_solve_12_all_vecs_1dmensnl_shrtcrct_n_rng_lps_n_mmze(b: &mut test::Bencher) {
    fn solve(input: &str) -> (usize, usize) {
        let mut grid = input
            .lines()
            .flat_map(|line| line.chars().map(|c| c.to_digit(10).expect("not a number")))
            .collect::<Vec<u32>>();

        let rows = input.lines().count();

        step(&mut grid, rows)
    }

    fn step(grid: &mut Vec<u32>, grid_rows: usize) -> (usize, usize) {
        let mut first_100_flashes = 0;

        let mut to_flash = Vec::<(usize, usize)>::with_capacity(100);
        let mut flashed = vec![-1_isize; grid.len()];

        for step in 0.. {
            // take my energy!
            for (c, cell) in grid.iter_mut().enumerate() {
                *cell += 1;
                if *cell > 9 {
                    to_flash.push((c % grid_rows, c / grid_rows));
                }
            }

            while let Some((c, r)) = to_flash.pop() {
                if flashed[r * grid_rows + c] == step as isize {
                    continue;
                }
                flashed[r * grid_rows + c] = step as isize;

                for rr in -1..=1 {
                    for cc in -1..=1 {
                        if rr == 0 && cc == 0 {
                            continue;
                        }

                        let r = (r as i64 + rr) as usize;
                        let c = (c as i64 + cc) as usize;

                        if let Some(val) = grid.get_mut(r) {
                            *val += 1;
                            if *val > 9 {
                                to_flash.push((c, r));
                            }
                        }
                    }
                }
            }

            let mut flash_count = 0;
            for cell in grid.iter_mut() {
                if *cell > 9 {
                    *cell = 0;
                    flash_count += 1;
                }
            }

            if step < 100 {
                first_100_flashes += flash_count;
            }

            if flash_count == grid.len() {
                return (first_100_flashes, step + 1);
            }

            to_flash.clear();
        }

        unreachable!();
    }

    b.iter(|| {
        assert_eq!(solve(INPUT), (1591, 314));
    });
}
