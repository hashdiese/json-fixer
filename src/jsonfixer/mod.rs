pub mod json_tokenizer;
pub mod jsonfixer_config;
pub mod jsonfixer_error;
pub mod jsonformatter;
pub mod jsonparser;

pub use json_tokenizer::{JsonTokenizer, Token};
pub use jsonfixer_config::JsonFixerConfig;
pub use jsonfixer_error::JsonFixerError;
pub use jsonformatter::JsonFormatter;
pub use jsonparser::JsonParser;

/// A utility for parsing and fixing malformed JSON input.
///
/// This struct provides static methods to handle various JSON formatting and parsing tasks:
/// - Fix common JSON syntax errors
/// - Apply different formatting styles
/// - Convert between JSON and Rust types (with serde feature)
///
/// # Features
///
/// - Fix malformed JSON with missing quotes, commas, and brackets
/// - Multiple formatting options including pretty printing and key sorting
/// - Serde integration for type conversion (optional)
/// 
/// # Examples
///
/// Basic JSON fixing:
/// ```
/// use json_fixer::JsonFixer;
///
/// let input = r#"{ name: "John", age: 30, }"#;  // Note: unquoted keys and trailing comma
/// let result = JsonFixer::fix(input).unwrap();
/// assert_eq!(result, r#"{"name":"John","age":30}"#);
/// ```
///
/// Pretty printing:
/// ```
/// use json_fixer::JsonFixer;
///
/// let input = r#"{name:"John",age:30}"#;
/// let result = JsonFixer::fix_pretty(input).unwrap();
/// // Result:
/// // {
/// //     "name": "John",
/// //     "age": 30
/// // }
/// ```
pub struct JsonFixer;

impl JsonFixer {
    /// Fixes JSON input using custom configuration options.
    ///
    /// This method allows full control over the fixing and formatting process through
    /// the provided configuration.
    ///
    /// # Arguments
    ///
    /// * `input` - The JSON string to fix
    /// * `config` - Configuration options for fixing and formatting
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - The fixed JSON string
    /// * `Err(JsonFixerError)` - If the input is too malformed to be fixed
    ///
    /// # Examples
    ///
    /// ```
    /// use json_fixer::{JsonFixer, JsonFixerConfig};
    ///
    /// let input = r#"{
    ///     c: 3,
    ///     a: 1,
    ///     b: 2
    /// }"#;
    ///
    /// let mut config = JsonFixerConfig::default();
    /// config.sort_keys = true;
    /// config.beautify = true;
    ///
    /// let result = JsonFixer::fix_with_config(input, config).unwrap();
    /// ```
    pub fn fix_with_config(input: &str, config: JsonFixerConfig) -> Result<String, JsonFixerError> {
        let mut parser = JsonParser::new(input, config);
        parser.parse()
    }
    /// Fixes malformed JSON using default configuration.
    ///
    /// This method attempts to fix common JSON syntax errors while maintaining
    /// a compact output format.
    ///
    /// # Arguments
    ///
    /// * `input` - The JSON string to fix
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - The fixed JSON string
    /// * `Err(JsonFixerError)` - If the input is too malformed to be fixed
    ///
    /// # Examples
    ///
    /// ```
    /// use json_fixer::JsonFixer;
    ///
    /// let input = r#"{ name: 'John', age: 30 hobbies: ['reading' 'coding'] }"#;
    /// let result = JsonFixer::fix(input).unwrap();
    /// assert_eq!(result, r#"{"name":"John","age":30,"hobbies":["reading","coding"]}"#);
    /// ```
    pub fn fix(input: &str) -> Result<String, JsonFixerError> {
        let mut parser = JsonParser::new(input, JsonFixerConfig::default());
        parser.parse()
    }
    /// Fixes JSON and adds spaces between keys, values, and punctuation.
    ///
    /// This method applies minimal formatting to make the JSON more readable
    /// while keeping it on a single line.
    ///
    /// # Arguments
    ///
    /// * `input` - The JSON string to fix
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - The fixed JSON string with added spacing
    /// * `Err(JsonFixerError)` - If the input is too malformed to be fixed
    ///
    /// # Examples
    ///
    /// ```
    /// use json_fixer::JsonFixer;
    ///
    /// let input = r#"{name:"John",age:30}"#;
    /// let result = JsonFixer::fix_with_space_between(input).unwrap();
    /// assert_eq!(result, r#"{ "name": "John", "age": 30 }"#);
    /// ```
    pub fn fix_with_space_between(input: &str) -> Result<String, JsonFixerError> {
        let mut config = JsonFixerConfig::default();

        config.space_between = true;
        config.beautify = false;
        config.preserve = false;
        let mut parser = JsonParser::new(input, config);
        parser.parse()
    }
     /// Fixes JSON and applies pretty printing with proper indentation.
    ///
    /// This method formats the JSON to be human-readable with proper indentation
    /// and line breaks.
    ///
    /// # Arguments
    ///
    /// * `input` - The JSON string to fix
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - The fixed and formatted JSON string
    /// * `Err(JsonFixerError)` - If the input is too malformed to be fixed
    ///
    /// # Examples
    ///
    /// ```
    /// use json_fixer::JsonFixer;
    ///
    /// let input = r#"{name:"John",age:30,hobbies:["reading","coding"]}"#;
    /// let result = JsonFixer::fix_pretty(input).unwrap();
    /// // Result will be:
    /// // {
    /// //     "name": "John",
    /// //     "age": 30,
    /// //     "hobbies": [
    /// //         "reading",
    /// //         "coding"
    /// //     ]
    /// // }
    /// ```
    pub fn fix_pretty(input: &str) -> Result<String, JsonFixerError> {
        let mut config = JsonFixerConfig::default();
        config.beautify = true;
        config.preserve = false;
        config.space_between = false;

        let mut parser = JsonParser::new(input, config);
        parser.parse()
    }
}

/*
************************** Gated behind serde *************************
*/


#[cfg(feature = "serde")]
impl JsonFixer {
    /// Converts a Rust type to a JSON string with optional formatting.
    ///
    /// This method is only available when the `serde` feature is enabled.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type to serialize, must implement `serde::Serialize`
    ///
    /// # Arguments
    ///
    /// * `value` - The value to convert to JSON
    /// * `config` - Optional configuration for JSON formatting
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - The JSON string representation
    /// * `Err(JsonFixerError)` - If serialization fails
    ///
    /// # Examples
    ///
    /// ```
    /// use json_fixer::JsonFixer;
    /// use serde::Serialize;
    ///
    /// #[derive(Serialize)]
    /// struct Person {
    ///     name: String,
    ///     age: u32,
    /// }
    ///
    /// let person = Person {
    ///     name: "John".to_string(),
    ///     age: 30,
    /// };
    ///
    /// let json = JsonFixer::to_json(&person, None).unwrap();
    /// ```
    pub fn to_json<T: serde::Serialize>(
        value: &T,
        config: Option<JsonFixerConfig>,
    ) -> Result<String, JsonFixerError> {
        let serde_output =
            serde_json::to_string(value).map_err(|e| JsonFixerError::SerdeError(e.to_string()))?;

        let mut parser = JsonParser::new(&serde_output, config.unwrap_or_default());
        parser.parse()
    }

    /// Parses a JSON string into a Rust type without fixing.
    ///
    /// This method is only available when the `serde` feature is enabled.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type to deserialize into, must implement `serde::Deserialize`
    ///
    /// # Arguments
    ///
    /// * `input` - The JSON string to parse
    ///
    /// # Returns
    ///
    /// * `Ok(T)` - The deserialized value
    /// * `Err(JsonFixerError)` - If parsing fails
    ///
    /// # Examples
    ///
    /// ```
    /// use json_fixer::JsonFixer;
    /// use serde::Deserialize;
    ///
    /// #[derive(Deserialize)]
    /// struct Person {
    ///     name: String,
    ///     age: u32,
    /// }
    ///
    /// let json = r#"{"name":"John","age":30}"#;
    /// let person: Person = JsonFixer::from_str(json).unwrap();
    /// ```
    pub fn from_str<T: for<'de> serde::Deserialize<'de>>(input: &str) -> Result<T, JsonFixerError> {
        serde_json::from_str::<T>(input).map_err(|e| JsonFixerError::SerdeError(e.to_string()))
    }

    /// Fixes malformed JSON and then parses it into a Rust type.
    ///
    /// This method is only available when the `serde` feature is enabled.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type to deserialize into, must implement `serde::Deserialize`
    ///
    /// # Arguments
    ///
    /// * `input` - The potentially malformed JSON string to fix and parse
    /// * `config` - Optional configuration for JSON fixing
    ///
    /// # Returns
    ///
    /// * `Ok(T)` - The deserialized value
    /// * `Err(JsonFixerError)` - If fixing or parsing fails
    ///
    /// # Examples
    ///
    /// ```
    /// use json_fixer::JsonFixer;
    /// use serde::Deserialize;
    ///
    /// #[derive(Deserialize)]
    /// struct Person {
    ///     name: String,
    ///     age: u32,
    /// }
    ///
    /// let json = r#"{ name: "John", age: 30 }"#;  // Note: unquoted keys
    /// let person: Person = JsonFixer::from_fixed(json, None).unwrap();
    /// ```
    pub fn from_fixed<T: for<'de> serde::Deserialize<'de>>(
        input: &str,
        config: Option<JsonFixerConfig>,
    ) -> Result<T, JsonFixerError> {
        let mut parser = JsonParser::new(input, config.unwrap_or_default());
        let fixed = parser.parse()?;
        serde_json::from_str(&fixed).map_err(|e| JsonFixerError::SerdeError(e.to_string()))
    }
}
