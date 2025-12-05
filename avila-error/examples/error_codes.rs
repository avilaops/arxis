//! Exemplo de error codes e tipos
//!
//! Demonstra o sistema de códigos de erro e categorização

#![no_std]

extern crate alloc;
use alloc::string::{String, ToString};

use avila_error::{AvilaError, ErrorKind};

fn demonstrate_error_codes() {
    #[cfg(feature = "alloc")]
    {
        extern crate std;
        use std::println;
        
        println!("=== Error Codes and Categories ===\n");
        
        // Demonstrar que cada erro tem um código único
        let err1 = AvilaError::invalid_input("test".into());
        let err2 = AvilaError::not_found("test".into());
        let err3 = AvilaError::internal("test".into());
        
        println!("InvalidInput error code: {}", err1.code());
        println!("NotFound error code: {}", err2.code());
        println!("Internal error code: {}", err3.code());
        
        println!("\n=== Error Kind Conversion ===\n");
        
        // Demonstrar conversão de u8 para ErrorKind
        for i in 1..=16 {
            let kind = ErrorKind::from_u8(i);
            println!("Code {}: {} - {}", i, kind.as_str(), kind.default_message());
        }
        
        println!("\n=== Custom Error Codes ===\n");
        
        // Criar erros com códigos customizados
        let custom1 = AvilaError::with_message(2001, ErrorKind::Internal, "Database connection lost".into());
        let custom2 = AvilaError::with_message(3001, ErrorKind::Timeout, "Request took too long".into());
        let custom3 = AvilaError::with_message(4001, ErrorKind::DataCorruption, "Checksum mismatch".into());
        
        println!("Custom error 1: [{}:{}] {}", 
                 custom1.code(), 
                 custom1.kind().as_str(), 
                 custom1.message().unwrap_or(""));
        println!("Custom error 2: [{}:{}] {}", 
                 custom2.code(), 
                 custom2.kind().as_str(), 
                 custom2.message().unwrap_or(""));
        println!("Custom error 3: [{}:{}] {}", 
                 custom3.code(), 
                 custom3.kind().as_str(), 
                 custom3.message().unwrap_or(""));
    }
}

fn main() {
    demonstrate_error_codes();
}
