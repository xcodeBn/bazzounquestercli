//! OAuth 2.0 authentication

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// OAuth 2.0 grant type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GrantType {
    /// Authorization code flow
    AuthorizationCode,

    /// Client credentials flow
    ClientCredentials,

    /// Resource owner password credentials
    Password,

    /// Refresh token flow
    RefreshToken,

    /// Implicit flow (deprecated but still used)
    Implicit,
}

/// OAuth 2.0 token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuth2Token {
    /// Access token
    pub access_token: String,

    /// Token type (usually "Bearer")
    pub token_type: String,

    /// Expiration timestamp
    pub expires_at: Option<DateTime<Utc>>,

    /// Refresh token (optional)
    pub refresh_token: Option<String>,

    /// Scopes granted
    pub scopes: Vec<String>,
}

impl OAuth2Token {
    /// Create a new OAuth2 token
    pub fn new(access_token: String, token_type: String) -> Self {
        Self {
            access_token,
            token_type,
            expires_at: None,
            refresh_token: None,
            scopes: Vec::new(),
        }
    }

    /// Set expiration (in seconds from now)
    pub fn with_expiration(mut self, expires_in: i64) -> Self {
        self.expires_at = Some(Utc::now() + chrono::Duration::seconds(expires_in));
        self
    }

    /// Set refresh token
    pub fn with_refresh_token(mut self, refresh_token: String) -> Self {
        self.refresh_token = Some(refresh_token);
        self
    }

    /// Set scopes
    pub fn with_scopes(mut self, scopes: Vec<String>) -> Self {
        self.scopes = scopes;
        self
    }

    /// Check if token is expired
    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            Utc::now() >= expires_at
        } else {
            false
        }
    }

    /// Check if token needs refresh (expires within 5 minutes)
    pub fn needs_refresh(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            let threshold = Utc::now() + chrono::Duration::minutes(5);
            threshold >= expires_at
        } else {
            false
        }
    }

    /// Get authorization header value
    pub fn to_header_value(&self) -> String {
        format!("{} {}", self.token_type, self.access_token)
    }
}

/// OAuth 2.0 authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuth2Auth {
    /// Grant type
    pub grant_type: GrantType,

    /// Client ID
    pub client_id: String,

    /// Client secret
    pub client_secret: Option<String>,

    /// Authorization endpoint URL
    pub auth_url: Option<String>,

    /// Token endpoint URL
    pub token_url: Option<String>,

    /// Current token
    pub token: Option<OAuth2Token>,

    /// Requested scopes
    pub scopes: Vec<String>,
}

impl OAuth2Auth {
    /// Create a new OAuth2 auth configuration
    pub fn new(grant_type: GrantType, client_id: String) -> Self {
        Self {
            grant_type,
            client_id,
            client_secret: None,
            auth_url: None,
            token_url: None,
            token: None,
            scopes: Vec::new(),
        }
    }

    /// Set client secret
    pub fn with_client_secret(mut self, secret: String) -> Self {
        self.client_secret = Some(secret);
        self
    }

    /// Set authorization URL
    pub fn with_auth_url(mut self, url: String) -> Self {
        self.auth_url = Some(url);
        self
    }

    /// Set token URL
    pub fn with_token_url(mut self, url: String) -> Self {
        self.token_url = Some(url);
        self
    }

    /// Set token
    pub fn with_token(mut self, token: OAuth2Token) -> Self {
        self.token = Some(token);
        self
    }

    /// Set scopes
    pub fn with_scopes(mut self, scopes: Vec<String>) -> Self {
        self.scopes = scopes;
        self
    }

    /// Apply to headers
    pub fn apply_to_headers(&self, headers: &mut Vec<String>) {
        if let Some(token) = &self.token {
            if !token.is_expired() {
                headers.push(format!("Authorization:{}", token.to_header_value()));
            }
        }
    }

    /// Check if authentication is configured and valid
    pub fn is_valid(&self) -> bool {
        if let Some(token) = &self.token {
            !token.is_expired()
        } else {
            false
        }
    }
}

impl PartialEq for OAuth2Auth {
    fn eq(&self, other: &Self) -> bool {
        self.grant_type == other.grant_type
            && self.client_id == other.client_id
            && self.client_secret == other.client_secret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oauth2_token_creation() {
        let token = OAuth2Token::new("access123".to_string(), "Bearer".to_string());
        assert_eq!(token.access_token, "access123");
        assert_eq!(token.token_type, "Bearer");
    }

    #[test]
    fn test_oauth2_token_with_expiration() {
        let token = OAuth2Token::new("token".to_string(), "Bearer".to_string())
            .with_expiration(3600);

        assert!(!token.is_expired());
        assert!(!token.needs_refresh());
    }

    #[test]
    fn test_oauth2_token_with_refresh() {
        let token = OAuth2Token::new("access".to_string(), "Bearer".to_string())
            .with_refresh_token("refresh123".to_string());

        assert_eq!(token.refresh_token, Some("refresh123".to_string()));
    }

    #[test]
    fn test_oauth2_token_with_scopes() {
        let token = OAuth2Token::new("token".to_string(), "Bearer".to_string())
            .with_scopes(vec!["read".to_string(), "write".to_string()]);

        assert_eq!(token.scopes.len(), 2);
        assert!(token.scopes.contains(&"read".to_string()));
    }

    #[test]
    fn test_oauth2_token_to_header_value() {
        let token = OAuth2Token::new("mytoken123".to_string(), "Bearer".to_string());
        assert_eq!(token.to_header_value(), "Bearer mytoken123");
    }

    #[test]
    fn test_oauth2_auth_creation() {
        let auth = OAuth2Auth::new(GrantType::ClientCredentials, "client123".to_string());
        assert_eq!(auth.grant_type, GrantType::ClientCredentials);
        assert_eq!(auth.client_id, "client123");
    }

    #[test]
    fn test_oauth2_auth_with_secret() {
        let auth = OAuth2Auth::new(GrantType::AuthorizationCode, "client".to_string())
            .with_client_secret("secret456".to_string());

        assert_eq!(auth.client_secret, Some("secret456".to_string()));
    }

    #[test]
    fn test_oauth2_auth_with_urls() {
        let auth = OAuth2Auth::new(GrantType::AuthorizationCode, "client".to_string())
            .with_auth_url("https://auth.example.com".to_string())
            .with_token_url("https://token.example.com".to_string());

        assert_eq!(
            auth.auth_url,
            Some("https://auth.example.com".to_string())
        );
        assert_eq!(
            auth.token_url,
            Some("https://token.example.com".to_string())
        );
    }

    #[test]
    fn test_oauth2_auth_apply_to_headers() {
        let token = OAuth2Token::new("testtoken".to_string(), "Bearer".to_string());
        let auth = OAuth2Auth::new(GrantType::ClientCredentials, "client".to_string())
            .with_token(token);

        let mut headers = Vec::new();
        auth.apply_to_headers(&mut headers);

        assert_eq!(headers.len(), 1);
        assert_eq!(headers[0], "Authorization:Bearer testtoken");
    }

    #[test]
    fn test_oauth2_auth_is_valid() {
        let token = OAuth2Token::new("token".to_string(), "Bearer".to_string())
            .with_expiration(3600);

        let auth = OAuth2Auth::new(GrantType::ClientCredentials, "client".to_string())
            .with_token(token);

        assert!(auth.is_valid());
    }

    #[test]
    fn test_oauth2_auth_invalid_without_token() {
        let auth = OAuth2Auth::new(GrantType::ClientCredentials, "client".to_string());
        assert!(!auth.is_valid());
    }

    #[test]
    fn test_oauth2_grant_types() {
        let types = vec![
            GrantType::AuthorizationCode,
            GrantType::ClientCredentials,
            GrantType::Password,
            GrantType::RefreshToken,
            GrantType::Implicit,
        ];

        // Just verify they can be created and compared
        assert_eq!(types.len(), 5);
    }
}
