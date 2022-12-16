#![feature(test)]

use std::iter::Peekable;

use anyhow::{anyhow, Context, Result};

extern crate test;

const EXAMPLE: &str = include_str!("example13.txt");
const INPUT: &str = include_str!("input13.txt");

fn main() {
    dbg!(solve(EXAMPLE));
    dbg!(solve(INPUT));
}

fn solve(input: &str) -> (usize, usize) {
    let pairs = parse(input).expect("could not parse input");

    (
        pairs
            .iter()
            .enumerate()
            .map(|(index, (left, right))| {
                if left.partial_cmp(right).unwrap() == std::cmp::Ordering::Less {
                    index + 1
                } else {
                    0
                }
            })
            .sum(),
        0,
    )
}

#[derive(Debug, PartialEq)]
enum Packet {
    Int(u8),
    List(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Packet::Int(a), Packet::Int(b)) => a.partial_cmp(b),
            (Packet::List(a), Packet::List(b)) => {
                let mut a = a.iter();
                let mut b = b.iter();

                loop {
                    match (a.next(), b.next()) {
                        (Some(a), Some(b)) => match a.partial_cmp(b) {
                            Some(std::cmp::Ordering::Equal) => continue,
                            Some(ordering) => return Some(ordering),
                            None => return None, // ???
                        },
                        (Some(_), None) => return Some(std::cmp::Ordering::Greater),
                        (None, Some(_)) => return Some(std::cmp::Ordering::Less),
                        (None, None) => return Some(std::cmp::Ordering::Equal),
                    }
                }
            }
            (Packet::Int(int), list /* must be list */) => {
                Packet::List(vec![Packet::Int(*int)]).partial_cmp(list)
            }
            (list /* must be list */, Packet::Int(int)) => {
                list.partial_cmp(&Packet::List(vec![Packet::Int(*int)]))
            }
        }
    }
}

fn parse(input: &str) -> Result<Vec<(Packet, Packet)>> {
    input
        .split("\n\n")
        .map(|pair| {
            let (a, b) = pair
                .split_once('\n')
                .context("requires two packets per block")?;

            Ok((
                parse_packet(&mut Lexer::from(a.as_bytes()).peekable())?
                    .context("expected packet")?,
                parse_packet(&mut Lexer::from(b.as_bytes()).peekable())?
                    .context("expected packet")?,
            ))
        })
        .collect()
}

fn parse_packet(lexer: &mut Peekable<Lexer>) -> Result<Option<Packet>> {
    if let Some(Token::OpenBracket) = lexer.peek() {
        lexer.next();
    } else {
        return Ok(None);
    };

    let mut list = vec![];

    loop {
        if let Some(packet) = parse_packet(lexer)? {
            list.push(packet);
            continue;
        } else {
            match lexer.next() {
                Some(Token::Int(int)) => list.push(Packet::Int(int)),
                Some(Token::Comma) => continue,
                Some(Token::CloseBracket) => return Ok(Some(Packet::List(list))),
                _ => return Err(anyhow!("expected , or ] or digit")),
            }
        }

        if let Some(Token::Comma) = lexer.peek() {
            lexer.next();
            continue;
        } else if let Some(Token::CloseBracket) = lexer.next() {
            return Ok(Some(Packet::List(list)));
        } else {
            return Err(anyhow!("expected , or ]"));
        }
    }
}

#[derive(Debug)]
enum Token {
    Int(u8),
    OpenBracket,
    CloseBracket,
    Comma,
}

struct Lexer<'a> {
    input: &'a [u8],
}

impl<'a> Lexer<'a> {
    fn from(input: &'a [u8]) -> Self {
        Self { input }
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(input) = self.input.strip_prefix(b"[") {
            self.input = input;
            Some(Token::OpenBracket)
        } else if let Some(input) = self.input.strip_prefix(b"]") {
            self.input = input;
            Some(Token::CloseBracket)
        } else if let Some(input) = self.input.strip_prefix(b",") {
            self.input = input;
            Some(Token::Comma)
        } else {
            parse_int(&mut self.input).map(Token::Int)
        }
    }
}

fn parse_int(input: &mut &[u8]) -> Option<u8> {
    let original = *input;
    while !input.is_empty() && input[0].is_ascii_digit() {
        *input = &input[1..];
    }
    let int = &original[..original.len() - input.len()];
    if int.is_empty() {
        return None;
    }
    let int = std::str::from_utf8(int)
        .unwrap() // SAFETY: we know it's ascii digits
        .parse::<u8>()
        .expect("expected integer"); // SAFETY: we know it's ascii digits
    Some(int)
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (13, 0));
}

#[bench]
fn bench_solve_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (6_656, 0));
    });
}
