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
    istream: Chars<'a>,
    last: char,
    eof: bool,
    line: usize,
    col: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(source: &'a str) -> Tokenizer<'a> {
        Tokenizer {
            istream: source.chars(),
            // Could be any character because it always gets skipped as `eof` is false
            last: ' ',
            eof: false,
            line: 1,
            col: 0,
        }
    }

    pub fn tokenize(&mut self) -> TokenAt {
        self.skip_whitespace();
        self.skip_comment();

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
                self.next_char();

                if self.last_char_is('=') {
                    Token::NotEqual
                } else {
                    advance = false;
                    Token::Slash
                }
            }
            '&' => Token::And,
            '|' => Token::Or,
            '<' => {
                self.next_char();

                if self.last_char_is('=') {
                    Token::Le
                } else {
                    advance = false;
                    Token::Less
                }
            }
            '>' => {
                self.next_char();

                if self.last_char_is('=') {
                    Token::Ge
                } else {
                    advance = false;
                    Token::Greater
                }
            }
            '=' => {
                self.next_char();

                if self.last_char_is('=') {
                    Token::Equal
                } else if self.last_char_is('>') {
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

                if self.last.is_alphabetic() {
                    self.read_word()
                } else if self.last.is_numeric() {
                    self.read_num()
                } else if self.last_char_is('"') {
                    self.read_str()
                } else {
                    Token::Error(format!(
                        "Unknown character '{}' at {}:{} — did you mean an operator, identifier or a string? \
                        Try adding spaces, or wrap text in double quotes.",
                        self.last, line, col
                    ))
                }
            }
        };

        if advance {
            self.next_char();
        }

        TokenAt { token, line, col }
    }

    fn next_char(&mut self) {
        if self.eof {
            return;
        }

        match self.istream.next() {
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

    fn read_word(&mut self) -> Token {
        let mut buf = String::new();

        while self.possible_part_of_identifier() {
            buf.push(self.last);
            self.next_char();
        }

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

    fn skip_whitespace(&mut self) {
        while self.satisfies(char::is_whitespace) {
            self.next_char();
        }
    }

    fn skip_comment(&mut self) {
        if self.last_char_is('#') {
            self.next_char();

            while !self.last_char_is('\n') {
                self.next_char();
            }
        }
    }

    fn read_num(&mut self) -> Token {
        let mut buf = String::new();
        let mut dot = false;

        while self.possible_part_of_number() {
            buf.push(self.last);
            self.next_char();

            if !dot && self.last_char_is('.') {
                dot = true;
                buf.push(self.last);
                self.next_char();
            }
        }

        if dot {
            Token::Float(buf)
        } else {
            Token::Int(buf)
        }
    }

    fn read_str(&mut self) -> Token {
        let mut buf = String::new();

        // Skip the first quote without checking it (checking is done in `tokenize`)
        self.next_char();

        while !self.eof && !self.last_char_is('"') {
            if self.last == '\n' {
                return Token::Error(format!(
                    "Unterminated string started at {}:{} — found a newline before the closing quote. \
                        Keep strings on one line.",
                    self.line, self.col,
                ));
            }

            buf.push(self.last);
            self.next_char();
        }

        if self.eof {
            return Token::Error(format!(
                "Unterminated string started at {}:{} — reached end of input before closing quote. \
                    Add a closing '\"'.",
                self.line, self.col,
            ));
        }

        // Skip the second quote
        self.next_char();

        Token::Str(buf)
    }

    fn satisfies<F>(&self, pred: F) -> bool
    where
        F: Fn(char) -> bool,
    {
        !self.eof && pred(self.last)
    }

    fn last_char_is(&self, ch: char) -> bool {
        self.satisfies(|c| c == ch)
    }

    fn possible_part_of_identifier(&self) -> bool {
        self.satisfies(|c| {
            c.is_alphabetic() || c.is_numeric() || c == '?' || c == '!' || c == '-' || c == '_'
        })
    }

    fn possible_part_of_number(&self) -> bool {
        self.satisfies(|c| c.is_numeric() || c == '_')
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = TokenAt;

    fn next(&mut self) -> Option<Self::Item> {
        let ta = self.tokenize();

        if matches!(ta.token, Token::Eof | Token::Error(_)) {
            None
        } else {
            Some(ta)
        }
    }
}
