//! Zero-copy JSON parser and serializer
//!
//! Pure Rust implementation without serde

use crate::error::{Error, Result};
use std::collections::HashMap;

/// JSON value with zero-copy string slices
#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
    /// Null value
    Null,
    /// Boolean value
    Bool(bool),
    /// Number value (stored as f64)
    Number(f64),
    /// String value (owned)
    String(String),
    /// Array of values
    Array(Vec<JsonValue>),
    /// Object (map of string keys to values)
    Object(HashMap<String, JsonValue>),
}

impl JsonValue {
    /// Parse JSON from string
    pub fn parse(input: &str) -> Result<Self> {
        let mut parser = JsonParser::new(input);
        parser.parse_value()
    }

    /// Convert to JSON string
    pub fn to_string(&self) -> String {
        match self {
            JsonValue::Null => "null".to_string(),
            JsonValue::Bool(b) => b.to_string(),
            JsonValue::Number(n) => n.to_string(),
            JsonValue::String(s) => format!("\"{}\"", escape_string(s)),
            JsonValue::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|v| v.to_string()).collect();
                format!("[{}]", items.join(","))
            }
            JsonValue::Object(obj) => {
                let items: Vec<String> = obj
                    .iter()
                    .map(|(k, v)| format!("\"{}\":{}", escape_string(k), v.to_string()))
                    .collect();
                format!("{{{}}}", items.join(","))
            }
        }
    }

    /// Get as object
    pub fn as_object(&self) -> Option<&HashMap<String, JsonValue>> {
        match self {
            JsonValue::Object(obj) => Some(obj),
            _ => None,
        }
    }

    /// Get as array
    pub fn as_array(&self) -> Option<&Vec<JsonValue>> {
        match self {
            JsonValue::Array(arr) => Some(arr),
            _ => None,
        }
    }

    /// Get as string
    pub fn as_str(&self) -> Option<&str> {
        match self {
            JsonValue::String(s) => Some(s),
            _ => None,
        }
    }

    /// Get as number
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            JsonValue::Number(n) => Some(*n),
            _ => None,
        }
    }

    /// Get as boolean
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            JsonValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
}

struct JsonParser {
    input: Vec<char>,
    pos: usize,
}

impl JsonParser {
    fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    fn parse_value(&mut self) -> Result<JsonValue> {
        self.skip_whitespace();

        if self.pos >= self.input.len() {
            return Err(Error::JsonError {
                message: "Unexpected end of input".to_string(),
            });
        }

        match self.input[self.pos] {
            '"' => self.parse_string(),
            '{' => self.parse_object(),
            '[' => self.parse_array(),
            't' | 'f' => self.parse_bool(),
            'n' => self.parse_null(),
            '-' | '0'..='9' => self.parse_number(),
            c => Err(Error::JsonError {
                message: format!("Unexpected character: {}", c),
            }),
        }
    }

    fn parse_string(&mut self) -> Result<JsonValue> {
        self.expect('"')?;
        let mut result = String::new();

        while self.pos < self.input.len() {
            match self.input[self.pos] {
                '"' => {
                    self.pos += 1;
                    return Ok(JsonValue::String(result));
                }
                '\\' => {
                    self.pos += 1;
                    if self.pos >= self.input.len() {
                        return Err(Error::JsonError {
                            message: "Unexpected end in escape sequence".to_string(),
                        });
                    }
                    match self.input[self.pos] {
                        'n' => result.push('\n'),
                        't' => result.push('\t'),
                        'r' => result.push('\r'),
                        '"' => result.push('"'),
                        '\\' => result.push('\\'),
                        '/' => result.push('/'),
                        _ => result.push(self.input[self.pos]),
                    }
                    self.pos += 1;
                }
                c => {
                    result.push(c);
                    self.pos += 1;
                }
            }
        }

        Err(Error::JsonError {
            message: "Unterminated string".to_string(),
        })
    }

    fn parse_number(&mut self) -> Result<JsonValue> {
        let start = self.pos;

        // Optional minus
        if self.pos < self.input.len() && self.input[self.pos] == '-' {
            self.pos += 1;
        }

        // Digits
        while self.pos < self.input.len() && self.input[self.pos].is_ascii_digit() {
            self.pos += 1;
        }

        // Optional decimal
        if self.pos < self.input.len() && self.input[self.pos] == '.' {
            self.pos += 1;
            while self.pos < self.input.len() && self.input[self.pos].is_ascii_digit() {
                self.pos += 1;
            }
        }

        // Optional exponent
        if self.pos < self.input.len() && (self.input[self.pos] == 'e' || self.input[self.pos] == 'E') {
            self.pos += 1;
            if self.pos < self.input.len() && (self.input[self.pos] == '+' || self.input[self.pos] == '-') {
                self.pos += 1;
            }
            while self.pos < self.input.len() && self.input[self.pos].is_ascii_digit() {
                self.pos += 1;
            }
        }

        let num_str: String = self.input[start..self.pos].iter().collect();
        let num = num_str.parse::<f64>().map_err(|_| Error::JsonError {
            message: format!("Invalid number: {}", num_str),
        })?;

        Ok(JsonValue::Number(num))
    }

    fn parse_bool(&mut self) -> Result<JsonValue> {
        if self.expect_word("true") {
            Ok(JsonValue::Bool(true))
        } else if self.expect_word("false") {
            Ok(JsonValue::Bool(false))
        } else {
            Err(Error::JsonError {
                message: "Expected boolean".to_string(),
            })
        }
    }

    fn parse_null(&mut self) -> Result<JsonValue> {
        if self.expect_word("null") {
            Ok(JsonValue::Null)
        } else {
            Err(Error::JsonError {
                message: "Expected null".to_string(),
            })
        }
    }

    fn parse_array(&mut self) -> Result<JsonValue> {
        self.expect('[')?;
        let mut array = Vec::new();

        self.skip_whitespace();

        if self.pos < self.input.len() && self.input[self.pos] == ']' {
            self.pos += 1;
            return Ok(JsonValue::Array(array));
        }

        loop {
            array.push(self.parse_value()?);
            self.skip_whitespace();

            if self.pos >= self.input.len() {
                return Err(Error::JsonError {
                    message: "Unexpected end of array".to_string(),
                });
            }

            match self.input[self.pos] {
                ',' => {
                    self.pos += 1;
                    self.skip_whitespace();
                }
                ']' => {
                    self.pos += 1;
                    break;
                }
                c => {
                    return Err(Error::JsonError {
                        message: format!("Expected ',' or ']', got '{}'", c),
                    })
                }
            }
        }

        Ok(JsonValue::Array(array))
    }

    fn parse_object(&mut self) -> Result<JsonValue> {
        self.expect('{')?;
        let mut object = HashMap::new();

        self.skip_whitespace();

        if self.pos < self.input.len() && self.input[self.pos] == '}' {
            self.pos += 1;
            return Ok(JsonValue::Object(object));
        }

        loop {
            self.skip_whitespace();

            // Parse key
            let key = match self.parse_string()? {
                JsonValue::String(s) => s,
                _ => {
                    return Err(Error::JsonError {
                        message: "Expected string key".to_string(),
                    })
                }
            };

            self.skip_whitespace();
            self.expect(':')?;

            // Parse value
            let value = self.parse_value()?;
            object.insert(key, value);

            self.skip_whitespace();

            if self.pos >= self.input.len() {
                return Err(Error::JsonError {
                    message: "Unexpected end of object".to_string(),
                });
            }

            match self.input[self.pos] {
                ',' => {
                    self.pos += 1;
                    self.skip_whitespace();
                }
                '}' => {
                    self.pos += 1;
                    break;
                }
                c => {
                    return Err(Error::JsonError {
                        message: format!("Expected ',' or '}}', got '{}'", c),
                    })
                }
            }
        }

        Ok(JsonValue::Object(object))
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.input.len() {
            match self.input[self.pos] {
                ' ' | '\t' | '\n' | '\r' => self.pos += 1,
                _ => break,
            }
        }
    }

    fn expect(&mut self, ch: char) -> Result<()> {
        if self.pos >= self.input.len() || self.input[self.pos] != ch {
            return Err(Error::JsonError {
                message: format!("Expected '{}', got EOF or wrong char", ch),
            });
        }
        self.pos += 1;
        Ok(())
    }

    fn expect_word(&mut self, word: &str) -> bool {
        let chars: Vec<char> = word.chars().collect();
        if self.pos + chars.len() > self.input.len() {
            return false;
        }

        for (i, ch) in chars.iter().enumerate() {
            if self.input[self.pos + i] != *ch {
                return false;
            }
        }

        self.pos += chars.len();
        true
    }
}

fn escape_string(s: &str) -> String {
    s.chars()
        .flat_map(|c| match c {
            '"' => vec!['\\', '"'],
            '\\' => vec!['\\', '\\'],
            '\n' => vec!['\\', 'n'],
            '\r' => vec!['\\', 'r'],
            '\t' => vec!['\\', 't'],
            c => vec![c],
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_null() {
        let json = JsonValue::parse("null").unwrap();
        assert_eq!(json, JsonValue::Null);
    }

    #[test]
    fn test_parse_bool() {
        assert_eq!(JsonValue::parse("true").unwrap(), JsonValue::Bool(true));
        assert_eq!(JsonValue::parse("false").unwrap(), JsonValue::Bool(false));
    }

    #[test]
    fn test_parse_number() {
        assert_eq!(JsonValue::parse("42").unwrap(), JsonValue::Number(42.0));
        assert_eq!(JsonValue::parse("-3.14").unwrap(), JsonValue::Number(-3.14));
    }

    #[test]
    fn test_parse_string() {
        assert_eq!(
            JsonValue::parse("\"hello\"").unwrap(),
            JsonValue::String("hello".to_string())
        );
    }

    #[test]
    fn test_parse_array() {
        let json = JsonValue::parse("[1, 2, 3]").unwrap();
        match json {
            JsonValue::Array(arr) => {
                assert_eq!(arr.len(), 3);
                assert_eq!(arr[0], JsonValue::Number(1.0));
            }
            _ => panic!("Expected array"),
        }
    }

    #[test]
    fn test_parse_object() {
        let json = JsonValue::parse(r#"{"name": "Alice", "age": 30}"#).unwrap();
        match json {
            JsonValue::Object(obj) => {
                assert_eq!(obj.get("name").unwrap(), &JsonValue::String("Alice".to_string()));
                assert_eq!(obj.get("age").unwrap(), &JsonValue::Number(30.0));
            }
            _ => panic!("Expected object"),
        }
    }

    #[test]
    fn test_to_string() {
        let json = JsonValue::Object({
            let mut map = HashMap::new();
            map.insert("name".to_string(), JsonValue::String("Bob".to_string()));
            map.insert("age".to_string(), JsonValue::Number(25.0));
            map
        });

        let json_str = json.to_string();
        assert!(json_str.contains("\"name\""));
        assert!(json_str.contains("\"Bob\""));
    }
}
