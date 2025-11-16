//! History entry data structures

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use uuid::Uuid;

/// A complete request/response entry in history
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HistoryEntry {
    /// Unique identifier
    pub id: Uuid,

    /// Request information
    pub request: RequestLog,

    /// Response information (if completed)
    pub response: Option<ResponseLog>,

    /// Timestamp when request was sent
    pub timestamp: DateTime<Utc>,

    /// Duration of the request
    pub duration: Option<Duration>,

    /// Collection ID (if from a collection)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_id: Option<Uuid>,

    /// Environment ID used (if any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_id: Option<Uuid>,

    /// Tags for categorization
    #[serde(default)]
    pub tags: Vec<String>,

    /// Custom metadata
    #[serde(default)]
    pub metadata: HashMap<String, String>,
}

/// Request log information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RequestLog {
    /// HTTP method
    pub method: String,

    /// Request URL (after variable substitution)
    pub url: String,

    /// Original URL (before substitution)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_url: Option<String>,

    /// Request headers
    #[serde(default)]
    pub headers: HashMap<String, String>,

    /// Query parameters
    #[serde(default)]
    pub query_params: HashMap<String, String>,

    /// Request body
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,

    /// Request body size in bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_size: Option<usize>,
}

/// Response log information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseLog {
    /// Status code
    pub status_code: u16,

    /// Status text
    pub status_text: String,

    /// Response headers
    #[serde(default)]
    pub headers: HashMap<String, String>,

    /// Response body
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,

    /// Response body size in bytes
    pub body_size: usize,

    /// Content type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,

    /// Was the response successful (2xx)?
    pub is_success: bool,

    /// Was there an error?
    pub is_error: bool,

    /// Error message (if any)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

impl HistoryEntry {
    /// Create a new history entry for a request
    pub fn new(request: RequestLog) -> Self {
        Self {
            id: Uuid::new_v4(),
            request,
            response: None,
            timestamp: Utc::now(),
            duration: None,
            collection_id: None,
            environment_id: None,
            tags: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Set the response
    pub fn set_response(&mut self, response: ResponseLog, duration: Duration) {
        self.response = Some(response);
        self.duration = Some(duration);
    }

    /// Check if this entry was successful
    pub fn is_successful(&self) -> bool {
        self.response
            .as_ref()
            .map(|r| r.is_success)
            .unwrap_or(false)
    }

    /// Check if this entry had an error
    pub fn has_error(&self) -> bool {
        self.response.as_ref().map(|r| r.is_error).unwrap_or(false)
    }

    /// Add a tag
    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
    }

    /// Set metadata
    pub fn set_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    /// Get short summary
    pub fn summary(&self) -> String {
        let status = if let Some(resp) = &self.response {
            format!("{} {}", resp.status_code, resp.status_text)
        } else {
            "Pending".to_string()
        };

        format!("{} {} - {}", self.request.method, self.request.url, status)
    }
}

impl RequestLog {
    /// Create a new request log
    pub fn new(method: String, url: String) -> Self {
        Self {
            method,
            url,
            original_url: None,
            headers: HashMap::new(),
            query_params: HashMap::new(),
            body: None,
            body_size: None,
        }
    }

    /// Calculate body size
    pub fn calculate_body_size(&mut self) {
        if let Some(body) = &self.body {
            self.body_size = Some(body.len());
        }
    }
}

impl ResponseLog {
    /// Create a new response log
    pub fn new(status_code: u16, status_text: String) -> Self {
        let is_success = (200..300).contains(&status_code);
        let is_error = status_code >= 400;

        Self {
            status_code,
            status_text,
            headers: HashMap::new(),
            body: None,
            body_size: 0,
            content_type: None,
            is_success,
            is_error,
            error_message: None,
        }
    }

    /// Set body and calculate size
    pub fn set_body(&mut self, body: String) {
        self.body_size = body.len();
        self.body = Some(body);
    }

    /// Set error
    pub fn set_error(&mut self, error: String) {
        self.is_error = true;
        self.error_message = Some(error);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_history_entry_creation() {
        let request = RequestLog::new("GET".to_string(), "https://api.example.com".to_string());
        let entry = HistoryEntry::new(request);

        assert_eq!(entry.request.method, "GET");
        assert_eq!(entry.request.url, "https://api.example.com");
        assert!(entry.response.is_none());
    }

    #[test]
    fn test_set_response() {
        let request = RequestLog::new("GET".to_string(), "https://api.example.com".to_string());
        let mut entry = HistoryEntry::new(request);

        let response = ResponseLog::new(200, "OK".to_string());
        entry.set_response(response, Duration::from_millis(150));

        assert!(entry.response.is_some());
        assert!(entry.is_successful());
        assert!(!entry.has_error());
    }

    #[test]
    fn test_request_log_with_body() {
        let mut request =
            RequestLog::new("POST".to_string(), "https://api.example.com".to_string());
        request.body = Some(r#"{"key":"value"}"#.to_string());
        request.calculate_body_size();

        assert_eq!(request.body_size, Some(15));
    }

    #[test]
    fn test_response_log_success() {
        let response = ResponseLog::new(200, "OK".to_string());
        assert!(response.is_success);
        assert!(!response.is_error);
    }

    #[test]
    fn test_response_log_error() {
        let response = ResponseLog::new(404, "Not Found".to_string());
        assert!(!response.is_success);
        assert!(response.is_error);
    }

    #[test]
    fn test_entry_tags() {
        let request = RequestLog::new("GET".to_string(), "https://api.example.com".to_string());
        let mut entry = HistoryEntry::new(request);

        entry.add_tag("api".to_string());
        entry.add_tag("test".to_string());
        entry.add_tag("api".to_string()); // Duplicate

        assert_eq!(entry.tags.len(), 2);
        assert!(entry.tags.contains(&"api".to_string()));
    }

    #[test]
    fn test_entry_summary() {
        let request = RequestLog::new("GET".to_string(), "https://api.example.com".to_string());
        let mut entry = HistoryEntry::new(request);

        let response = ResponseLog::new(200, "OK".to_string());
        entry.set_response(response, Duration::from_millis(100));

        let summary = entry.summary();
        assert!(summary.contains("GET"));
        assert!(summary.contains("200"));
        assert!(summary.contains("OK"));
    }

    #[test]
    fn test_serialization() {
        let request = RequestLog::new("POST".to_string(), "https://api.example.com".to_string());
        let entry = HistoryEntry::new(request);

        let json = serde_json::to_string(&entry).unwrap();
        let deserialized: HistoryEntry = serde_json::from_str(&json).unwrap();

        assert_eq!(entry.id, deserialized.id);
        assert_eq!(entry.request.method, deserialized.request.method);
    }
}
