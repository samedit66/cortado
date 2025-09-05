use std::thread::current;

use unicode_segmentation::{Graphemes, UnicodeSegmentation};

fn main() {
    let mut tokenizer = Tokenizer::new("   method f(a, ");
    println!("{}", tokenizer.finished);
}

enum Token {
    Identifier(String),
    Dot,
    Comma,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Assign,
    Semi,
    Eof,
}

struct TokenAt {
    token: Token,
    line: usize,
    col: usize,
}

struct Tokenizer<'a> {
    istream: Graphemes<'a>,
    last: &'a str,
    finished: bool,
    line: usize,
    col: usize,
}

impl<'a> Tokenizer<'a> {
    fn new(source: &'a str) -> Tokenizer<'a> {
        Tokenizer {
            istream: source.graphemes(true),
            last: "",
            finished: false,
            line: 1,
            col: 0,
        }
    }

    fn next(&mut self) {
        if self.finished {
            return;
        }

        match self.istream.next() {
            Some(grapheme) => {
                self.last = grapheme;

                if self.last == "\n" {
                    self.line += 1;
                    self.col = 1;
                } else {
                    self.col += 1;
                }
            },
            None => {
                self.finished = true;
            }
        }
    }

    fn tokenize(&mut self) -> Token {
        while !self.finished && is_whitespace(self.last) {
            self.next();
        }

        if self.finished {
            return Token::Eof;
        }

        match self.last {
            "." => Token::Dot,
            "," => Token::Comma,
            "(" => Token::LParen,
            ")" => Token::RParen,
            "{" => Token::LBrace,
            "}" => Token::RBrace,
            ";" => Token::Semi,
            _ => panic!("Unknown symbol")
        }
    }
}

fn is_whitespace(s: &str) -> bool {
    s.trim().is_empty()
}