//! Bearer token authentication (JWT, API tokens)

use serde::{Deserialize, Serialize};

/// Bearer token authentication
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BearerAuth {
    /// Access token
    pub token: String,

    /// Optional token prefix (default: "Bearer")
    pub prefix: Option<String>,
}

impl BearerAuth {
    /// Create a new bearer auth with default prefix
    pub fn new(token: String) -> Self {
        Self {
            token,
            prefix: Some("Bearer".to_string()),
        }
    }

    /// Create with custom prefix
    pub fn with_prefix(token: String, prefix: String) -> Self {
        Self {
            token,
            prefix: Some(prefix),
        }
    }

    /// Create without prefix (token only)
    pub fn token_only(token: String) -> Self {
        Self {
            token,
            prefix: None,
        }
    }

    /// Apply to headers
    pub fn apply_to_headers(&self, headers: &mut Vec<String>) {
        let auth_value = match &self.prefix {
            Some(prefix) => format!("{} {}", prefix, self.token),
            None => self.token.clone(),
        };

        headers.push(format!("Authorization:{}", auth_value));
    }

    /// Create from header value
    pub fn from_header(header_value: &str) -> Option<Self> {
        // Try to extract prefix and token
        if let Some((prefix, token)) = header_value.split_once(' ') {
            Some(Self::with_prefix(token.to_string(), prefix.to_string()))
        } else {
            // No prefix, just token
            Some(Self::token_only(header_value.to_string()))
        }
    }

    /// Check if this is a JWT token (basic check)
    pub fn is_jwt(&self) -> bool {
        self.token.matches('.').count() == 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bearer_auth_new() {
        let auth = BearerAuth::new("mytoken123".to_string());
        assert_eq!(auth.token, "mytoken123");
        assert_eq!(auth.prefix, Some("Bearer".to_string()));
    }

    #[test]
    fn test_bearer_auth_with_prefix() {
        let auth = BearerAuth::with_prefix("token456".to_string(), "Token".to_string());
        assert_eq!(auth.token, "token456");
        assert_eq!(auth.prefix, Some("Token".to_string()));
    }

    #[test]
    fn test_bearer_auth_token_only() {
        let auth = BearerAuth::token_only("justtoken".to_string());
        assert_eq!(auth.token, "justtoken");
        assert_eq!(auth.prefix, None);
    }

    #[test]
    fn test_bearer_auth_apply_to_headers() {
        let auth = BearerAuth::new("abc123".to_string());
        let mut headers = Vec::new();

        auth.apply_to_headers(&mut headers);

        assert_eq!(headers.len(), 1);
        assert_eq!(headers[0], "Authorization:Bearer abc123");
    }

    #[test]
    fn test_bearer_auth_apply_token_only() {
        let auth = BearerAuth::token_only("xyz789".to_string());
        let mut headers = Vec::new();

        auth.apply_to_headers(&mut headers);

        assert_eq!(headers.len(), 1);
        assert_eq!(headers[0], "Authorization:xyz789");
    }

    #[test]
    fn test_bearer_auth_from_header() {
        let auth = BearerAuth::from_header("Bearer mytoken").unwrap();
        assert_eq!(auth.token, "mytoken");
        assert_eq!(auth.prefix, Some("Bearer".to_string()));
    }

    #[test]
    fn test_bearer_auth_from_header_custom_prefix() {
        let auth = BearerAuth::from_header("Token customtoken").unwrap();
        assert_eq!(auth.token, "customtoken");
        assert_eq!(auth.prefix, Some("Token".to_string()));
    }

    #[test]
    fn test_bearer_auth_from_header_no_prefix() {
        let auth = BearerAuth::from_header("standalone_token").unwrap();
        assert_eq!(auth.token, "standalone_token");
        assert_eq!(auth.prefix, None);
    }

    #[test]
    fn test_bearer_auth_is_jwt() {
        let jwt = BearerAuth::new("eyJhbGc.eyJzdWI.SflKxw".to_string());
        assert!(jwt.is_jwt());

        let simple = BearerAuth::new("simpletoken".to_string());
        assert!(!simple.is_jwt());
    }

    #[test]
    fn test_bearer_auth_round_trip() {
        let original = BearerAuth::new("testtoken123".to_string());
        let mut headers = Vec::new();
        original.apply_to_headers(&mut headers);

        // Extract the auth value
        let auth_value = headers[0].strip_prefix("Authorization:").unwrap();
        let decoded = BearerAuth::from_header(auth_value).unwrap();

        assert_eq!(original, decoded);
    }
}
