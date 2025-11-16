//! Authentication helpers for various auth schemes

pub mod api_key;
pub mod basic;
pub mod bearer;
pub mod oauth2;

pub use api_key::ApiKeyAuth;
pub use basic::BasicAuth;
pub use bearer::BearerAuth;
pub use oauth2::OAuth2Auth;

use serde::{Deserialize, Serialize};

/// Authentication scheme types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum AuthScheme {
    /// No authentication
    #[default]
    None,

    /// Basic authentication (username/password)
    Basic(BasicAuth),

    /// Bearer token authentication
    Bearer(BearerAuth),

    /// API key authentication
    ApiKey(ApiKeyAuth),

    /// OAuth 2.0 authentication
    OAuth2(OAuth2Auth),
}

impl AuthScheme {
    /// Apply authentication to headers or query params
    pub fn apply(&self, headers: &mut Vec<String>, query_params: &mut Vec<String>) {
        match self {
            AuthScheme::None => {}
            AuthScheme::Basic(auth) => auth.apply_to_headers(headers),
            AuthScheme::Bearer(auth) => auth.apply_to_headers(headers),
            AuthScheme::ApiKey(auth) => auth.apply(headers, query_params),
            AuthScheme::OAuth2(auth) => auth.apply_to_headers(headers),
        }
    }

    /// Check if authentication is configured
    pub fn is_configured(&self) -> bool {
        !matches!(self, AuthScheme::None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_scheme_default() {
        let auth = AuthScheme::default();
        assert_eq!(auth, AuthScheme::None);
        assert!(!auth.is_configured());
    }

    #[test]
    fn test_auth_scheme_none() {
        let auth = AuthScheme::None;
        let mut headers = Vec::new();
        let mut query_params = Vec::new();

        auth.apply(&mut headers, &mut query_params);

        assert_eq!(headers.len(), 0);
        assert_eq!(query_params.len(), 0);
    }
}
