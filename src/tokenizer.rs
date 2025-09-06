use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum Token {
    Ident(String),
    Num(String),
    Str(String),
    Dot,
    Comma,
    LParen,
    RParen,
    LBrace,
    RBrace,
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

    LessThan,
    Greater,
    GreaterThan,
    Equal,
    NotEqual,

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
        while !self.eof && self.last.is_whitespace() {
            self.next();
        }

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
            '/' => {
                self.next();

                if !self.eof && self.last == '=' {
                    Token::NotEqual
                } else {
                    advance = false;
                    Token::Slash
                }
            }
            '&' => Token::And,
            '|' => Token::Or,
            '<' => {
                self.next();

                if !self.eof && self.last == '=' {
                    Token::LessThan
                } else {
                    advance = false;
                    Token::Less
                }
            }
            '>' => {
                self.next();

                if !self.eof && self.last == '=' {
                    Token::GreaterThan
                } else {
                    advance = false;
                    Token::Greater
                }
            }
            '=' => {
                self.next();

                if !self.eof && self.last == '=' {
                    Token::Equal
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
                } else if self.last == '"' {
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
            self.next();
        }

        TokenAt { token, line, col }
    }

    fn next(&mut self) {
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

        while !self.eof
            && (self.last.is_alphabetic()
                || self.last.is_numeric()
                || self.last == '?'
                || self.last == '!'
                || self.last == '-'
                || self.last == '_')
        {
            buf.push(self.last);
            self.next();
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

    fn read_num(&mut self) -> Token {
        let mut buf = String::new();

        while !self.eof && (self.last.is_numeric() || self.last == '_') {
            buf.push(self.last);
            self.next();
        }

        Token::Num(buf)
    }

    fn read_str(&mut self) -> Token {
        let mut buf = String::new();

        // Skip the first quote without checking it (checking is done in `tokenize`)
        self.next();

        while !self.eof && self.last != '"' {
            if self.last == '\n' {
                return Token::Error(format!(
                    "Unterminated string started at {}:{} — found a newline before the closing quote. \
                        Keep strings on one line.",
                    self.line, self.col,
                ));
            }

            buf.push(self.last);
            self.next();
        }

        if self.eof {
            return Token::Error(format!(
                "Unterminated string started at {}:{} — reached end of input before closing quote. \
                    Add a closing '\"'.",
                self.line, self.col,
            ));
        }

        // Skip the second quote
        self.next();

        Token::Str(buf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: collect tokens until we hit Eof or Error (both considered terminal here).
    fn collect_tokens(src: &str) -> Vec<Token> {
        let mut t = Tokenizer::new(src);
        let mut out = Vec::new();

        loop {
            let TokenAt { token, .. } = t.tokenize();
            let terminal = matches!(token, Token::Eof | Token::Error(_));
            out.push(token);
            if terminal {
                break;
            }
        }

        out
    }

    #[test]
    fn test_single_char_tokens() {
        let input = ". , ( ) { } : ; + - * / & |";
        let got = collect_tokens(input);
        let expect = vec![
            Token::Dot,
            Token::Comma,
            Token::LParen,
            Token::RParen,
            Token::LBrace,
            Token::RBrace,
            Token::Colon,
            Token::Semicolon,
            Token::Plus,
            Token::Minus,
            Token::Star,
            Token::Slash,
            Token::And,
            Token::Or,
            Token::Eof,
        ];
        assert_eq!(got, expect);
    }

    #[test]
    fn test_comparisons_and_assignments() {
        let input = "< <= > >= = == /=";
        let got = collect_tokens(input);
        let expect = vec![
            Token::Less,
            Token::LessThan,
            Token::Greater,
            Token::GreaterThan,
            Token::Assign,
            Token::Equal,
            Token::NotEqual,
            Token::Eof,
        ];
        assert_eq!(got, expect);
    }

    #[test]
    fn test_keywords_and_identifiers() {
        let input = "method given when default true false foo foo123 bar? baz! qux-abc foo_bar";
        let got = collect_tokens(input);
        let expect = vec![
            Token::KwMethod,
            Token::KwGiven,
            Token::KwWhen,
            Token::KwDefault,
            Token::KwTrue,
            Token::KwFalse,
            Token::Ident("foo".into()),
            Token::Ident("foo123".into()),
            Token::Ident("bar?".into()),
            Token::Ident("baz!".into()),
            Token::Ident("qux-abc".into()),
            Token::Ident("foo_bar".into()),
            Token::Eof,
        ];
        assert_eq!(got, expect);
    }

    #[test]
    fn test_numbers() {
        let input = "123 4_567";
        let got = collect_tokens(input);
        let expect = vec![
            Token::Num("123".into()),
            Token::Num("4_567".into()),
            Token::Eof,
        ];
        assert_eq!(got, expect);
    }

    #[test]
    fn test_string_success() {
        let input = r#""hello""#;
        let got = collect_tokens(input);
        let expect = vec![Token::Str("hello".into()), Token::Eof];
        assert_eq!(got, expect);
    }

    #[test]
    fn test_unterminated_string_newline_error() {
        // string contains a newline before closing quote -> Error variant
        let mut tok = Tokenizer::new("\"\n");
        let TokenAt { token, .. } = tok.tokenize();
        match token {
            Token::Error(msg) => {
                assert!(
                    msg.to_lowercase().contains("unterminated string")
                        && msg.to_lowercase().contains("newline"),
                    "message did not mention newline/unterminated: {}",
                    msg
                );
            }
            other => panic!("expected Error token, got {:?}", other),
        }

        // Also test a multi-line string body (content then newline)
        let mut tok2 = Tokenizer::new(
            r#""bad
rest"#,
        );
        let TokenAt { token: token2, .. } = tok2.tokenize();
        match token2 {
            Token::Error(msg) => {
                assert!(
                    msg.to_lowercase().contains("unterminated string")
                        && msg.to_lowercase().contains("newline"),
                    "message did not mention newline/unterminated: {}",
                    msg
                );
            }
            other => panic!("expected Error token, got {:?}", other),
        }
    }

    #[test]
    fn test_unterminated_string_eof_error() {
        // string that never closes before EOF
        let mut tok = Tokenizer::new("\"not closed");
        let TokenAt { token, .. } = tok.tokenize();
        match token {
            Token::Error(msg) => {
                assert!(
                    msg.to_lowercase().contains("reached end of input")
                        || msg.to_lowercase().contains("closing"),
                    "unexpected error message: {}",
                    msg
                );
            }
            other => panic!("expected Error token, got {:?}", other),
        }
    }

    #[test]
    fn test_unknown_character_error() {
        // unknown single character (e.g. '@') should produce an Error token
        let mut tok = Tokenizer::new("@");
        let TokenAt { token, .. } = tok.tokenize();
        match token {
            Token::Error(msg) => {
                assert!(
                    msg.contains("@") && msg.to_lowercase().contains("unknown character"),
                    "unexpected error message: {}",
                    msg
                );
            }
            other => panic!("expected Error token, got {:?}", other),
        }
    }
}
