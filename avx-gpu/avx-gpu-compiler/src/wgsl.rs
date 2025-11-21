//! WGSL compiler utilities

use naga::front::wgsl;

/// Validate WGSL shader source
pub fn validate(source: &str) -> anyhow::Result<()> {
    let _module = wgsl::parse_str(source)?;
    Ok(())
}

/// Parse WGSL and extract entry points
pub fn extract_entry_points(source: &str) -> anyhow::Result<Vec<String>> {
    let module = wgsl::parse_str(source)?;

    let entry_points: Vec<String> = module
        .entry_points
        .iter()
        .map(|ep| ep.name.clone())
        .collect();

    Ok(entry_points)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_simple_kernel() {
        let source = r#"
            @compute @workgroup_size(256)
            fn main(@builtin(global_invocation_id) id: vec3<u32>) {
                // Empty kernel
            }
        "#;

        assert!(validate(source).is_ok());
    }

    #[test]
    fn test_extract_entry_points() {
        let source = r#"
            @compute @workgroup_size(256)
            fn kernel_a() {}

            @compute @workgroup_size(128)
            fn kernel_b() {}
        "#;

        let entry_points = extract_entry_points(source).unwrap();
        assert_eq!(entry_points.len(), 2);
        assert!(entry_points.contains(&"kernel_a".to_string()));
        assert!(entry_points.contains(&"kernel_b".to_string()));
    }
}
