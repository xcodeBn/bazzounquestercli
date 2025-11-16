//! Request item data structure for collections

use crate::http::HttpMethod;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// A saved HTTP request in a collection
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RequestItem {
    /// Unique identifier
    pub id: Uuid,

    /// Request name
    pub name: String,

    /// Optional description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// HTTP method
    pub method: String,

    /// Request URL (can include {{variables}})
    pub url: String,

    /// Headers
    #[serde(default)]
    pub headers: HashMap<String, String>,

    /// Query parameters
    #[serde(default)]
    pub query_params: HashMap<String, String>,

    /// Request body
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,

    /// Body type (json, form, raw, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_type: Option<String>,

    /// Created timestamp
    pub created_at: DateTime<Utc>,

    /// Last modified timestamp
    pub updated_at: DateTime<Utc>,

    /// Tags for categorization
    #[serde(default)]
    pub tags: Vec<String>,

    /// Custom metadata
    #[serde(default)]
    pub metadata: HashMap<String, String>,
}

impl RequestItem {
    /// Create a new request item
    pub fn new(name: String, method: HttpMethod, url: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            description: None,
            method: method.as_str().to_string(),
            url,
            headers: HashMap::new(),
            query_params: HashMap::new(),
            body: None,
            body_type: None,
            created_at: now,
            updated_at: now,
            tags: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Add a header to the request
    pub fn with_header(mut self, key: String, value: String) -> Self {
        self.headers.insert(key, value);
        self.updated_at = Utc::now();
        self
    }

    /// Add a query parameter
    pub fn with_query(mut self, key: String, value: String) -> Self {
        self.query_params.insert(key, value);
        self.updated_at = Utc::now();
        self
    }

    /// Set request body
    pub fn with_body(mut self, body: String, body_type: Option<String>) -> Self {
        self.body = Some(body);
        self.body_type = body_type;
        self.updated_at = Utc::now();
        self
    }

    /// Add a tag
    pub fn with_tag(mut self, tag: String) -> Self {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
            self.updated_at = Utc::now();
        }
        self
    }

    /// Set description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self.updated_at = Utc::now();
        self
    }

    /// Update the modified timestamp
    pub fn touch(&mut self) {
        self.updated_at = Utc::now();
    }

    /// Convert to HTTP request builder
    pub fn to_request_builder(&self) -> crate::http::RequestBuilder {
        let method = HttpMethod::parse(&self.method).unwrap_or(HttpMethod::Get);
        let mut builder = crate::http::RequestBuilder::new(method, self.url.clone());

        // Add headers
        for (key, value) in &self.headers {
            builder = builder.header(format!("{}:{}", key, value));
        }

        // Add query params
        for (key, value) in &self.query_params {
            builder = builder.query(format!("{}={}", key, value));
        }

        // Add body
        if let Some(body) = &self.body {
            builder = builder.body(body.clone());
        }

        builder
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_item_creation() {
        let item = RequestItem::new(
            "Test Request".to_string(),
            HttpMethod::Get,
            "https://example.com".to_string(),
        );

        assert_eq!(item.name, "Test Request");
        assert_eq!(item.method, "GET");
        assert_eq!(item.url, "https://example.com");
        assert!(item.headers.is_empty());
    }

    #[test]
    fn test_request_item_with_header() {
        let item = RequestItem::new(
            "Test".to_string(),
            HttpMethod::Post,
            "https://example.com".to_string(),
        )
        .with_header("Content-Type".to_string(), "application/json".to_string());

        assert_eq!(item.headers.len(), 1);
        assert_eq!(
            item.headers.get("Content-Type"),
            Some(&"application/json".to_string())
        );
    }

    #[test]
    fn test_request_item_with_body() {
        let item = RequestItem::new(
            "Test".to_string(),
            HttpMethod::Post,
            "https://example.com".to_string(),
        )
        .with_body(r#"{"key":"value"}"#.to_string(), Some("json".to_string()));

        assert_eq!(item.body, Some(r#"{"key":"value"}"#.to_string()));
        assert_eq!(item.body_type, Some("json".to_string()));
    }

    #[test]
    fn test_request_item_with_tags() {
        let item = RequestItem::new(
            "Test".to_string(),
            HttpMethod::Get,
            "https://example.com".to_string(),
        )
        .with_tag("api".to_string())
        .with_tag("test".to_string());

        assert_eq!(item.tags.len(), 2);
        assert!(item.tags.contains(&"api".to_string()));
        assert!(item.tags.contains(&"test".to_string()));
    }

    #[test]
    fn test_request_item_serialization() {
        let item = RequestItem::new(
            "Test".to_string(),
            HttpMethod::Post,
            "https://example.com".to_string(),
        )
        .with_header("Content-Type".to_string(), "application/json".to_string());

        let json = serde_json::to_string(&item).unwrap();
        let deserialized: RequestItem = serde_json::from_str(&json).unwrap();

        assert_eq!(item.id, deserialized.id);
        assert_eq!(item.name, deserialized.name);
        assert_eq!(item.method, deserialized.method);
    }
}
