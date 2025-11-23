//! Object operations

use crate::Result;

/// Object with metadata
#[derive(Debug, Clone)]
pub struct Object {
    pub key: String,
    pub body: Vec<u8>,
    pub content_type: String,
}

impl Object {
    pub fn new(key: String, body: Vec<u8>, content_type: String) -> Self {
        Self {
            key,
            body,
            content_type,
        }
    }
}
