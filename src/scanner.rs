use std::collections::HashMap;
use std::sync::OnceLock;

use crate::Error;
use crate::Result;

static KEYWORDS: OnceLock<HashMap<&'static str, Token>> = OnceLock::new();

#[derive(Debug, Clone)]
pub enum Token {
    Ident(String),
    String(String),
    Number(f64),
    True,
    False,
    Comma,
    Dot,
    Plus,
    Minus,
    Star,
    Slash,
    Eq,
    EqEq,
    Not,
    Ne,
    Gt,
    Ge,
    Lt,
    Le,
    And,
    Or,
    If,
    Else,
    For,
    While,
    Nil,
    Fun,
    Class,
    Return,
    Super,
    This,
    Var,
    Print,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Semicolon,
    Eof,
}

pub struct Scanner {
    source: String,
    line: usize,
    current: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            line: 1,
            current: 0,
        }
    }

    pub fn read_tokens(&mut self) -> Result<Vec<Token>> {
        let mut tokens = vec![];

        while !self.is_at_end() {
            if let Some(token) = self.read_token()? {
                tokens.push(token);
            }
        }

        tokens.push(Token::Eof);
        Ok(tokens)
    }

    fn read_token(&mut self) -> Result<Option<Token>> {
        let token = match self.advance() {
            '"' => Some(Token::String(self.string_lit()?)),
            ',' => Some(Token::Comma),
            '.' => Some(Token::Dot),
            '+' => Some(Token::Plus),
            '-' => Some(Token::Minus),
            '*' => Some(Token::Star),
            '=' => {
                if self.next_matches('=') {
                    Some(Token::EqEq)
                } else {
                    Some(Token::Eq)
                }
            }
            '!' => {
                if self.next_matches('=') {
                    Some(Token::Ne)
                } else {
                    Some(Token::Not)
                }
            }
            '>' => {
                if self.next_matches('=') {
                    Some(Token::Ge)
                } else {
                    Some(Token::Gt)
                }
            }
            '<' => {
                if self.next_matches('=') {
                    Some(Token::Le)
                } else {
                    Some(Token::Lt)
                }
            }
            '(' => Some(Token::LeftParen),
            ')' => Some(Token::RightParen),
            '{' => Some(Token::LeftBrace),
            '}' => Some(Token::RightBrace),
            ';' => Some(Token::Semicolon),
            '/' => {
                if self.next_matches('/') {
                    self.comment();
                    None
                } else {
                    Some(Token::Slash)
                }
            }
            ' ' | '\r' | '\t' => None,
            '\n' => {
                self.line += 1;
                None
            }
            c if c.is_digit(10) => Some(Token::Number(self.number_lit(c)?)),
            c if is_valid_identifier_start(c) => {
                let ident = self.identifier(c);
                keyword(&ident).cloned().or(Some(Token::Ident(ident)))
            }
            c => {
                return Err(Error::message(
                    &self.source,
                    &format!("Unexpected character \'{c}\'"),
                ));
            }
        };

        Ok(token)
    }

    fn string_lit(&mut self) -> Result<String> {
        let mut s = String::new();

        while !matches!(self.peek(), Some('"')) && !self.is_at_end() {
            if matches!(self.peek(), Some('\n')) {
                // Multi-line string handling
                self.line += 1;
            }

            s.push(self.advance());
        }

        if self.is_at_end() {
            return Err(Error::message(
                &self.source,
                &format!("Unterminated string"),
            ));
        }

        self.advance();
        Ok(s)
    }

    fn number_lit(&mut self, c: char) -> Result<f64> {
        let mut s = String::new();
        s.push(c);

        while self.peek().is_some_and(|c| c.is_digit(10)) {
            s.push(self.advance());
        }

        if matches!(self.peek(), Some('.')) && self.peek_next().is_some_and(|c| c.is_digit(10)) {
            s.push(self.advance());

            while self.peek().is_some_and(|c| c.is_digit(10)) {
                s.push(self.advance());
            }
        }

        let number = s.parse::<f64>()?;
        Ok(number)
    }

    fn identifier(&mut self, c: char) -> String {
        let mut s = String::new();
        s.push(c);

        while self.peek().is_some_and(|c| is_valid_identifier_char(c)) {
            s.push(self.advance());
        }

        s
    }

    fn comment(&mut self) {
        while !matches!(self.peek(), Some('\n')) && !self.is_at_end() {
            self.advance();
        }
    }

    fn next_matches(&mut self, c: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if !self.peek().is_some_and(|next| next == c) {
            return false;
        }

        self.current += 1;
        true
    }

    fn advance(&mut self) -> char {
        let c = self.peek().expect("index into source string is invalid");
        self.current += 1;
        c
    }

    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.current)
    }

    fn peek_next(&self) -> Option<char> {
        self.source.chars().nth(self.current + 1)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}

fn is_valid_identifier_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

fn is_valid_identifier_char(c: char) -> bool {
    is_valid_identifier_start(c) || c.is_digit(10)
}

fn keyword(s: &str) -> Option<&Token> {
    KEYWORDS
        .get_or_init(|| {
            let mut keywords = HashMap::new();

            keywords.insert("true", Token::True);
            keywords.insert("false", Token::False);
            keywords.insert("and", Token::And);
            keywords.insert("or", Token::Or);
            keywords.insert("if", Token::If);
            keywords.insert("else", Token::Else);
            keywords.insert("for", Token::For);
            keywords.insert("while", Token::While);
            keywords.insert("nil", Token::Nil);
            keywords.insert("fun", Token::Fun);
            keywords.insert("class", Token::Class);
            keywords.insert("return", Token::Return);
            keywords.insert("super", Token::Super);
            keywords.insert("this", Token::This);
            keywords.insert("var", Token::Var);
            keywords.insert("print", Token::Print);

            keywords
        })
        .get(s)
}
