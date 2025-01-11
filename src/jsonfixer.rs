//! A robust JSON parser and fixer that handles malformed JSON input.
//!
//! This module provides functionality to parse and fix JSON data that may be slightly malformed,
//! such as missing commas, extra commas, or unquoted identifiers. It attempts to produce valid
//! JSON output while maintaining the original data structure.

use crate::{JsonFixerConfig, JsonFixerError, JsonTokenizer, Token};
use std::fmt::Write;

/// Main struct for fixing potentially malformed JSON input.
///
/// # Examples
///
/// ```
/// use json_fixer::JsonFixer;
///
/// let input = r#"{ name: "John", age: 30, }"#;  // Note: unquoted keys
/// let mut fixer = JsonFixer::new(input);
/// let result = fixer.fix().unwrap();
/// assert_eq!(result, r#"{"name":"John","age":30}"#);
/// ```
pub struct JsonFixer<'a> {
    input: &'a str,
    config: JsonFixerConfig,
}

impl<'a> JsonFixer<'a> {
    /// Creates a new JsonFixer instance with the given input string.
    pub fn new(input: &'a str) -> Self {
        Self { input, config : JsonFixerConfig::default(), }
    }

    pub fn with_config(input: &'a str, config: JsonFixerConfig) -> Self {
        Self{input, config}
    }
    /// Attempts to parse and fix the JSON input, returning the corrected JSON string.
    ///
    /// # Errors
    ///
    /// Returns `JsonFixerError` if the input is too malformed to be fixed.
    pub fn fix(&mut self) -> Result<String, JsonFixerError> {
        let mut parser = JsonParser::new(&self.input, self.config.clone());
        parser.parse()
    }
}

/// Internal parser that handles the actual JSON parsing and fixing.
struct JsonParser<'a> {
    tokenizer: JsonTokenizer<'a>,
    current_token: Option<Token>,
    config : JsonFixerConfig,
    output: String,
    expect_value: bool,
}

impl<'a> JsonParser<'a> {
    /// Creates a new parser instance and advances to the first token.
    pub fn new(input: &'a str, config: JsonFixerConfig) -> Self {
        let mut parser = Self {
            tokenizer: JsonTokenizer::new(input),
            current_token: None,
            config: config,
            output: String::new(),
            expect_value: false,
        };

        let _ = parser.advance();
        parser
    }

    /// Advances to the next token in the input stream.
    fn advance(&mut self) -> Result<(), JsonFixerError> {
        loop {
            self.current_token = self.tokenizer.next_token()?;
            if let Some(Token::Whitespace(spaces,_ )) = &self.current_token {
                if self.config.preserve() {

                    self.output.push_str(&spaces);
                }
                continue;
            }
            break;
        }
        
        Ok(())
    }

    /// Parses the entire JSON input and returns the fixed JSON string.
    pub fn parse(&mut self) -> Result<String, JsonFixerError> {
        
        self.parse_value()?;
        Ok(std::mem::take(&mut self.output))
    }

    /// Parses a JSON value (object, array, string, number, boolean, or null).
    fn parse_value(&mut self,) -> Result<(), JsonFixerError> {
        match &self.current_token{
            
            Some(Token::LeftBrace(_)) => self.parse_object(),
            Some(Token::LeftBracket(_)) => self.parse_array(),
            Some(Token::String(s, _)) => {
                write!(self.output, "\"{}\"", s.replace('"', "\\\"")).unwrap();
                self.advance()?;
                Ok(())
            }
            Some(Token::Number(n, pos)) => {
                let _result: f64 = n
                    .parse()
                    .map_err(|_| JsonFixerError::InvalidNumber(n.clone(), pos.clone()))?;

                write!(self.output, "{}", n).unwrap();
                self.advance()?;
                Ok(())
            }
            Some(Token::Boolean(b, _)) => {
                write!(self.output, "{}", b).unwrap();
                self.advance()?;
                Ok(())
            }
            Some(Token::Null(_)) => {
                write!(self.output, "null").unwrap();
                self.advance()?;
                Ok(())
            }
            
            Some(Token::UnquotedString(s, pos)) => {

                if !self.expect_value {
                    write!(self.output, "\"{}\"", s).unwrap();
                    self.advance()?;
                    Ok(())
                }else {
                    Err(JsonFixerError::UnexpectedToken(
                        s.to_string(),
                        pos.clone(),
                    ))
                }
            },
            None => Err(JsonFixerError::UnexpectedEndOfInput(
                self.tokenizer.current_position(),
            )),
            
            // Should be reached
            Some(unexpect_token)=> Err(JsonFixerError::UnexpectedToken(
                unexpect_token.get(),
                unexpect_token.pos().clone(),
            )),
        }
    }

    fn add_comma_after_object_value(&mut self){

        let mut spaces = String::new();
            let last_ch: Option<(usize, char)> = self.output
                .chars()
                .rev()
                .enumerate()
                .find(|(_, c)| {
                    spaces.push(c.clone());
                    !c.is_whitespace()
                });
            
            if let Some((i, _c)) = last_ch {
                if spaces.contains('\n') {
                    self.output.insert(self.output.len() - i, ',');
                }else {
                    self.output.push(',');
                }
            }
    }
    fn remove_last_comma(&mut self){
        let last_ch = self.output.chars().rev().enumerate().find(|(_, c)| !c.is_whitespace());
        if let Some((i, c)) = last_ch {
            if c == ',' {
                self.output.remove(self.output.len() - 1 - i);
            }
        }
    }
    /// Parses a JSON object, handling potential formatting issues.
    /// Supports unquoted keys and trailing/multiple commas.
    fn parse_object(&mut self,) -> Result<(), JsonFixerError> {
        self.output.push('{');
        if self.config.space_between() {
            self.output.push(' ');
        }
        self.expect_value = false;

        self.advance()?; // Consume {

        while let Some(token) = &self.current_token {
            match token {
                Token::RightBrace(_) => break,
                Token::Comma(_) => {
                    self.expect_value = false;
                    // Consume consecutive commas (e.g., {,,})
                    self.advance()?;
                    continue;
                }
                _ => (),
            }

            if self.config.space_between(){
                self.output.push(' ');
            }

            // parse key
            match token {
                // Instead of key reached end '}'
                Token::RightBrace(_) => break,
                Token::String(key, _) => {
                    write!(self.output, "\"{}\"", key.replace('"', "\\\"")).unwrap()
                }
                Token::UnquotedString(key, _) => {
                    write!(self.output, "\"{}\"", key).unwrap()
                }
                _ => {
                    return Err(JsonFixerError::UnexpectedToken(
                        format!("\nExpected a 'Key' after '{}' but found {}", '{', token.get()),
                        token.pos().clone(),
                    ));
                }
            }

            self.advance()?; // Consume the key

            // Expect colon
            match &self.current_token {
                Some(Token::Colon(_)) => {
                    self.output.push(':');
                    self.advance()?; // Consume the : 
                }
                Some(unexped_token) => {
                    return Err(JsonFixerError::UnexpectedToken(
                        format!("\nExpected ':' after a 'key' but found {}", unexped_token.get()),
                        unexped_token.pos().clone(),
                    ));
                }
                None => {
                    // Unexpected end of the input
                    return Err(JsonFixerError::UnexpectedEndOfInput(
                        self.tokenizer.current_position(),
                    ));
                }
            }

            if self.config.space_between() || self.config.space_after_key() {
                self.output.push(' ');
            }

            self.expect_value = true;
            // Parse value
            self.parse_value()?;

            // Ensure comma after value
            self.add_comma_after_object_value();     
        }
        
        // Remove last comma if any
        self.remove_last_comma();

        self.expect_value = false;
        self.output.push('}');
        self.advance()?; // Consume }

        Ok(())
    }

    /// Parses a JSON array, handling trailing/multiple commas.
    fn parse_array(&mut self,) -> Result<(), JsonFixerError> {
        self.output.push('[');
        self.expect_value = true;
        if self.config.space_between() || self.config.space_after_key(){
            self.output.push(' ');
        }

        self.advance()?; // Consume [

        while let Some(token) = &self.current_token {
            match token {
                &Token::RightBracket(_) => break,
                &Token::Comma(_) => {
                    self.expect_value = false;
                    // Consume consecutive commas (e.g., [,,])
                    self.advance()?;
                    continue;
                }
                _ => (),
            }

            self.expect_value = true;
            self.parse_value()?;

            self.output.push(',');
            if self.config.space_between() || self.config.space_after_key() {
                self.output.push(' ');
            } 
        }
        // Remove last comma if any
        self.remove_last_comma();

        self.expect_value = false;
        self.output.push(']');
        self.advance()?; // Consume ]

        Ok(())
    }
}


/*
************************** Gated behind serde *************************
*/

#[cfg(feature = "serde")]
impl <'a>JsonFixer<'a> {
    pub fn to_value<T: serde::Serialize>(&mut self, value: &T) -> Result<String, JsonFixerError>{
        let json_string = serde_json::to_string(value)
            .map_err(|e| JsonFixerError::SerdeError(e.to_string()))?;

        self.input = json_string.as_str();
        self.fix()
    }

    pub fn from_str<T: for<'de> serde::Deserialize<'de>>(&self) -> Result<T, JsonFixerError>{
        serde_json::from_str(self.input).map_err(|e| JsonFixerError::SerdeError(e.to_string()))?;
    }

    pub fn from_fixed<T:>(&mut self, value: &T) -> Result<String, JsonFixerError>{
        let fixed = self.fix()?;
        serde_json::from_str(fixed)
            .map_err(|e| JsonFixerError::SerdeError(e.to_string()))
    }
}