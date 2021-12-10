#![feature(test)]

extern crate test;

const EXAMPLE: &str = include_str!("example10.txt");
const INPUT: &str = include_str!("input10.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    let lines = input.lines().map(parse_line);

    let invalid_score = lines
        .clone()
        .map(|result| match result {
            Ok(_) => 0,
            Err(')') => 3,
            Err(']') => 57,
            Err('}') => 1197,
            Err('>') => 25137,
            Err(_) => panic!("unknown syntax error"),
        })
        .sum();

    let mut scores = lines
        .clone()
        .flat_map(|result| match result {
            Ok(stack) => {
                assert!(!stack.is_empty());

                Some(stack.iter().rev().fold(0, |mut score, &brace| {
                    score *= 5;
                    score += match get_closing(brace) {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => panic!("don't know how to score"),
                    };
                    score
                }))
            }
            Err(_) => None,
        })
        .collect::<Vec<usize>>();

    scores.sort_unstable(); // unstable is slightly faster

    let autocomplete_score = scores[scores.len() / 2];

    (invalid_score, autocomplete_score)
}

fn parse_line(line: &str) -> Result<Vec<char>, char> {
    let mut stack: Vec<char> = Vec::new();

    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            ')' | ']' | '}' | '>' => {
                if let Some(m) = stack.pop() {
                    if get_closing(m) == c {
                        continue;
                    }
                }
                return Err(c);
            }
            _ => panic!("illegal character"),
        }
    }
    Ok(stack)
}

fn get_closing(brace: char) -> char {
    match brace {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("illegal character"),
    }
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (26397, 288957));
}

#[bench]
fn bench_solve_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (311895, 2904180541));
    });
}
