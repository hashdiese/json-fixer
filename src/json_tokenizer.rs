use std::iter::Peekable;
use std::str::Chars;

use crate::jsonfixer_error::JsonFixerError;

#[derive(Debug, PartialEq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Colon,
    Comma,
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
}

impl Token {
    pub fn get(&self) -> String {
        match self {
            Self::LeftBrace => '{'.to_string(),
            Self::RightBrace => '}'.to_string(),
            Self::LeftBracket => '['.to_string(),
            Self::RightBracket => ']'.to_string(),
            Self::Colon => ':'.to_string(),
            Self::Comma => ','.to_string(),
            Self::String(str) => str.to_string(),
            Self::Number(num) => num.to_string(),
            Self::Boolean(bool) => bool.to_string(),
            Self::Null => "Null".to_string(),
        }
    }
}

pub struct JsonTokenizer<'a> {
    input: Peekable<Chars<'a>>,
    line: usize,
    column: usize,
}

impl<'a> JsonTokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.chars().peekable(),
            line: 1,
            column: 0,
        }
    }

    pub fn next_token(&mut self) -> Result<Option<Token>, JsonFixerError> {
        // Skip whitespaces
        self.skip_whitespace();

        if let Some(ch) = self.advance() {
            match ch {
                '{' => Ok(Some(Token::LeftBrace)),
                '}' => Ok(Some(Token::RightBrace)),
                '[' => Ok(Some(Token::LeftBracket)),
                ']' => Ok(Some(Token::RightBracket)),
                ':' => Ok(Some(Token::Colon)),
                ',' => Ok(Some(Token::Comma)),
                '\'' | '"' => self.tokenize_string(ch).map(Some),
                '+' | '-' | '0'..='9' => self.tokenize_number(ch).map(Some),
                'a'..='z' | 'A'..='Z' | '_' => self.tokenize_identifier(ch).map(Some),
                ch => Err(JsonFixerError::UnexpectedCharacter(ch, Position {
                    line: self.line,
                    column: self.column,
                })),
            }
        } else {
            Ok(None)
        }
    }
    fn skip_whitespace(&mut self) {
        while let Some(&ch) = self.peek() {
            if !ch.is_whitespace() {
                break;
            }
            self.advance();
        }
    }

    fn peek(&mut self) -> Option<&char> {
        self.input.peek()
    }
    fn advance(&mut self) -> Option<char> {
        if let Some(ch) = self.input.next() {
            self.column += 1;

            if ch == '\n' {
                self.line += 1;
                self.column = 1;
            }
            Some(ch)
        } else {
            None
        }
    }
    pub fn current_position(&self) -> Position {
        Position {
            line: self.line,
            column: self.column,
        }
    }
    fn tokenize_string(&mut self, quote_char: char) -> Result<Token, JsonFixerError> {
        let start_pos = self.current_position();
        let mut result = String::new();

        while let Some(ch) = self.advance() {
            match ch {
                ch if ch == quote_char => return Ok(Token::String(result)),
                '\\' => {
                    if let Some(next_ch) = self.advance() {
                        match next_ch {
                            '"' | '\\' | '/' => result.push(next_ch),
                            // handle controle characters
                            'b' => result.push('\x08'), // \b = backspace
                            'f' => result.push('\x0C'),
                            'n' => result.push('\n'),
                            'r' => result.push('\r'),
                            't' => result.push('\t'),
                            'u' => {
                                // Handle unicode escape sequences
                                let mut hex = String::with_capacity(4);
                                for _ in 0..4 {
                                    if let Some(h) = self.advance() {
                                        hex.push(h);
                                    }
                                }
                                if let Ok(code) = u32::from_str_radix(&hex, 16) {
                                    if let Some(chr) = std::char::from_u32(code) {
                                        result.push(chr);
                                    }
                                }
                            }
                            _ => result.push(next_ch),
                        }
                    }
                }
                _ => result.push(ch),
            }
        }
        Err(JsonFixerError::UnmatchedQuotes(start_pos)) // placeholder
    }

    fn tokenize_number(&mut self, first_char: char) -> Result<Token, JsonFixerError> {
        let start_pos = self.current_position();
        let mut number = String::from(first_char);

        // Handle numbers that start with plus
        if first_char == '+' {
            // If there is no digit after +, it's invalid
            if let Some(next_char) = self.peek() {
                if !next_char.is_digit(10) {
                    return Err(JsonFixerError::InvalidNumber(number, start_pos));
                }
            } else {
                return Err(JsonFixerError::InvalidNumber(number, start_pos));
            }

            // A + on a positive number not needed
            number.clear(); // Remove the + sign 
        }

        while let Some(&ch) = self.peek() {
            if !ch.is_digit(10) && ch != '.' && ch != 'e' && ch != 'E' && ch != '+' && ch != '-' {
                break;
            }
            number.push(self.advance().unwrap());
        }

        number
            .parse::<f64>()
            .map(Token::Number)
            .map_err(|_| JsonFixerError::InvalidNumber(number, start_pos))
    }

    fn tokenize_identifier(&mut self, first_char: char) -> Result<Token, JsonFixerError> {
        let mut ident = String::from(first_char);

        while let Some(&ch) = self.peek() {
            if !ch.is_alphanumeric() && ch != '_' {
                break;
            }
            ident.push(self.advance().unwrap());
        }

        match ident.as_str() {
            "true" => Ok(Token::Boolean(true)),
            "false" => Ok(Token::Boolean(false)),
            "null" => Ok(Token::Null),
            _ => Ok(Token::String(ident)),
        }
    }
}
