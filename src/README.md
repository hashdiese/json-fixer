# JSON Fixer
A robust Rust library for parsing and fixing malformed JSON. This library helps you handle common JSON formatting issues while maintaining the original data structure.

## Features
- Fixes common JSON formatting issues:
  - Unquoted object keys
  - Missing commas
  - Trailing commas
  - Single-quoted strings
  - Simple syntax errors
  - Closes last } or ]
- Detailed error reporting with line and column information
- Support for all JSON data types
- Proper handling of escape sequences
- No external dependencies

## Installation
Add this to your `Cargo.toml`:
```toml
[dependencies]
json-fixer = "0.1.0"
```

## Usage
```rust
use json_fixer::JsonFixer;

fn main() -> Result<(), Box> {
    // Example of fixing malformed JSON
    let input = r#"{
        name: 'John',
        age: 30,
        hobbies: ['reading', 'coding',],
    }"#;
    let mut fixer = JsonFixer::new(input);
    let fixed_json = fixer.fix()?;
    println!("Fixed JSON: {}", fixed_json);
    
    Ok(())
}
```

## Error Handling
The library provides detailed error information through the `JsonFixerError` enum:
```rust
pub enum JsonFixerError {
    UnexpectedCharacter(char, Position),
    UnmatchedQuotes(Position),
    UnexpectedEndOfInput(Position),
    MissingComma(Position),
    InvalidNumber(String, Position),
    UnexpectedToken(String, Position)
}
```
Each error includes position information (line and column) to help locate the issue in the input text.

## Examples

### Fixing Unquoted Keys
```rust
let input = r#"{ name: "John", age: 30 }"#;
let fixed = JsonFixer::new(input).fix()?;
// Result: {"name":"John","age":30}
```

### Handling Trailing Commas
```rust
let input = r#"[ 1, 2, 3, ]"#;
let fixed = JsonFixer::new(input).fix()?;
// Result: [1,2,3]
```

### Converting Single Quotes to Double Quotes
```rust
let input = r#"{ 'name': 'John' }"#;
let fixed = JsonFixer::new(input).fix()?;
// Result: {"name":"John"}
```

### Auto-closing Brackets and Braces
```rust
let input = r#"{ "numbers": [1, 2, 3"#;  // Missing closing bracket and brace
let fixed = JsonFixer::new(input).fix()?;
// Result: {"numbers":[1,2,3]}
```

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing
Contributions are welcome! Please feel free to submit a Pull Request.

## TODOs and Future Improvements
- [ ] Convert JSON to Type
- [ ] Type into JSON
- [ ] Add pretty printing option
- [ ] Support for JSON5 features
- [ ] Streaming input support
- [ ] Custom error recovery strategies
- [ ] Performance optimizations

## Documentation
Full documentation is available at [docs.rs](https://docs.rs/json-fixer).