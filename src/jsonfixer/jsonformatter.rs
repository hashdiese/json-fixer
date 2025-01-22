use std::fmt::Write;

use super::{jsonparser::JsonEntryValue, JsonFixerConfig, JsonFixerError, jsonparser::JsonValue};

#[derive(Debug, Clone)]
pub enum IndentStyle {
    Spaces,
    Tabs,
}

impl IndentStyle {
    fn with_size(&self, size: Option<usize>) -> String {
        match self {
            Self::Spaces => " ".repeat(size.unwrap_or(0)),
            Self::Tabs => "\t".to_string(),
        }
    }
}

pub trait Formatter {
    fn format(&self, value: &JsonValue, config: &JsonFixerConfig)
    -> Result<String, JsonFixerError>;
}

pub struct JsonFormatter;

impl Formatter for JsonFormatter {
    fn format(
        &self,
        value: &JsonValue,
        config: &JsonFixerConfig,
    ) -> Result<String, JsonFixerError> {
        let mut output = String::new();
        self.format_value(value, &mut output, 0, config)?;
        Ok(output)
    }
}

impl JsonFormatter {
    fn format_value(
        &self,
        value: &JsonValue,
        output: &mut String,
        depth: usize,
        config: &JsonFixerConfig,
    ) -> Result<(), JsonFixerError> {
        match value {
            JsonValue::Null => output.push_str("null"),
            JsonValue::Boolean(b) => output.push_str(if *b { "true" } else { "false" }),
            JsonValue::Number(n) => write!(output, "{}", n).map_err(|e| JsonFixerError::IO(e))?,
            JsonValue::String(s) => {
                output.push('"');
                //self.escaped_string(output, &s.replace('"', "\\\""))?;
                output.push_str(s);
                output.push('"');
            }
            JsonValue::Array(arr) => {
                if config.preserve() {
                    self.format_array_preserved(arr, output, depth, config)?;
                } else {
                    self.format_array(arr, output, depth, config)?;
                }
            }
            JsonValue::Object(obj) => {
                if config.preserve() {
                    self.format_object_preserved(obj, output, depth, config)?;
                } else {
                    self.format_object(obj, output, depth, config)?;
                }
            }
            JsonValue::Space(sp) => write!(output, "{}", sp).map_err(|e| JsonFixerError::IO(e))?,
        }
        Ok(())
    }
    fn _escaped_string(&self, output: &mut String, s: &str) -> Result<(), JsonFixerError> {
        for c in s.chars() {
            match c {
                '"' => output.push_str("\\\""),
                '\\' => output.push_str("\\\\"),
                '\n' => output.push_str("\\n"),
                '\r' => output.push_str("\\r"),
                '\t' => output.push_str("\\t"),
                '\u{0008}' => output.push_str("\\b"),
                '\u{000C}' => output.push_str("\\f"),
                c if c.is_control() => {
                    write!(output, "\\u{:04x}", c as u32).map_err(|e| JsonFixerError::IO(e))?
                }
                c => output.push(c),
            }
        }
        Ok(())
    }

    fn write_newline(
        &self,
        output: &mut String,
        _depth: usize,
        _config: &JsonFixerConfig,
    ) -> Result<(), JsonFixerError> {
        output.push('\n');
        Ok(())
    }

    fn write_indent(
        &self,
        output: &mut String,
        depth: usize,
        config: &JsonFixerConfig,
    ) -> Result<(), JsonFixerError> {
        let indent = config.indent_style.with_size(Some(config.indent_size));

        for _ in 0..depth {
            output.push_str(&indent);
        }

        Ok(())
    }

    fn format_array(
        &self,
        arr: &[JsonEntryValue],
        output: &mut String,
        depth: usize,
        config: &JsonFixerConfig,
    ) -> Result<(), JsonFixerError> {
        if arr.is_empty() {
            output.push_str("[]");
            return Ok(());
        }

        output.push('[');
        if config.beautify() {
            self.write_newline(output, depth + 1, config)?;
        }
        if config.space_between() {
            output.push(' ');
        }

        for (i, entry) in arr.iter().enumerate() {
            if entry.value.is_some() {
                if i > 0 {
                    output.push(',');
                    if config.beautify() {
                        self.write_newline(output, depth + 1, config)?;
                    }
                    if config.space_between() {
                        output.push(' ');
                    }
                }
                if config.beautify() {
                    self.write_indent(output, depth + 1, config)?;
                }
                self.format_value(&entry.get_value(), output, depth + 1, config)?;
            }
        }
        if config.beautify() {
            self.write_newline(output, depth, config)?;
            self.write_indent(output, depth, config)?;
        }
        if config.space_between() {
            output.push(' ');
        }

        output.push(']');
        Ok(())
    }

    fn format_array_preserved(
        &self,
        arr: &[JsonEntryValue],
        output: &mut String,
        depth: usize,
        config: &JsonFixerConfig,
    ) -> Result<(), JsonFixerError> {
        if arr.is_empty() {
            output.push_str("[]");
            return Ok(());
        }

        output.push('[');

        for (i, entry) in arr.iter().enumerate() {
            if i > 0 && entry.value.is_some() {
                output.push(',');
            }

            output.push_str(&entry.get_sp_bf_val());

            if entry.value.is_some() {
                self.format_value(&entry.get_value(), output, depth + 1, config)?;
            }
            output.push_str(&entry.get_sp_af_val());
        }

        output.push(']');
        Ok(())
    }

    fn format_object(
        &self,
        obj: &Vec<JsonEntryValue>,
        output: &mut String,
        depth: usize,
        config: &JsonFixerConfig,
    ) -> Result<(), JsonFixerError> {
        let mut entries = obj.to_vec();
        entries.retain(|val| val.value.is_some());

        if entries.is_empty() {
            output.push_str("{}");
            return Ok(());
        }

        output.push('{');
        if config.beautify() {
            self.write_newline(output, depth + 1, config)?;
        }

        if config.sort_keys {
            entries.sort_by(|a, b| a.key.cmp(&b.key));
        }

        if config.space_between() {
            output.push(' ');
        }

        for (i, entry) in entries.iter().enumerate() {
            if i > 0 {
                output.push(',');
                if config.beautify() {
                    self.write_newline(output, depth + 1, config)?;
                }
                if config.space_between() {
                    output.push(' ');
                }
            }

            if config.beautify() {
                self.write_indent(output, depth + 1, config)?;
            }

            output.push('"');
            //self.escaped_string(output, &entry.clone().key.unwrap())?;
            output.push_str(&entry.get_key());
            output.push('"');

            output.push(':');

            if config.space_between() || config.beautify() {
                output.push(' ');
            }

            self.format_value(&entry.get_value(), output, depth + 1, config)?;
        }

        if config.beautify() {
            self.write_newline(output, depth, config)?;
            self.write_indent(output, depth, config)?;
        }

        if config.space_between() {
            output.push(' ');
        }

        output.push('}');

        Ok(())
    }

    fn format_object_preserved(
        &self,
        obj: &Vec<JsonEntryValue>,
        output: &mut String,
        depth: usize,
        config: &JsonFixerConfig,
    ) -> Result<(), JsonFixerError> {
        let entries = self.clean_middle_spaces_and_sort(&obj, config);
        if entries.is_empty() {
            output.push_str("{}");
            return Ok(());
        }

        output.push('{');

        for (_i, entry) in entries.iter().enumerate() {
            //println!("Entry {i}: {:?}", entry);
            if entry.value.is_none() {
                output.push_str(&entry.get_sp_bf_key());
                output.push_str(&entry.get_sp_af_key());

                continue;
            } else {
                output.push_str(&entry.get_sp_bf_key());

                output.push('"');
                output.push_str(&entry.get_key());
                //self.escaped_string(output, &entry.clone().key.unwrap())?;
                output.push('"');

                output.push_str(&entry.get_sp_af_key());

                output.push(':');

                output.push_str(&entry.get_sp_bf_val());

                self.format_value(&entry.get_value(), output, depth + 1, config)?;
                let last_space = entry.get_sp_af_val();

                if last_space.contains('\n') {
                    output.push(',');
                    output.push_str(&last_space);
                } else {
                    output.push_str(&last_space);
                    output.push(',');
                }
            }
        }

        let found = output
            .chars()
            .rev()
            .enumerate()
            .find(|(_i, ch)| !ch.is_whitespace());
        if found.is_some() {
            let (i, ch) = found.unwrap();
            if ch == ',' {
                output.remove(output.len() - i - 1);
            }
        }

        output.push('}');

        Ok(())
    }

    fn clean_middle_spaces_and_sort(
        &self,
        obj: &Vec<JsonEntryValue>,
        config: &JsonFixerConfig,
    ) -> Vec<JsonEntryValue> {
        // Keep first and last whitespaces
        let first_whitespaces = obj.first();
        let mut last_whitespaces: Option<&JsonEntryValue> = None;
        if obj.len() > 1 {
            last_whitespaces = obj.last();
        }

        // Remove all wihtespaces
        let mut cleaned_obj = obj.to_vec();
        cleaned_obj.retain(|entry| entry.value.is_some());

        // Sort the cleaned obj entries
        if config.sort_keys {
            cleaned_obj.sort_by(|a, b| {
                let key_a = a.get_key();
                let key_b = b.get_key();
                key_a.cmp(&key_b)
            });
        }

        if let Some(entry) = first_whitespaces {
            if entry.value.is_none() {
                cleaned_obj.insert(0, entry.clone());
            }
        }

        if let Some(entry) = last_whitespaces {
            if entry.value.is_none() {
                cleaned_obj.push(entry.clone());
            }
        }

        cleaned_obj
    }
}
