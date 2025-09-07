use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum Token {
    Ident(String),
    Int(String),
    Float(String),
    Str(String),
    Dot,
    Comma,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Assign,
    Colon,
    Semicolon,
    Less,
    And,
    Or,
    Plus,
    Minus,
    Star,
    Slash,
    Tilda,

    Le,
    Greater,
    Ge,
    Equal,
    NotEqual,
    RArrow,

    KwMethod,
    KwGiven,
    KwWhen,
    KwDefault,
    KwTrue,
    KwFalse,

    Error(String),
    Eof,
}

#[derive(Debug, PartialEq)]
pub struct TokenAt {
    token: Token,
    line: usize,
    col: usize,
}

pub struct Tokenizer<'a> {
    input: Chars<'a>,
    last: char,
    eof: bool,
    line: usize,
    col: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(source: &'a str) -> Tokenizer<'a> {
        Tokenizer {
            input: source.chars(),
            // Could be any character because it always gets skipped as `eof` is false
            last: ' ',
            eof: false,
            line: 1,
            col: 1,
        }
    }

    pub fn next_token(&mut self) -> TokenAt {
        self.consume_whitespace();
        self.consume_comment();

        let line = self.line;
        let col = self.col;

        if self.eof {
            return TokenAt {
                token: Token::Eof,
                line,
                col,
            };
        }

        let mut advance = true;
        let token = match self.last {
            '.' => Token::Dot,
            ',' => Token::Comma,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            ':' => Token::Colon,
            ';' => Token::Semicolon,
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Star,
            '[' => Token::LBracket,
            ']' => Token::RBracket,
            '~' => Token::Tilda,
            '/' => {
                self.advance();

                if self.is_current_char('=') {
                    Token::NotEqual
                } else {
                    advance = false;
                    Token::Slash
                }
            }
            '&' => Token::And,
            '|' => Token::Or,
            '<' => {
                self.advance();

                if self.is_current_char('=') {
                    Token::Le
                } else {
                    advance = false;
                    Token::Less
                }
            }
            '>' => {
                self.advance();

                if self.is_current_char('=') {
                    Token::Ge
                } else {
                    advance = false;
                    Token::Greater
                }
            }
            '=' => {
                self.advance();

                if self.is_current_char('=') {
                    Token::Equal
                } else if self.is_current_char('>') {
                    Token::RArrow
                } else {
                    advance = false;
                    Token::Assign
                }
            }
            _ => {
                // Each of the following methods leave in `self.last`
                // the character after the found token, so no need to advance
                advance = false;

                if self.check(char::is_alphabetic) {
                    self.read_identifier_or_keyword()
                } else if self.check(char::is_numeric) {
                    self.read_number_literal()
                } else if self.is_current_char('"') {
                    self.read_string_literal()
                } else {
                    self.error(&format!(
                        "Unknown character '{}' -- did you mean an operator, \
                    identifier or a string? Try adding spaces, or wrap text in double quotes.",
                        self.peek()
                    ))
                }
            }
        };

        if advance {
            self.advance();
        }

        TokenAt { token, line, col }
    }

    fn peek(&self) -> char {
        self.last
    }

    fn advance(&mut self) {
        if self.eof {
            return;
        }

        match self.input.next() {
            Some(c) => {
                self.last = c;

                if self.last == '\n' {
                    self.line += 1;
                    self.col = 1;
                } else {
                    self.col += 1;
                }
            }
            None => {
                self.eof = true;
            }
        }
    }

    fn read_identifier_or_keyword(&mut self) -> Token {
        let mut buf = String::new();

        while self.possible_part_of_identifier() {
            buf.push(self.last);
            self.advance();
        }

        // TODO: replace it with a HashMap
        match buf.as_str() {
            "method" => Token::KwMethod,
            "given" => Token::KwGiven,
            "when" => Token::KwWhen,
            "default" => Token::KwDefault,
            "true" => Token::KwTrue,
            "false" => Token::KwFalse,
            _ => Token::Ident(buf),
        }
    }

    fn consume_whitespace(&mut self) {
        while self.check(char::is_whitespace) {
            self.advance();
        }
    }

    fn consume_comment(&mut self) {
        if self.is_current_char('#') {
            self.advance();

            while !self.is_current_char('\n') {
                self.advance();
            }
        }
    }

    fn read_number_literal(&mut self) -> Token {
        let mut buf = String::new();
        let mut dot = false;

        while self.possible_part_of_number() {
            buf.push(self.last);
            self.advance();

            if !dot && self.is_current_char('.') {
                dot = true;
                buf.push(self.last);
                self.advance();
            }
        }

        if dot {
            Token::Float(buf)
        } else {
            Token::Int(buf)
        }
    }

    fn read_string_literal(&mut self) -> Token {
        let mut buf = String::new();

        // Skip the first quote without checking it (checking is done in `tokenize`)
        self.advance();

        while !self.eof && !self.is_current_char('"') {
            if self.last == '\n' {
                return self.error(
                    "Unterminated string -- found a newline before \
                the closing quote. Keep strings on one line.",
                );
            }

            buf.push(self.last);
            self.advance();
        }

        if self.eof {
            return self.error(
                "Unterminated string -- reached end of input before closing quote. \
            Add a closing '\"'",
            );
        }

        // Skip the second quote
        self.advance();

        Token::Str(buf)
    }

    fn check<F>(&self, pred: F) -> bool
    where
        F: Fn(char) -> bool,
    {
        !self.eof && pred(self.last)
    }

    fn is_current_char(&self, ch: char) -> bool {
        self.check(|c| c == ch)
    }

    fn possible_part_of_identifier(&self) -> bool {
        self.check(|c| {
            c.is_alphabetic() || c.is_numeric() || c == '?' || c == '!' || c == '-' || c == '_'
        })
    }

    fn possible_part_of_number(&self) -> bool {
        self.check(|c| c.is_numeric() || c == '_')
    }

    fn error(&self, msg: &str) -> Token {
        Token::Error(format!("{}:{}: {}", self.line, self.col, msg))
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = TokenAt;

    fn next(&mut self) -> Option<Self::Item> {
        let ta = self.next_token();

        if matches!(ta.token, Token::Eof | Token::Error(_)) {
            None
        } else {
            Some(ta)
        }
    }
}
