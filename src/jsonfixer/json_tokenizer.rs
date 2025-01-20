//! JSON tokenizer module that converts input text into a stream of JSON tokens.
//! 
//! This module handles the lexical analysis of JSON input, including support for
//! various numeric formats, string escape sequences, and tracking of position information.

use std::iter::Peekable;
use std::str::Chars;

use crate::jsonfixer_error::SyntaxError;
use crate::JsonFixerError;

/// Represents a position in the input text.
#[derive(Debug, PartialEq, Clone)]
pub struct Position {
    /// Line number (1-based)
    pub line: usize,
    /// Column number (0-based)
    pub column: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token{
    LeftBrace(Position),              // '{'
    RightBrace(Position),             // '}'
    LeftBracket(Position),            // '['
    RightBracket(Position),           // ']'
    Colon(Position),                  // ':'
    Comma(Position),                  // ','
    String(String, Position),         // JSON string
    Number(String, Position),         // JSON number will kept as string to preserve the numbers like 1e5
    Boolean(bool, Position),          // true or false
    Null(Position),
    Whitespace(String, Position),     // null
    UnquotedString(String, Position),                   // null
}

impl Token {
    /// Converts the token to its string representation.
    pub fn get(&self) -> String {
        match self {
            Self::LeftBrace (_) => "'{'".to_string(),
            Self::RightBrace (_) => "'}'".to_string(),
            Self::LeftBracket (_) => "'['".to_string(),
            Self::RightBracket (_) => "']'".to_string(),
            Self::Colon  (_) => "':'".to_string(),
            Self::Comma  (_) => "','".to_string(),
            Self::String(s, _) => format!("String({s})"),
            Self::Number(n, _) => format!("Number({n})"),
            Self::Boolean(b, _) => format!("Boolean({b})"),
            Self::Null  (_) => "null".to_string(),
            Self::Whitespace(s, _) => format!("{}", s),
            Self::UnquotedString(s, _) => format!("{}", s),

        }
    }
    pub fn pos(&self)-> &Position {
        match self {
            Self::LeftBrace (pos) => pos,
            Self::RightBrace (pos) => pos,
            Self::LeftBracket (pos) => pos,
            Self::RightBracket (pos) => pos,
            Self::Colon  (pos) => pos,
            Self::Comma  (pos) => pos,
            Self::String(_, pos) => pos,
            Self::Number(_, pos) => pos,
            Self::Boolean(_, pos) => pos,
            Self::Null  (pos) => pos,
            Self::Whitespace(_, pos) => pos,
            Self::UnquotedString(_, pos) => pos,
        }
    }
}

/// Tokenizer that converts JSON input text into a stream of tokens.
pub struct JsonTokenizer<'a> {
    input: Peekable<Chars<'a>>,
    line: usize,
    column: usize,
}

impl<'a> JsonTokenizer<'a> {
    /// Creates a new tokenizer instance.
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.chars().peekable(),
            line: 1,
            column: 0,
        }
    }

    /// Returns the next token from the input stream.
    /// 
    /// # Errors
    /// 
    /// Returns `JsonFixerError` if an invalid token is encountered.
    pub fn next_token(&mut self) -> Result<Option<Token>, JsonFixerError> {
   
        if let Some(ch) = self.advance() {
            match ch {
                ch if ch.is_whitespace() => self.tokenize_whitespaces(ch).map(Some),
                '{' => {
                    Ok(Some(Token::LeftBrace(self.current_position())))
                },
                '}' => Ok(Some(Token::RightBrace(self.current_position()))),
                '[' => {
                    Ok(Some(Token::LeftBracket(self.current_position())))
                },
                ']' => {
                    Ok(Some(Token::RightBracket(self.current_position())))
                },
                ':' => {
                    Ok(Some(Token::Colon(self.current_position())))
                },
                ',' => {
                    Ok(Some(Token::Comma(self.current_position())))
                },
                '\'' | '"' => self.tokenize_string(ch).map(Some),
                '.'| '+' | '-' | '0'..='9' => self.tokenize_number(ch).map(Some),
                'a'..='z' | 'A'..='Z' | '_' => self.tokenize_identifier(ch).map(Some),
                ch => Err(JsonFixerError::Syntax(SyntaxError::UnexpectedCharacter(ch, Position {
                    line: self.line,
                    column: self.column,
                }))),
            }
        } else {
            Ok(None)
        }
    }
    
    fn tokenize_whitespaces(&mut self, first_space: char) -> Result<Token, JsonFixerError>{
        let start_pos = self.current_position();
        let mut whitespaces = String::new();
        whitespaces.push(first_space);

        while let Some(next_ch) = self.input.peek() {
            if !next_ch.is_whitespace() {
                break;
            }

            whitespaces.push(self.advance().unwrap());  
        }

        Ok(Token::Whitespace(whitespaces, start_pos))
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
                ch if ch == quote_char => return Ok(Token::String(result, start_pos)),          
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
                                    if let Some(h) = self.advance(){
                                        hex.push(h);
                                    }
                                }
                                if let Ok(code) = u32::from_str_radix(&hex, 16){
                                    if let Some(chr) = std::char::from_u32(code) {
                                        result.push(chr);
                                    }
                                }
                            }
                            _ => result.push(next_ch),
                        }
                    }
                }
                _=> result.push(ch),
            }
        }
        Err(JsonFixerError::Syntax(SyntaxError::UnmatchedQuotes(start_pos))) // placeholder
    }

    fn tokenize_number(&mut self, first_char: char) -> Result<Token, JsonFixerError>{
        let start_pos = self.current_position();
        let mut number = String::from(first_char);

        // Handle numbers that start with plus
        if first_char == '+'  || first_char == '.' {
            // If there is no digit after +, it's invalid
            if let Some(next_char) = self.peek() {
                if !next_char.is_digit(10) {
                    return Err(JsonFixerError::Syntax(SyntaxError::InvalidNumber(number, start_pos)));
                }
            } else {
                return Err(JsonFixerError::Syntax(SyntaxError::InvalidNumber(number, start_pos)));
            }

            if first_char == '+' { // Remove the +
                number.clear();
            }

            if first_char == '.' {// Add 0 before the . eg. .123 -> 0.123
                number.clear();
                number.push('0');
                number.push('.');
            }            
        }

        let mut multi_dots = false;
        while let Some(&ch) = self.peek() {
            if !ch.is_digit(10) && ch != '.' && ch != 'e' && ch != 'E' && ch != '+' && ch != '-' {
                break;
            }
            if first_char == '.' && ch == '.' { // Cannot accept two dots, a first dot already accepted
                multi_dots = true;
            }
            
            number.push(self.advance().unwrap());
        }

        // it's a number that includes many dots
        if multi_dots {
            return Err(JsonFixerError::Syntax(SyntaxError::InvalidNumber(number, start_pos)));
        }

        if number.chars().last().unwrap() == '.' { // remove the .
           number.pop();
        }

        Ok(Token::Number(number, self.current_position()))
    }

    fn tokenize_identifier(&mut self, first_char: char) -> Result<Token, JsonFixerError> {
        let start_pos = self.current_position();
        let mut ident = String::from(first_char);
        while let Some(&ch) = self.input.peek() {
            
            if !ch.is_alphanumeric() && ch != '_' {
                break;
            }
            
            ident.push(self.advance().unwrap());
        }

        match ident.as_str() {
            "true" => Ok(Token::Boolean(true, start_pos)),
            "false" => Ok(Token::Boolean(false, start_pos)),
            "null" => Ok(Token::Null(start_pos)),
            _ => {
                
                Ok(Token::UnquotedString(ident, start_pos))
            },
        }
    }
}
