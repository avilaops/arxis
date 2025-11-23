//! Template rendering support

pub mod filters {
    /// Format bytes to human-readable size
    pub fn format_bytes(bytes: &u64) -> askama::Result<String> {
        let size = *bytes as f64;
        let units = ["B", "KB", "MB", "GB", "TB"];
        let mut unit_index = 0;
        let mut value = size;

        while value >= 1024.0 && unit_index < units.len() - 1 {
            value /= 1024.0;
            unit_index += 1;
        }

        Ok(format!("{:.2} {}", value, units[unit_index]))
    }

    /// Format timestamp to human-readable date
    pub fn format_date(timestamp: &str) -> askama::Result<String> {
        // TODO: Use chrono for proper date formatting
        Ok(timestamp.to_string())
    }
}
