//! HTTP request building and configuration

use crate::auth::AuthScheme;
use crate::error::{Error, Result};
use crate::upload::FormData;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// HTTP methods supported by the client
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Head,
    Options,
}

impl HttpMethod {
    /// Convert method to uppercase string
    pub fn as_str(&self) -> &str {
        match self {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
            HttpMethod::Put => "PUT",
            HttpMethod::Delete => "DELETE",
            HttpMethod::Patch => "PATCH",
            HttpMethod::Head => "HEAD",
            HttpMethod::Options => "OPTIONS",
        }
    }

    /// Parse method from string
    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_uppercase().as_str() {
            "GET" => Ok(HttpMethod::Get),
            "POST" => Ok(HttpMethod::Post),
            "PUT" => Ok(HttpMethod::Put),
            "DELETE" => Ok(HttpMethod::Delete),
            "PATCH" => Ok(HttpMethod::Patch),
            "HEAD" => Ok(HttpMethod::Head),
            "OPTIONS" => Ok(HttpMethod::Options),
            _ => Err(Error::UnsupportedMethod(s.to_string())),
        }
    }
}

/// Builder for HTTP requests
#[derive(Debug, Clone)]
pub struct RequestBuilder {
    pub method: HttpMethod,
    pub url: String,
    pub headers: Vec<String>,
    pub query_params: Vec<String>,
    pub body: Option<String>,
    pub form_data: Option<FormData>,
    pub auth: AuthScheme,
}

impl RequestBuilder {
    /// Create a new request builder
    pub fn new(method: HttpMethod, url: String) -> Self {
        Self {
            method,
            url,
            headers: Vec::new(),
            query_params: Vec::new(),
            body: None,
            form_data: None,
            auth: AuthScheme::default(),
        }
    }

    /// Add a header
    pub fn header(mut self, header: String) -> Self {
        self.headers.push(header);
        self
    }

    /// Add headers
    pub fn headers(mut self, headers: Vec<String>) -> Self {
        self.headers.extend(headers);
        self
    }

    /// Add a query parameter
    pub fn query(mut self, param: String) -> Self {
        self.query_params.push(param);
        self
    }

    /// Add query parameters
    pub fn queries(mut self, params: Vec<String>) -> Self {
        self.query_params.extend(params);
        self
    }

    /// Set request body
    pub fn body(mut self, body: String) -> Self {
        self.body = Some(body);
        self
    }

    /// Set form data (for multipart/form-data or application/x-www-form-urlencoded)
    pub fn form(mut self, form_data: FormData) -> Self {
        self.form_data = Some(form_data);
        self
    }

    /// Get form data
    pub fn get_form_data(&self) -> Option<&FormData> {
        self.form_data.as_ref()
    }

    /// Set authentication
    pub fn auth(mut self, auth: AuthScheme) -> Self {
        self.auth = auth;
        self
    }

    /// Apply authentication to headers and query params
    pub fn apply_auth(&self, headers: &mut Vec<String>, query_params: &mut Vec<String>) {
        self.auth.apply(headers, query_params);
    }

    /// Parse headers into HeaderMap
    pub fn parse_headers(&self) -> Result<HeaderMap> {
        let mut header_map = HeaderMap::new();

        for header in &self.headers {
            if let Some((key, value)) = header.split_once(':') {
                let key = key.trim();
                let value = value.trim();

                let header_name = HeaderName::from_bytes(key.as_bytes())
                    .map_err(|_| Error::InvalidHeader(format!("Invalid header name: {}", key)))?;

                let header_value = HeaderValue::from_str(value)
                    .map_err(|_| Error::InvalidHeader(format!("Invalid header value: {}", value)))?;

                header_map.insert(header_name, header_value);
            } else {
                return Err(Error::InvalidHeader(format!(
                    "Header must be in format 'Key:Value', got: {}",
                    header
                )));
            }
        }

        Ok(header_map)
    }

    /// Parse query parameters into HashMap
    pub fn parse_query_params(&self) -> Result<HashMap<String, String>> {
        let mut query_map = HashMap::new();

        for param in &self.query_params {
            if let Some((key, value)) = param.split_once('=') {
                query_map.insert(key.to_string(), value.to_string());
            } else {
                return Err(Error::InvalidQuery(format!(
                    "Query parameter must be in format 'key=value', got: {}",
                    param
                )));
            }
        }

        Ok(query_map)
    }

    /// Parse body as JSON Value
    pub fn parse_body(&self) -> Result<Option<Value>> {
        if let Some(body_str) = &self.body {
            let json_value = serde_json::from_str::<Value>(body_str)?;
            Ok(Some(json_value))
        } else {
            Ok(None)
        }
    }

    /// Get raw body
    pub fn get_raw_body(&self) -> Option<&str> {
        self.body.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_method_from_str() {
        assert_eq!(HttpMethod::from_str("GET").unwrap(), HttpMethod::Get);
        assert_eq!(HttpMethod::from_str("get").unwrap(), HttpMethod::Get);
        assert_eq!(HttpMethod::from_str("POST").unwrap(), HttpMethod::Post);
        assert_eq!(HttpMethod::from_str("post").unwrap(), HttpMethod::Post);
    }

    #[test]
    fn test_http_method_as_str() {
        assert_eq!(HttpMethod::Get.as_str(), "GET");
        assert_eq!(HttpMethod::Post.as_str(), "POST");
    }

    #[test]
    fn test_request_builder() {
        let builder = RequestBuilder::new(HttpMethod::Get, "https://example.com".to_string())
            .header("Content-Type:application/json".to_string())
            .query("foo=bar".to_string());

        assert_eq!(builder.method, HttpMethod::Get);
        assert_eq!(builder.url, "https://example.com");
        assert_eq!(builder.headers.len(), 1);
        assert_eq!(builder.query_params.len(), 1);
    }

    #[test]
    fn test_parse_headers_valid() {
        let builder = RequestBuilder::new(HttpMethod::Get, "https://example.com".to_string())
            .header("Content-Type:application/json".to_string())
            .header("Authorization:Bearer token123".to_string());

        let headers = builder.parse_headers().unwrap();
        assert_eq!(headers.len(), 2);
    }

    #[test]
    fn test_parse_headers_invalid() {
        let builder = RequestBuilder::new(HttpMethod::Get, "https://example.com".to_string())
            .header("InvalidHeader".to_string());

        assert!(builder.parse_headers().is_err());
    }

    #[test]
    fn test_parse_query_params_valid() {
        let builder = RequestBuilder::new(HttpMethod::Get, "https://example.com".to_string())
            .query("foo=bar".to_string())
            .query("baz=qux".to_string());

        let params = builder.parse_query_params().unwrap();
        assert_eq!(params.len(), 2);
        assert_eq!(params.get("foo"), Some(&"bar".to_string()));
    }

    #[test]
    fn test_parse_query_params_invalid() {
        let builder = RequestBuilder::new(HttpMethod::Get, "https://example.com".to_string())
            .query("invalidparam".to_string());

        assert!(builder.parse_query_params().is_err());
    }

    #[test]
    fn test_parse_body_valid_json() {
        let builder = RequestBuilder::new(HttpMethod::Post, "https://example.com".to_string())
            .body(r#"{"key":"value"}"#.to_string());

        let body = builder.parse_body().unwrap();
        assert!(body.is_some());
    }

    #[test]
    fn test_parse_body_invalid_json() {
        let builder = RequestBuilder::new(HttpMethod::Post, "https://example.com".to_string())
            .body("not valid json".to_string());

        assert!(builder.parse_body().is_err());
    }
}
