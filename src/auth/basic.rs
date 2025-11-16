//! Basic authentication (username/password)

use serde::{Deserialize, Serialize};

/// Basic authentication credentials
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BasicAuth {
    /// Username
    pub username: String,

    /// Password
    pub password: String,
}

impl BasicAuth {
    /// Create a new basic auth
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }

    /// Encode credentials to base64
    pub fn encode(&self) -> String {
        let credentials = format!("{}:{}", self.username, self.password);
        base64::Engine::encode(
            &base64::engine::general_purpose::STANDARD,
            credentials.as_bytes(),
        )
    }

    /// Apply to headers
    pub fn apply_to_headers(&self, headers: &mut Vec<String>) {
        let encoded = self.encode();
        headers.push(format!("Authorization:Basic {}", encoded));
    }

    /// Create from header value
    pub fn from_header(header_value: &str) -> Option<Self> {
        // Remove "Basic " prefix if present
        let encoded = header_value.strip_prefix("Basic ").unwrap_or(header_value);

        // Decode from base64
        let decoded = base64::Engine::decode(
            &base64::engine::general_purpose::STANDARD,
            encoded.as_bytes(),
        )
        .ok()?;

        let decoded_str = String::from_utf8(decoded).ok()?;

        // Split into username:password
        let (username, password) = decoded_str.split_once(':')?;

        Some(Self::new(username.to_string(), password.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_auth_creation() {
        let auth = BasicAuth::new("user".to_string(), "pass".to_string());
        assert_eq!(auth.username, "user");
        assert_eq!(auth.password, "pass");
    }

    #[test]
    fn test_basic_auth_encode() {
        let auth = BasicAuth::new("user".to_string(), "pass".to_string());
        let encoded = auth.encode();
        // "user:pass" in base64
        assert_eq!(encoded, "dXNlcjpwYXNz");
    }

    #[test]
    fn test_basic_auth_apply_to_headers() {
        let auth = BasicAuth::new("admin".to_string(), "secret123".to_string());
        let mut headers = Vec::new();

        auth.apply_to_headers(&mut headers);

        assert_eq!(headers.len(), 1);
        assert!(headers[0].starts_with("Authorization:Basic "));
    }

    #[test]
    fn test_basic_auth_from_header() {
        let encoded = "Basic dXNlcjpwYXNz";
        let auth = BasicAuth::from_header(encoded).unwrap();

        assert_eq!(auth.username, "user");
        assert_eq!(auth.password, "pass");
    }

    #[test]
    fn test_basic_auth_from_header_without_prefix() {
        let encoded = "dXNlcjpwYXNz";
        let auth = BasicAuth::from_header(encoded).unwrap();

        assert_eq!(auth.username, "user");
        assert_eq!(auth.password, "pass");
    }

    #[test]
    fn test_basic_auth_round_trip() {
        let original = BasicAuth::new("testuser".to_string(), "testpass".to_string());
        let encoded = format!("Basic {}", original.encode());
        let decoded = BasicAuth::from_header(&encoded).unwrap();

        assert_eq!(original, decoded);
    }

    #[test]
    fn test_basic_auth_from_invalid_header() {
        assert!(BasicAuth::from_header("invalid").is_none());
        assert!(BasicAuth::from_header("Basic invalid!@#").is_none());
    }
}
