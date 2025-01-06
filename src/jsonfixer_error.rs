use std::fmt::{self};

use crate::json_tokenizer::Position;
pub enum JsonFixerError {
    UnexpectedCharacter(char, Position),
    UnmatchedQuotes(Position),
    UnexpectedEndOfInput(Position),
    MissingComma(Position),
    InvalidNumber(String, Position),
    UnexpectedToken(String, Position),
}

impl fmt::Display for JsonFixerError {
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
