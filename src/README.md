# JSON Fixer
A robust Rust library for parsing and fixing malformed JSON. This library helps you handle common JSON formatting issues while maintaining the original data structure.

## Features
- Fixes common JSON formatting issues:
  - Unquoted object keys
  - Missing commas in objects and arrays
  - Trailing commas
  - Single-quoted strings
  - Simple syntax errors
  - Closes unclosed brackets and braces
- Formatting options:
  - Pretty printing with customizable indentation
  - Space between keys and values
  - Preserve original formatting
  - Sort object keys alphabetically
- Detailed error reporting with line and column information
- Support for all JSON data types
- Proper handling of escape sequences
- Serde integration for type conversion (optional feature)
- No external dependencies (unless using serde features)

## Installation
Add this to your `Cargo.toml`:
```toml
[dependencies]
json-fixer = "0.1.0"  # Basic functionality
# Or with serde support:
json-fixer = { version = "0.1.0", features = ["serde"] }
```

## Usage

### Basic JSON Fixing
```rust
use json_fixer::JsonFixer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example of fixing malformed JSON
    let input = r#"{
        name: 'John'
        age: 30
        hobbies: ['reading' 'coding']
    }"#;
    
    // Using default configuration
    let fixed_json = JsonFixer::fix(input)?;
    println!("Fixed JSON: {}", fixed_json);
    // Result: {"name":"John","age":30,"hobbies":["reading","coding"]}
    
    Ok(())
}
```

### Formatting Options
```rust
use json_fixer::{JsonFixer, JsonFixerConfig};

// Pretty printing
let pretty_json = JsonFixer::fix_pretty(input)?;
// Result:
// {
//     "name": "John",
//     "age": 30,
//     "hobbies": [
//         "reading",
//         "coding"
//     ]
// }

// Add spaces between keys and values
let spaced_json = JsonFixer::fix_with_space_between(input)?;
// Result: { "name": "John", "age": 30, "hobbies": ["reading", "coding"] }

// Custom configuration
let mut config = JsonFixerConfig::default();
config.sort_keys = true;
config.indent_size = 2;
let custom_json = JsonFixer::fix_with_config(input, config)?;
```

### Serde Integration
When enabled with the `serde` feature, you can convert between JSON and Rust types:

```rust
use json_fixer::JsonFixer;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct Person {
    name: String,
    age: u32,
    hobbies: Vec<String>,
}

// Parse malformed JSON directly into a type
let input = r#"{ name: 'John', age: 30, hobbies: ['reading' 'coding'] }"#;
let person: Person = JsonFixer::from_fixed(input, None)?;

// Convert a type to properly formatted JSON
let json_string = JsonFixer::to_json(&person, None)?;

// Parse valid JSON into a type
let valid_json = r#"{"name":"John","age":30,"hobbies":["reading","coding"]}"#;
let person: Person = JsonFixer::from_str(valid_json)?;
```

## Error Handling
The library provides detailed error information through the `JsonFixerError` enum:

```rust
pub enum JsonFixerError {
    Syntax(SyntaxError),
    Format(JsonFormatError),
    IO(std::fmt::Error),
    #[cfg(feature = "serde")]
    SerdeError(String),
}

pub enum SyntaxError {
    UnexpectedCharacter(char, Position),
    UnmatchedQuotes(Position),
    UnexpectedEndOfInput(Position),
    MissingComma(Position),
    InvalidNumber(String, Position),
    UnexpectedToken(String, Position),
}

pub enum JsonFormatError {
    LineTooLong {
        line: usize,
        length: usize,
        max: usize,
    },
    InvalidIndentation {
        line: usize,
    },
}
```

## Examples

### Fixing Missing Commas
```rust
// In arrays
let input = r#"[1 2 3 4]"#;
let fixed = JsonFixer::fix(input)?;
// Result: [1,2,3,4]

// In objects
let input = r#"{
    name: "Hicham-dine"
    age: 36
    job: "programmer"
}"#;
let fixed = JsonFixer::fix(input)?;
// Result: {"name":"Hicham-dine","age":36,"job":"programmer"}
```

### Sorting Object Keys
```rust
let input = r#"{
    c: 3,
    a: 1,
    b: 2
}"#;
let mut config = JsonFixerConfig::default();
config.sort_keys = true;
let fixed = JsonFixer::fix_with_config(input, config)?;
// Result: {"a":1,"b":2,"c":3}
```

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing
Contributions are welcome! Please feel free to submit a Pull Request.

## TODOs and Future Improvements
- [ ] Full Support for JSON5 features
- [ ] Streaming input support
- [ ] Performance optimizations

## Documentation
Full documentation is available at [docs.rs](https://docs.rs/json-fixer).