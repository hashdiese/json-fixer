use crate::json_tokenizer::{JsonTokenizer, Token};
use crate::jsonfixer_error::JsonFixerError;
use std::fmt::Write;

pub struct JsonFixer<'a> {
    input: &'a str,
}

impl<'a> JsonFixer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input }
    }
    pub fn fix(&mut self) -> Result<String, JsonFixerError> {
        let mut parser = JsonParser::new(&self.input);
        parser.parse()
    }
}

struct JsonParser<'a> {
    tokenizer: JsonTokenizer<'a>,
    current_token: Option<Token>,
}

impl<'a> JsonParser<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut parser = Self {
            tokenizer: JsonTokenizer::new(input),
            current_token: None,
        };

        let _ = parser.advance();

        parser
    }

    fn advance(&mut self) -> Result<(), JsonFixerError> {
        self.current_token = self.tokenizer.next_token()?;
        Ok(())
    }

    pub fn parse(&mut self) -> Result<String, JsonFixerError> {
        let mut result = String::new();
        self.parse_value(&mut result)?;
        Ok(result)
    }

    fn parse_value(&mut self, output: &mut String) -> Result<(), JsonFixerError> {
        match &self.current_token {
            Some(Token::LeftBrace) => self.parse_object(output),
            Some(Token::LeftBracket) => self.parse_array(output),
            Some(Token::String(s)) => {
                write!(output, "\"{}\"", s.replace('"', "\\\"")).unwrap();
                self.advance()?;
                Ok(())
            }
            Some(Token::Number(n)) => {
                write!(output, "{}", n).unwrap();
                self.advance()?;
                Ok(())
            }
            Some(Token::Boolean(b)) => {
                write!(output, "{}", b).unwrap();
                self.advance()?;
                Ok(())
            }
            Some(Token::Null) => {
                write!(output, "null").unwrap();
                self.advance()?;
                Ok(())
            }
            None => Err(JsonFixerError::UnexpectedEndOfInput(
                self.tokenizer.current_position(),
            )),
            Some(token) => Err(JsonFixerError::UnexpectedToken(
                token.get(),
                self.tokenizer.current_position(),
            )),
        }
    }
    fn parse_object(&mut self, output: &mut String) -> Result<(), JsonFixerError> {
        output.push('{');
        self.advance()?; // Consume {

        let mut first_round = true;
        while let Some(token) = &self.current_token {
            match token {
                &Token::RightBrace => break,
                &Token::Comma => {
                    // If token is , before value eg. {,1: 1,key: 2,name: "ali",,, }
                    // Consume the ,
                    self.advance()?;
                    continue;
                }
                _ => (),
            }

            if !first_round {
                output.push(',');
            }

            first_round = false;

            // parse key
            match token {
                Token::String(key) => write!(output, "\"{}\"", key.replace('"', "\\\"")).unwrap(),

                _ => {
                    return Err(JsonFixerError::UnexpectedToken(
                        token.get(),
                        self.tokenizer.current_position(),
                    ));
                }
            }

            self.advance()?; // Consume the key

            // Expect colon
            match &self.current_token {
                Some(Token::Colon) => {
                    output.push(':');
                    self.advance()?; // Consume the : 
                }
                Some(unexped_token) => {
                    return Err(JsonFixerError::UnexpectedToken(
                        format!("\nExpected ':' after key but found {}", unexped_token.get()),
                        self.tokenizer.current_position(),
                    ));
                }
                None => {
                    // Unexpected end of the input
                    return Err(JsonFixerError::UnexpectedEndOfInput(
                        self.tokenizer.current_position(),
                    ));
                }
            }

            // Parse value
            self.parse_value(output)?;
        }

        output.push('}');
        self.advance()?; // Consume }

        Ok(())
    }

    fn parse_array(&mut self, output: &mut String) -> Result<(), JsonFixerError> {
        output.push('[');
        self.advance()?; // Consume [

        let mut first_round = true;
        while let Some(token) = &self.current_token {
            match token {
                &Token::RightBracket => break,
                &Token::Comma => {
                    // If token is , before value eg. [,1,2,3,4,,, ]
                    // Consume the ,
                    self.advance()?;
                    continue;
                }
                _ => (),
            }

            if !first_round {
                output.push(',');
            }
            first_round = false;

            self.parse_value(output)?;
        }

        output.push(']');
        self.advance()?; // Consume ]

        Ok(())
    }
}
