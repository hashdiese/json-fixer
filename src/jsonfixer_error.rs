use std::fmt::{self};

use crate::json_tokenizer::Position;
/// Errors that may occur while fixing a malformed JSON.
#[derive(Debug)]
pub enum JsonFixerError {
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

    /// Serde error
    #[cfg( feature = "serde")]
    SerdeError(String),
}

impl fmt::Display for JsonFixerError{
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
