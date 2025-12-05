//! Exemplo básico de uso do avila-error
//!
//! Demonstra o sistema de erros unificado do Avila

#![no_std]

extern crate alloc;
use alloc::string::{String, ToString};

use avila_error::{AvilaError, ErrorKind, Result, bail, ensure};

fn validate_input(value: i32) -> Result<i32> {
    // Usando ensure! para validação
    ensure!(
        value >= 0,
        AvilaError::invalid_input("Value must be non-negative".into())
    );
    
    ensure!(
        value <= 100,
        AvilaError::invalid_input("Value must be <= 100".into())
    );
    
    Ok(value)
}

fn process_data(value: i32) -> Result<i32> {
    // Validar entrada
    let validated = validate_input(value)?;
    
    // Processar
    if validated == 42 {
        // Usando bail! para retornar erro antecipadamente
        bail!(AvilaError::internal("Cannot process the answer to everything".into()));
    }
    
    Ok(validated * 2)
}

fn example_with_context() -> Result<()> {
    let result = process_data(150)
        .map_err(|e| e.add_context("In example_with_context function".into()))?;
    
    Ok(())
}

fn main() {
    #[cfg(feature = "alloc")]
    {
        extern crate std;
        use std::println;
        
        println!("=== Avila Error - Basic Usage Examples ===\n");
        
        // Exemplo 1: Validação bem-sucedida
        match validate_input(50) {
            Ok(v) => println!("✅ Valid input: {}", v),
            Err(e) => println!("❌ Error: {}", e.format()),
        }
        
        // Exemplo 2: Valor negativo
        match validate_input(-10) {
            Ok(_) => println!("Unexpected success"),
            Err(e) => println!("❌ Expected error: {}", e.format()),
        }
        
        // Exemplo 3: Valor muito grande
        match validate_input(200) {
            Ok(_) => println!("Unexpected success"),
            Err(e) => println!("❌ Expected error: {}", e.format()),
        }
        
        // Exemplo 4: Processamento
        match process_data(10) {
            Ok(v) => println!("✅ Processed value: {}", v),
            Err(e) => println!("❌ Error: {}", e.format()),
        }
        
        // Exemplo 5: Valor especial (42)
        match process_data(42) {
            Ok(_) => println!("Unexpected success"),
            Err(e) => println!("❌ Expected error: {}", e.format()),
        }
        
        // Exemplo 6: Com contexto
        match example_with_context() {
            Ok(_) => println!("Unexpected success"),
            Err(e) => println!("❌ Error with context:\n{}", e.format()),
        }
        
        // Exemplo 7: Todos os tipos de erro
        println!("\n=== All Error Kinds ===");
        let errors = [
            AvilaError::invalid_input("test".into()),
            AvilaError::not_found("test".into()),
            AvilaError::permission_denied("test".into()),
            AvilaError::connection_failed("test".into()),
            AvilaError::timeout("test".into()),
            AvilaError::data_corruption("test".into()),
            AvilaError::config_error("test".into()),
            AvilaError::authentication_failed("test".into()),
            AvilaError::authorization_failed("test".into()),
            AvilaError::already_exists("test".into()),
            AvilaError::resource_exhausted("test".into()),
            AvilaError::cancelled("test".into()),
            AvilaError::internal("test".into()),
            AvilaError::not_implemented("test".into()),
            AvilaError::unavailable("test".into()),
            AvilaError::unknown("test".into()),
        ];
        
        for err in errors {
            println!("  - {}", err.format());
        }
        
        println!("\n=== Examples completed ===");
    }
}
