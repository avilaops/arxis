//! # avila-websocket
extern crate alloc;
use alloc::vec::Vec;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OpCode { Text = 1, Binary = 2, Close = 8, Ping = 9, Pong = 10 }

pub struct Frame {
    pub opcode: OpCode,
    pub payload: Vec<u8>,
}

impl Frame {
    pub fn new(opcode: OpCode, payload: Vec<u8>) -> Self { Self { opcode, payload } }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_frame() { let f = Frame::new(OpCode::Text, vec![1,2,3]); assert_eq!(f.opcode, OpCode::Text); }
}
