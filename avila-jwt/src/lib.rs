//! # avila-jwt - JSON Web Tokens
extern crate alloc;
use alloc::string::String;

pub struct Jwt {
    pub header: String,
    pub payload: String,
    pub signature: String,
}

impl Jwt {
    pub fn new(payload: String) -> Self {
        Self {
            header: "eyJ0eXAiOiJKV1QifQ".into(),
            payload,
            signature: String::new(),
        }
    }
    
    pub fn encode(&self) -> String {
        format!("{}.{}.{}", self.header, self.payload, self.signature)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_jwt() {
        let jwt = Jwt::new("payload".into());
        let encoded = jwt.encode();
        assert!(encoded.contains("payload"));
    }
}
