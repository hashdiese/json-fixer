pub mod json_tokenizer;
pub mod jsonparser;
pub mod jsonfixer_error;
pub mod jsonfixer_config;
pub mod jsonformatter;

pub use json_tokenizer::{JsonTokenizer, Token};
pub use jsonparser::JsonParser;
pub use jsonfixer_error::JsonFixerError;
pub use jsonfixer_config::JsonFixerConfig;
pub use jsonformatter::JsonFormatter;




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
        Self { input, config : JsonFixerConfig::default(),}
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

    pub fn fixed_space_between(&mut self) -> Result<String, JsonFixerError> {
        self.config.space_between = true;
        self.config.beautify = false;
        self.config.preserve = false;
        let mut parser = JsonParser::new(&self.input, self.config.clone());
        parser.parse()
    }
    pub fn fixed_pretty(&mut self) -> Result<String, JsonFixerError> {
        self.config.beautify = true;
        self.config.preserve = false;
        self.config.space_between = false;
        
        let mut parser = JsonParser::new(&self.input, self.config.clone());
        parser.parse()
    }
}

/*
************************** Gated behind serde *************************
*/

//#[cfg(feature = "serde")]
impl <'a>JsonFixer<'a> {
    pub fn to_value<T: serde::Serialize>(&mut self, value: &T) -> Result<String, JsonFixerError>{
        let serde_output = serde_json::to_string(value)
            .map_err(|e| JsonFixerError::SerdeError(e.to_string()))?;

        let mut parser = JsonParser::new(&serde_output, self.config.clone());
        parser.parse()
    }

    pub fn from_str<T: for<'de> serde::Deserialize<'de>>(&self) -> Result<T, JsonFixerError>{
        serde_json::from_str::<T>(self.input)
        .map_err(|e| JsonFixerError::SerdeError(e.to_string()))
    }

    pub fn from_fixed<T:>(&mut self) -> Result<String, JsonFixerError>{
        let mut parser = JsonParser::new(self.input, self.config.clone());
        let fixed = parser.parse()?;
        serde_json::from_str(&fixed)
            .map_err(|e| JsonFixerError::SerdeError(e.to_string()))
    }
}


