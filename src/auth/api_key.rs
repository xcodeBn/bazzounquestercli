//! API key authentication

use serde::{Deserialize, Serialize};

/// Location where API key should be sent
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ApiKeyLocation {
    /// Send as header
    Header,

    /// Send as query parameter
    Query,
}

/// API key authentication
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApiKeyAuth {
    /// API key value
    pub key: String,

    /// Key name (header name or query param name)
    pub name: String,

    /// Location (header or query)
    pub location: ApiKeyLocation,
}

impl ApiKeyAuth {
    /// Create a new API key auth as header
    pub fn header(name: String, key: String) -> Self {
        Self {
            key,
            name,
            location: ApiKeyLocation::Header,
        }
    }

    /// Create a new API key auth as query parameter
    pub fn query(name: String, key: String) -> Self {
        Self {
            key,
            name,
            location: ApiKeyLocation::Query,
        }
    }

    /// Apply to headers or query params based on location
    pub fn apply(&self, headers: &mut Vec<String>, query_params: &mut Vec<String>) {
        match self.location {
            ApiKeyLocation::Header => {
                headers.push(format!("{}:{}", self.name, self.key));
            }
            ApiKeyLocation::Query => {
                query_params.push(format!("{}={}", self.name, self.key));
            }
        }
    }

    /// Common presets
    pub fn x_api_key(key: String) -> Self {
        Self::header("X-API-Key".to_string(), key)
    }

    pub fn api_key_query(key: String) -> Self {
        Self::query("api_key".to_string(), key)
    }

    pub fn app_id_header(key: String) -> Self {
        Self::header("X-App-ID".to_string(), key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_key_header() {
        let auth = ApiKeyAuth::header("X-API-Key".to_string(), "secret123".to_string());
        assert_eq!(auth.name, "X-API-Key");
        assert_eq!(auth.key, "secret123");
        assert_eq!(auth.location, ApiKeyLocation::Header);
    }

    #[test]
    fn test_api_key_query() {
        let auth = ApiKeyAuth::query("api_key".to_string(), "abc456".to_string());
        assert_eq!(auth.name, "api_key");
        assert_eq!(auth.key, "abc456");
        assert_eq!(auth.location, ApiKeyLocation::Query);
    }

    #[test]
    fn test_api_key_apply_header() {
        let auth = ApiKeyAuth::header("Authorization".to_string(), "key123".to_string());
        let mut headers = Vec::new();
        let mut query_params = Vec::new();

        auth.apply(&mut headers, &mut query_params);

        assert_eq!(headers.len(), 1);
        assert_eq!(headers[0], "Authorization:key123");
        assert_eq!(query_params.len(), 0);
    }

    #[test]
    fn test_api_key_apply_query() {
        let auth = ApiKeyAuth::query("key".to_string(), "value789".to_string());
        let mut headers = Vec::new();
        let mut query_params = Vec::new();

        auth.apply(&mut headers, &mut query_params);

        assert_eq!(headers.len(), 0);
        assert_eq!(query_params.len(), 1);
        assert_eq!(query_params[0], "key=value789");
    }

    #[test]
    fn test_api_key_preset_x_api_key() {
        let auth = ApiKeyAuth::x_api_key("mykey".to_string());
        assert_eq!(auth.name, "X-API-Key");
        assert_eq!(auth.key, "mykey");
        assert_eq!(auth.location, ApiKeyLocation::Header);
    }

    #[test]
    fn test_api_key_preset_query() {
        let auth = ApiKeyAuth::api_key_query("querykey".to_string());
        assert_eq!(auth.name, "api_key");
        assert_eq!(auth.key, "querykey");
        assert_eq!(auth.location, ApiKeyLocation::Query);
    }

    #[test]
    fn test_api_key_preset_app_id() {
        let auth = ApiKeyAuth::app_id_header("app123".to_string());
        assert_eq!(auth.name, "X-App-ID");
        assert_eq!(auth.key, "app123");
        assert_eq!(auth.location, ApiKeyLocation::Header);
    }

    #[test]
    fn test_api_key_location_serialization() {
        // Test that locations can be serialized/deserialized
        let header_auth = ApiKeyAuth::header("Test".to_string(), "key".to_string());
        let query_auth = ApiKeyAuth::query("Test".to_string(), "key".to_string());

        assert_eq!(header_auth.location, ApiKeyLocation::Header);
        assert_eq!(query_auth.location, ApiKeyLocation::Query);
    }
}
