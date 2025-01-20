use std::fmt::{self};

use crate::json_tokenizer::Position;
/// Errors that may occur while fixing a malformed JSON.
#[derive(Debug,)]
pub enum JsonFixerError {
    Syntax(SyntaxError),
    Format(JsonFormatError),
    IO(std::fmt::Error),
    /// Serde error
    //#[cfg( feature = "serde")]
    SerdeError(String),
}

#[derive(Debug)]
pub enum JsonFormatError {
    LineTooLong {line: usize, length: usize, max: usize},
    InvalidIndentation {line: usize},
}
impl fmt::Display for JsonFormatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LineTooLong{line, length, max} => write!(f, "Line {} is too long of length: {} \nExpeced max length: {}", line, length, max),
            Self::InvalidIndentation{line} => write!(f, "Invalid Indentation at line: {}", line),
        }
    }
}


impl std::error::Error for JsonFixerError{}

impl fmt::Display for JsonFixerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Syntax(err) => write!(f, "Syntax error: {}", err),
            Self::Format(err) => write!(f, "Format error: {}", err),
            Self::IO(err) => write!(f, "IO error: {}", err),
            //#[cfg(feature = "serde")]
            Self::SerdeError(err) => write!(f, "Serde error: {}", err),
        }
    }
}


#[derive(Debug)]
pub enum SyntaxError {
    /// Unexpected character found in input.
    UnexpectedCharacter(char, Position),
    /// Unmatched quotes in a string.
    UnmatchedQuotes(Position),
    /// Input ended unexpectedly.
    UnexpectedEndOfInput(Position),
    /// Comma is missing between JSON elements.
    MissingComma(Position),
    /// Invalid number format encountered.
    InvalidNumber(String, Position),
    /// Unexpected token in the input.
    UnexpectedToken(String, Position),
}

impl fmt::Display for SyntaxError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedToken(token, pos) => write!(
                f,
                "Unexpected Token : '{}' at line {}, column {}",
                token, pos.line, pos.column
            ),
            Self::UnexpectedCharacter(ch, pos) => write!(
                f,
                "Unexpected character '{}' at line {}, column {}",
                ch, pos.line, pos.column
            ),
            Self::UnmatchedQuotes(pos) => write!(
                f,
                "Unmatched quotes at line {}, column {}",
                pos.line, pos.column
            ),
            Self::UnexpectedEndOfInput(pos) => write!(
                f,
                "Unexpected end of input at line {}, column {}",
                pos.line, pos.column
            ),
            Self::MissingComma(pos) => write!(
                f,
                "Missing comma at line {}, column {}",
                pos.line, pos.column
            ),
            Self::InvalidNumber(ch, pos) => write!(
                f,
                "Invalid number '{}' at line {}, column {}",
                ch, pos.line, pos.column
            ),
        }
    }
}
