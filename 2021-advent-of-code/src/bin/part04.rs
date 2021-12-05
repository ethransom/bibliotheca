#![feature(test)]

extern crate test;

const EXAMPLE: &[u8] = include_bytes!("example04.txt");
const INPUT: &[u8] = include_bytes!("input04.txt");

fn main() {
    dbg!(solve1(EXAMPLE));
    dbg!(solve(EXAMPLE));
    dbg!(solve1(INPUT));
    dbg!(solve(INPUT));
}

type Board = [[u8; 5]; 5];

fn parse(input: &[u8]) -> (Vec<u8>, Vec<Board>) {
    let blocks = std::str::from_utf8(input)
        .expect("input was not utf8")
        .split("\n\n")
        .collect::<Vec<&str>>();
    let numbers: Vec<u8> = blocks[0]
        .split(",")
        .map(|n| n.parse::<u8>().expect("not a number"))
        .collect();

    let boards: Vec<Board> = blocks[1..]
        .into_iter()
        .map(|block| {
            let mut rows: Board = [[0; 5]; 5];
            for (row, line) in block.lines().enumerate() {
                for (cell, s) in line.split_whitespace().enumerate() {
                    let n = s.parse::<u8>().expect("not a number");
                    rows[row][cell] = n;
                }
            }
            rows
        })
        .collect();

    (numbers, boards)
}

fn solve1(input: &[u8]) -> usize {
    let (numbers, boards) = parse(input);
    for turn in 1..numbers.len() {
        let drawn = &numbers[0..turn];

        for board in boards.iter() {
            if has_won(board, drawn) {
                let score = score(board, drawn);

                let winning_draw = *drawn.last().unwrap();

                return score * winning_draw as usize;
            }
        }
    }

    unreachable!("no solutions!");
}

fn solve(input: &[u8]) -> (usize, usize) {
    let (numbers, boards) = parse(input);

    let mut wins_on: Vec<Option<usize>> = vec![None; boards.len()];
    for turn in 1..numbers.len() {
        let drawn = &numbers[0..turn];

        for (board_num, board) in boards.iter().enumerate() {
            if matches!(wins_on[board_num], None) && has_won(board, drawn) {
                wins_on[board_num] = Some(turn);
            }
        }
    }

    // do we really need this check? puzzle input seems to always satisfy this...
    let wins_on: Vec<usize> = wins_on
        .into_iter()
        .map(|win| {
            if let Some(win) = win {
                win
            } else {
                panic!("not all boards won!")
            }
        })
        .collect();

    let (board_no, turn) = wins_on
        .into_iter()
        .enumerate()
        .max_by(|(_, turn_a), (_, turn_b)| turn_a.cmp(turn_b))
        .expect("no wins");

    let score = score(&boards[board_no], &numbers[0..turn]);
    let last_draw = numbers[turn - 1];

    (score * last_draw as usize, solve1(input))
}

fn has_won(board: &Board, drawn: &[u8]) -> bool {
    'nextrow: for row in board {
        for cell in row {
            if let None = drawn.iter().position(|&r| r == *cell) {
                continue 'nextrow;
            }
        }

        return true;
    }

    'nextcol: for col in 0..5 {
        for row in board {
            if let None = drawn.iter().position(|&r| r == row[col]) {
                continue 'nextcol;
            }
        }

        return true;
    }

    false
}

fn score(board: &Board, drawn: &[u8]) -> usize {
    let mut sum: usize = 0;
    for row in board {
        for cell in row {
            if let None = drawn.iter().position(|r| *r == *cell) {
                sum += *cell as usize;
            }
        }
    }
    sum
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (1924, 4512));
}

#[bench]
fn bench_solve_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (8468, 39984));
    });
}
