//! A robust JSON parser and fixer that handles malformed JSON input.
//!
//! This module provides functionality to parse and fix JSON data that may be slightly malformed,
//! such as missing commas, extra commas, or unquoted identifiers. It attempts to produce valid
//! JSON output while maintaining the original data structure.

use std::fmt::Write;

use super::{
    json_tokenizer::{JsonTokenizer, Token},
    jsonfixer_config::JsonFixerConfig,
    jsonfixer_error::{JsonFixerError, SyntaxError},
    jsonformatter::{Formatter, JsonFormatter},
};

#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
    Null,
    Boolean(bool),
    Number(String),
    String(String),
    Array(Vec<JsonEntryValue>),
    Object(Vec<JsonEntryValue>),
    Space(String),
}

/*
************************** JsonParser *************************
*/

#[derive(Debug, Clone, PartialEq)]
pub struct JsonEntryValue {
    pub space_bf_key: Option<String>,
    pub key: Option<String>,
    pub space_af_key: Option<String>,
    pub space_bf_val: Option<String>,
    pub value: Option<JsonValue>,
    pub space_af_val: Option<String>,
}

impl JsonEntryValue {
    fn new() -> Self {
        Self {
            space_bf_key: None,
            key: None,
            space_af_key: None,
            space_bf_val: None,
            value: None,
            space_af_val: None,
        }
    }

    pub fn get_sp_bf_key(&self) -> String {
        let sp = self.space_bf_key.clone();
        sp.unwrap_or_default()
    }
    pub fn get_key(&self) -> String {
        let key = self.key.clone();
        key.unwrap_or_default()
    }
    pub fn get_sp_af_key(&self) -> String {
        let sp = self.space_af_key.clone();
        sp.unwrap_or_default()
    }

    pub fn get_value(&self) -> JsonValue {
        let val = self.value.clone();
        val.unwrap()
    }
    pub fn get_sp_bf_val(&self) -> String {
        let sp = self.space_bf_val.clone();
        sp.unwrap_or_default()
    }
    pub fn get_sp_af_val(&self) -> String {
        let sp = self.space_af_val.clone();
        sp.unwrap_or_default()
    }
}

/// Internal parser that handles the actual JSON parsing and fixing.
pub struct JsonParser<'a> {
    tokenizer: JsonTokenizer<'a>,
    current_token: Option<Token>,
    config: JsonFixerConfig,
}

impl<'a> JsonParser<'a> {
    /// Creates a new parser instance and advances to the first token.
    pub fn new(input: &'a str, config: JsonFixerConfig) -> Self {
        let mut parser = Self {
            tokenizer: JsonTokenizer::new(input),
            current_token: None,
            config: config,
        };

        let _ = parser.advance();
        parser
    }

    /// Advances to the next token in the input stream.
    fn advance(&mut self) -> Result<(), JsonFixerError> {
        self.current_token = self.tokenizer.next_token()?;

        Ok(())
    }

    /// Parses the entire JSON input and returns the fixed JSON string.
    pub fn parse(&mut self) -> Result<String, JsonFixerError> {
        let mut output = String::new();
        // Input can be whitespace-value-whitespace
        // Handle white space if any
        if let Some(Token::Whitespace(_sp, _)) = &self.current_token {
            // Ignore spaces before an actual value
            self.advance()?; // Consume spaces
        }
        let config = self.config.clone();
        // Handle JsonValue
        let value = self.parse_value()?;
        self.advance()?; // Consume value

        // Format the output
        let formetter = JsonFormatter;
        write!(output, "{}", formetter.format(&value, &config)?)
            .map_err(|err| JsonFixerError::IO(err))?;

        loop {
            match &self.current_token {
                Some(Token::Whitespace(_sp, _)) => {
                    // Ignore spaces before an actual value
                    self.advance()?; // Consume spaces
                    continue;
                }
                Some(token) => {
                    // Error if there is anything else after a value was found
                    return Err(JsonFixerError::Syntax(SyntaxError::UnexpectedToken(
                        format!("\nExpected  EOF but found {}", token.get()),
                        token.pos().clone(),
                    )));
                }
                None => break, // EOF
            }
        }

        Ok(output)
    }

    /// Parses a JSON value (object, array, string, number, boolean, or null).
    fn parse_value(&mut self) -> Result<JsonValue, JsonFixerError> {
        match &self.current_token {
            Some(Token::LeftBrace(_)) => self.parse_object(),
            Some(Token::LeftBracket(_)) => self.parse_array(),
            Some(Token::String(s, _)) => Ok(JsonValue::String(s.replace('"', "\\\""))),
            Some(Token::Number(n, pos)) => {
                let _result: f64 = n.parse().map_err(|_| {
                    JsonFixerError::Syntax(SyntaxError::InvalidNumber(n.clone(), pos.clone()))
                })?;

                Ok(JsonValue::Number(n.to_string()))
            }
            Some(Token::Boolean(b, _)) => Ok(JsonValue::Boolean(*b)),
            Some(Token::Null(_)) => Ok(JsonValue::Null),

            Some(Token::UnquotedString(s, pos)) => {
                //println!("Here....");
                Err(JsonFixerError::Syntax(SyntaxError::UnexpectedToken(
                    s.to_string(),
                    pos.clone(),
                )))
            }
            None => Err(JsonFixerError::Syntax(SyntaxError::UnexpectedEndOfInput(
                self.tokenizer.current_position(),
            ))),

            // Should be reached
            Some(unexpect_token) => {
                //println!("There....");
                Err(JsonFixerError::Syntax(SyntaxError::UnexpectedToken(
                    unexpect_token.get(),
                    unexpect_token.pos().clone(),
                )))
            }
        }
    }

    /// Parses a JSON object, handling potential formatting issues.
    /// Supports unquoted keys and trailing/multiple commas.
    fn parse_object(&mut self) -> Result<JsonValue, JsonFixerError> {
        let mut obj = Vec::new();
        self.advance()?; // Consume {

        //let go_next_token = true;
        while !self.current_token.is_none() {
            let mut entry = JsonEntryValue::new();
            //println!("Obj: {:?}", obj);
            //println!("Current_token: {:?}", &self.current_token);

            match &self.current_token {
                Some(Token::RightBrace(_)) => break,
                Some(Token::Comma(_)) => {
                    // Empty entry
                    // Consume consecutive commas (e.g., {,,})
                    self.advance()?;
                    continue;
                }
                Some(Token::Whitespace(sp, _)) => {
                    // Consume spaces before 'Key' if any
                    entry.space_bf_key = Some(sp.to_string());
                    self.advance()?;
                }
                _ => (),
            }

            // parse key
            match &self.current_token {
                Some(Token::RightBrace(_)) => {
                    // Empty object with inside spaces eg. {   }
                    entry.value = None;
                    obj.push(entry);
                    break;
                }
                Some(Token::Comma(_)) => {
                    // Empty entry
                    // Consume consecutive commas (e.g., {,,})
                    entry.value = None;
                    obj.push(entry);
                    self.advance()?;
                    continue;
                }
                Some(Token::String(k, _)) | Some(Token::UnquotedString(k, _)) => {
                    entry.key = Some(k.to_string());

                    self.advance()?; // Consume the key
                }
                token => {
                    if let Some(t) = &token {
                        return Err(JsonFixerError::Syntax(SyntaxError::UnexpectedToken(
                            format!("\nExpected a 'Key' after '{}' but found {}", '{', t.get()),
                            t.pos().clone(),
                        )));
                    } else {
                        // Reach the EOF with no closing } a no key
                        // Empty object with inside spaces and not closed eg. {
                        entry.value = None;
                        obj.push(entry);
                        break;
                    }
                }
            }

            // Consume spaces before ':' if any
            if let Some(Token::Whitespace(sp, _)) = &self.current_token {
                entry.space_af_key = Some(sp.to_string());
                self.advance()?;
            }

            // Expect colon
            match &self.current_token {
                Some(Token::Colon(_)) => {
                    self.advance()?; // Consume the : 
                }
                Some(unexped_token) => {
                    return Err(JsonFixerError::Syntax(SyntaxError::UnexpectedToken(
                        format!(
                            "\nExpected ':' after a 'key' but found {}",
                            unexped_token.get()
                        ),
                        unexped_token.pos().clone(),
                    )));
                }
                None => {
                    // Unexpected end of the input
                    return Err(JsonFixerError::Syntax(SyntaxError::UnexpectedEndOfInput(
                        self.tokenizer.current_position(),
                    )));
                }
            }

            // Consume spaces before Value if any
            if let Some(Token::Whitespace(sp, _)) = &self.current_token {
                entry.space_bf_val = Some(sp.to_string());
                self.advance()?;
            }

            // Parse value
            entry.value = Some(self.parse_value()?);

            // Consume spaces After Value if any
            if let Some(Token::Whitespace(sp, _)) = &self.current_token {
                entry.space_af_val = Some(sp.to_string());
                self.advance()?;
            }

            self.advance()?;
            // Push the entry
            obj.push(entry);
        }

        self.advance()?; // Consume }
        Ok(JsonValue::Object(obj))
    }

    /// Parses a JSON array, handling trailing/multiple commas.
    fn parse_array(&mut self) -> Result<JsonValue, JsonFixerError> {
        let mut arr = Vec::new();
        self.advance()?; // Consume [

        while !self.current_token.is_none() {
            let mut entry = JsonEntryValue::new();

            match &self.current_token {
                Some(Token::RightBracket(_)) => break, // Empty array without spaces
                Some(Token::Comma(_)) => {
                    // Consume consecutive commas (e.g., [,,])
                    self.advance()?;
                    continue;
                }
                Some(Token::Whitespace(sp, _)) => {
                    // Consume spaces
                    entry.space_bf_val = Some(sp.to_string());
                    self.advance()?;
                }
                _ => (),
            }

            match &self.current_token {
                Some(Token::RightBracket(_)) => {
                    // Empty array with spaces inside it
                    entry.value = None;
                    arr.push(entry);
                    break;
                }
                Some(Token::Comma(_)) => {
                    // Empty array with spaces inside it and commas
                    // Consume consecutive commas (e.g., [,,])
                    entry.value = None;
                    arr.push(entry);
                    self.advance()?;
                    continue;
                }
                _ => {
                    //println!("current_token : {:?}", self.current_token);
                    // Get the value
                    let curr_t = self.current_token.clone();
                    entry.value = Some(self.parse_value()?);

                    // Primitive value needs to be consumed after parse value
                    if curr_t == self.current_token {
                        self.advance()?;
                    }

                    // Consume spaces After Value if any
                    if let Some(Token::Whitespace(sp, _)) = &self.current_token {
                        entry.space_af_val = Some(sp.to_string());
                        self.advance()?;
                    }

                    arr.push(entry);
                }
            }
        }

        self.advance()?; // Consume ]

        Ok(JsonValue::Array(arr))
    }
}
