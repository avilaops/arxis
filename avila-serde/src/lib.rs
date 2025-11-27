//! Avila Serde - AVL Platform serialization
//! Replacement for serde/serde_json - 100% Rust std
//! Simple JSON serialization/deserialization

use std::collections::HashMap;
use std::fmt;

/// JSON value type
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

impl Value {
    /// Parse JSON string to Value
    pub fn from_json(json: &str) -> Result<Self, Error> {
        let mut parser = Parser::new(json);
        parser.parse()
    }

    /// Convert Value to JSON string
    pub fn to_json(&self) -> String {
        match self {
            Value::Null => "null".to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Number(n) => {
                if n.is_finite() {
                    format!("{}", n)
                } else {
                    "null".to_string()
                }
            }
            Value::String(s) => format!("\"{}\"", escape_string(s)),
            Value::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|v| v.to_json()).collect();
                format!("[{}]", items.join(","))
            }
            Value::Object(obj) => {
                let pairs: Vec<String> = obj
                    .iter()
                    .map(|(k, v)| format!("\"{}\":{}", escape_string(k), v.to_json()))
                    .collect();
                format!("{{{}}}", pairs.join(","))
            }
        }
    }

    /// Pretty print JSON
    pub fn to_json_pretty(&self) -> String {
        self.to_json_pretty_indent(0)
    }

    fn to_json_pretty_indent(&self, indent: usize) -> String {
        let ind = "  ".repeat(indent);
        let ind_next = "  ".repeat(indent + 1);

        match self {
            Value::Array(arr) if arr.is_empty() => "[]".to_string(),
            Value::Array(arr) => {
                let items: Vec<String> = arr
                    .iter()
                    .map(|v| format!("{}{}", ind_next, v.to_json_pretty_indent(indent + 1)))
                    .collect();
                format!("[\n{}\n{}]", items.join(",\n"), ind)
            }
            Value::Object(obj) if obj.is_empty() => "{}".to_string(),
            Value::Object(obj) => {
                let pairs: Vec<String> = obj
                    .iter()
                    .map(|(k, v)| {
                        format!(
                            "{}\"{}\": {}",
                            ind_next,
                            escape_string(k),
                            v.to_json_pretty_indent(indent + 1)
                        )
                    })
                    .collect();
                format!("{{\n{}\n{}}}", pairs.join(",\n"), ind)
            }
            _ => self.to_json(),
        }
    }

    /// Get as string
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    /// Get as number
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::Number(n) => Some(*n),
            _ => None,
        }
    }

    /// Get as bool
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// Get as array
    pub fn as_array(&self) -> Option<&Vec<Value>> {
        match self {
            Value::Array(arr) => Some(arr),
            _ => None,
        }
    }

    /// Get as object
    pub fn as_object(&self) -> Option<&HashMap<String, Value>> {
        match self {
            Value::Object(obj) => Some(obj),
            _ => None,
        }
    }
}

fn escape_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('\"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

struct Parser {
    chars: Vec<char>,
    pos: usize,
}

impl Parser {
    fn new(input: &str) -> Self {
        Self {
            chars: input.chars().collect(),
            pos: 0,
        }
    }

    fn parse(&mut self) -> Result<Value, Error> {
        self.skip_whitespace();
        self.parse_value()
    }

    fn parse_value(&mut self) -> Result<Value, Error> {
        self.skip_whitespace();

        if self.pos >= self.chars.len() {
            return Err(Error::UnexpectedEnd);
        }

        match self.chars[self.pos] {
            'n' => self.parse_null(),
            't' | 'f' => self.parse_bool(),
            '\"' => self.parse_string(),
            '[' => self.parse_array(),
            '{' => self.parse_object(),
            '-' | '0'..='9' => self.parse_number(),
            _ => Err(Error::InvalidChar(self.chars[self.pos])),
        }
    }

    fn parse_null(&mut self) -> Result<Value, Error> {
        if self.consume_word("null") {
            Ok(Value::Null)
        } else {
            Err(Error::InvalidLiteral)
        }
    }

    fn parse_bool(&mut self) -> Result<Value, Error> {
        if self.consume_word("true") {
            Ok(Value::Bool(true))
        } else if self.consume_word("false") {
            Ok(Value::Bool(false))
        } else {
            Err(Error::InvalidLiteral)
        }
    }

    fn parse_string(&mut self) -> Result<Value, Error> {
        self.consume_char('\"')?;
        let mut s = String::new();

        while self.pos < self.chars.len() && self.chars[self.pos] != '\"' {
            if self.chars[self.pos] == '\\' {
                self.pos += 1;
                if self.pos >= self.chars.len() {
                    return Err(Error::UnexpectedEnd);
                }
                match self.chars[self.pos] {
                    'n' => s.push('\n'),
                    't' => s.push('\t'),
                    'r' => s.push('\r'),
                    '\"' => s.push('\"'),
                    '\\' => s.push('\\'),
                    _ => s.push(self.chars[self.pos]),
                }
            } else {
                s.push(self.chars[self.pos]);
            }
            self.pos += 1;
        }

        self.consume_char('\"')?;
        Ok(Value::String(s))
    }

    fn parse_number(&mut self) -> Result<Value, Error> {
        let start = self.pos;

        if self.chars[self.pos] == '-' {
            self.pos += 1;
        }

        while self.pos < self.chars.len() && (self.chars[self.pos].is_numeric() || self.chars[self.pos] == '.') {
            self.pos += 1;
        }

        let num_str: String = self.chars[start..self.pos].iter().collect();
        num_str
            .parse::<f64>()
            .map(Value::Number)
            .map_err(|_| Error::InvalidNumber)
    }

    fn parse_array(&mut self) -> Result<Value, Error> {
        self.consume_char('[')?;
        let mut arr = Vec::new();

        self.skip_whitespace();
        if self.pos < self.chars.len() && self.chars[self.pos] == ']' {
            self.pos += 1;
            return Ok(Value::Array(arr));
        }

        loop {
            arr.push(self.parse_value()?);
            self.skip_whitespace();

            if self.pos >= self.chars.len() {
                return Err(Error::UnexpectedEnd);
            }

            if self.chars[self.pos] == ']' {
                self.pos += 1;
                break;
            }

            self.consume_char(',')?;
        }

        Ok(Value::Array(arr))
    }

    fn parse_object(&mut self) -> Result<Value, Error> {
        self.consume_char('{')?;
        let mut obj = HashMap::new();

        self.skip_whitespace();
        if self.pos < self.chars.len() && self.chars[self.pos] == '}' {
            self.pos += 1;
            return Ok(Value::Object(obj));
        }

        loop {
            self.skip_whitespace();
            let key = match self.parse_string()? {
                Value::String(s) => s,
                _ => return Err(Error::ExpectedString),
            };

            self.skip_whitespace();
            self.consume_char(':')?;

            let value = self.parse_value()?;
            obj.insert(key, value);

            self.skip_whitespace();
            if self.pos >= self.chars.len() {
                return Err(Error::UnexpectedEnd);
            }

            if self.chars[self.pos] == '}' {
                self.pos += 1;
                break;
            }

            self.consume_char(',')?;
        }

        Ok(Value::Object(obj))
    }

    fn consume_char(&mut self, expected: char) -> Result<(), Error> {
        if self.pos >= self.chars.len() {
            return Err(Error::UnexpectedEnd);
        }
        if self.chars[self.pos] != expected {
            return Err(Error::ExpectedChar(expected));
        }
        self.pos += 1;
        Ok(())
    }

    fn consume_word(&mut self, word: &str) -> bool {
        let end = self.pos + word.len();
        if end > self.chars.len() {
            return false;
        }

        let slice: String = self.chars[self.pos..end].iter().collect();
        if slice == word {
            self.pos = end;
            true
        } else {
            false
        }
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.chars.len() && self.chars[self.pos].is_whitespace() {
            self.pos += 1;
        }
    }
}

#[derive(Debug, Clone)]
pub enum Error {
    UnexpectedEnd,
    InvalidChar(char),
    InvalidLiteral,
    InvalidNumber,
    ExpectedChar(char),
    ExpectedString,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::UnexpectedEnd => write!(f, "Unexpected end of input"),
            Error::InvalidChar(c) => write!(f, "Invalid character: {}", c),
            Error::InvalidLiteral => write!(f, "Invalid literal"),
            Error::InvalidNumber => write!(f, "Invalid number"),
            Error::ExpectedChar(c) => write!(f, "Expected character: {}", c),
            Error::ExpectedString => write!(f, "Expected string"),
        }
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_string() {
        let v = Value::from_json("\"hello\"").unwrap();
        assert_eq!(v, Value::String("hello".to_string()));
    }

    #[test]
    fn test_parse_number() {
        let v = Value::from_json("42").unwrap();
        assert_eq!(v, Value::Number(42.0));
    }

    #[test]
    fn test_parse_array() {
        let v = Value::from_json("[1,2,3]").unwrap();
        if let Value::Array(arr) = v {
            assert_eq!(arr.len(), 3);
        } else {
            panic!("Expected array");
        }
    }

    #[test]
    fn test_parse_object() {
        let v = Value::from_json(r#"{"name":"test","value":42}"#).unwrap();
        if let Value::Object(obj) = v {
            assert_eq!(obj.get("name").and_then(|v| v.as_str()), Some("test"));
            assert_eq!(obj.get("value").and_then(|v| v.as_f64()), Some(42.0));
        } else {
            panic!("Expected object");
        }
    }

    #[test]
    fn test_to_json() {
        let v = Value::String("hello".to_string());
        assert_eq!(v.to_json(), "\"hello\"");
    }
}
