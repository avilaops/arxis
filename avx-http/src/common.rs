//! Common utilities shared between client and server

use std::time::Duration;

/// Default timeout for requests
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

/// Parse URL into host and port
pub fn parse_url(url: &str) -> crate::Result<(String, u16, bool)> {
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err(crate::Error::InvalidUrl {
            url: url.to_string(),
            reason: "URL must start with http:// or https://".to_string(),
        });
    }

    let is_https = url.starts_with("https://");
    let url_without_scheme = url.trim_start_matches("http://").trim_start_matches("https://");

    let (host, port) = if let Some(pos) = url_without_scheme.find('/') {
        let host_port = &url_without_scheme[..pos];
        parse_host_port(host_port, is_https)?
    } else {
        parse_host_port(url_without_scheme, is_https)?
    };

    Ok((host, port, is_https))
}

fn parse_host_port(host_port: &str, is_https: bool) -> crate::Result<(String, u16)> {
    if let Some(pos) = host_port.rfind(':') {
        let host = host_port[..pos].to_string();
        let port_str = &host_port[pos + 1..];
        let port = port_str.parse::<u16>().map_err(|_| crate::Error::InvalidUrl {
            url: host_port.to_string(),
            reason: format!("Invalid port: {}", port_str),
        })?;
        Ok((host, port))
    } else {
        let default_port = if is_https { 443 } else { 80 };
        Ok((host_port.to_string(), default_port))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_url_http() {
        let (host, port, is_https) = parse_url("http://example.com").unwrap();
        assert_eq!(host, "example.com");
        assert_eq!(port, 80);
        assert!(!is_https);
    }

    #[test]
    fn test_parse_url_https() {
        let (host, port, is_https) = parse_url("https://example.com").unwrap();
        assert_eq!(host, "example.com");
        assert_eq!(port, 443);
        assert!(is_https);
    }

    #[test]
    fn test_parse_url_custom_port() {
        let (host, port, is_https) = parse_url("http://example.com:8080").unwrap();
        assert_eq!(host, "example.com");
        assert_eq!(port, 8080);
        assert!(!is_https);
    }

    #[test]
    fn test_parse_url_with_path() {
        let (host, port, is_https) = parse_url("https://api.avila.cloud/data").unwrap();
        assert_eq!(host, "api.avila.cloud");
        assert_eq!(port, 443);
        assert!(is_https);
    }

    #[test]
    fn test_parse_url_invalid() {
        let result = parse_url("not-a-url");
        assert!(result.is_err());
    }
}
