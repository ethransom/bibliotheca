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
    let pairs = parse(input);

    dbg!(pairs);

    (0, 0)
}

fn parse(input: &str) -> Result<Vec<(Packet, Packet)>> {
    input
        .split("\n\n")
        .map(|pair| {
            let (a, b) = pair
                .split_once("\n")
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
        } else if let Some(int) = parse_int(&mut self.input) {
            Some(Token::Int(int))
        } else {
            None
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

#[derive(Debug, PartialEq)]
enum Packet {
    Int(u8),
    List(Vec<Packet>),
}

#[test]
fn test_example() {
    assert_eq!(solve(EXAMPLE), (0, 0));
}

#[bench]
fn bench_solve_current(b: &mut test::Bencher) {
    b.iter(|| {
        assert_eq!(solve(INPUT), (0, 0));
    });
}
