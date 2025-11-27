//! JSON parser example
//!
//! Demonstrates zero-copy JSON parsing without serde

use avx_http::json::JsonValue;
use avx_http::error::Result;

fn main() -> Result<()> {
    println!("🚀 avx-http JSON Parser Example");
    println!("================================\n");

    // Parse simple JSON
    let json1 = r#"{"name": "Alice", "age": 30, "active": true}"#;
    let value1 = JsonValue::parse(json1)?;
    println!("✅ Parsed: {}", value1.to_string());

    if let Some(obj) = value1.as_object() {
        println!("   Name: {}", obj.get("name").unwrap().as_str().unwrap());
        println!("   Age: {}", obj.get("age").unwrap().as_f64().unwrap());
        println!("   Active: {}", obj.get("active").unwrap().as_bool().unwrap());
    }

    // Parse array
    println!("\n✅ Array parsing:");
    let json2 = r#"[1, 2, 3, "four", true, null]"#;
    let value2 = JsonValue::parse(json2)?;
    println!("   Parsed: {}", value2.to_string());

    if let Some(arr) = value2.as_array() {
        println!("   Length: {}", arr.len());
        for (i, item) in arr.iter().enumerate() {
            println!("   [{}] = {:?}", i, item);
        }
    }

    // Parse nested structure
    println!("\n✅ Nested structure:");
    let json3 = r#"{
        "user": {
            "name": "Bob",
            "scores": [95, 87, 92]
        },
        "metadata": {
            "created": "2025-01-01",
            "version": 1.0
        }
    }"#;
    let value3 = JsonValue::parse(json3)?;
    println!("   Parsed: {}", value3.to_string());

    println!("\n✨ Zero serde, zero external dependencies!");
    println!("   Pure Rust, fast compilation, fully auditable");

    Ok(())
}
