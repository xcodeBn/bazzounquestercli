//! History logger for capturing requests and responses

use crate::history::{HistoryEntry, RequestLog, ResponseLog};
use crate::http::{HttpResponse, RequestBuilder};
use std::collections::HashMap;
use uuid::Uuid;

/// Logger for capturing HTTP request/response history
pub struct HistoryLogger {
    entries: Vec<HistoryEntry>,
    max_entries: usize,
    current_collection_id: Option<Uuid>,
    current_environment_id: Option<Uuid>,
}

impl HistoryLogger {
    /// Create a new history logger
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            max_entries: 1000, // Default max
            current_collection_id: None,
            current_environment_id: None,
        }
    }

    /// Create with custom max entries
    pub fn with_max_entries(max_entries: usize) -> Self {
        Self {
            entries: Vec::new(),
            max_entries,
            current_collection_id: None,
            current_environment_id: None,
        }
    }

    /// Set current collection ID
    pub fn set_collection_id(&mut self, id: Option<Uuid>) {
        self.current_collection_id = id;
    }

    /// Set current environment ID
    pub fn set_environment_id(&mut self, id: Option<Uuid>) {
        self.current_environment_id = id;
    }

    /// Log a request (before sending)
    pub fn log_request(&mut self, request: &RequestBuilder) -> Uuid {
        let mut request_log = RequestLog::new(
            request.method.as_str().to_string(),
            request.url.clone(),
        );

        // Parse headers
        for header in &request.headers {
            if let Some((key, value)) = header.split_once(':') {
                request_log.headers.insert(
                    key.trim().to_string(),
                    value.trim().to_string(),
                );
            }
        }

        // Parse query params
        for param in &request.query_params {
            if let Some((key, value)) = param.split_once('=') {
                request_log.query_params.insert(
                    key.to_string(),
                    value.to_string(),
                );
            }
        }

        // Add body
        if let Some(body) = &request.body {
            request_log.body = Some(body.clone());
            request_log.calculate_body_size();
        }

        let mut entry = HistoryEntry::new(request_log);
        entry.collection_id = self.current_collection_id;
        entry.environment_id = self.current_environment_id;

        let id = entry.id;

        // Add to history
        self.entries.push(entry);

        // Trim if exceeding max
        if self.entries.len() > self.max_entries {
            self.entries.remove(0);
        }

        id
    }

    /// Log a response (after receiving)
    pub fn log_response(&mut self, entry_id: &Uuid, response: &HttpResponse) {
        if let Some(entry) = self.entries.iter_mut().find(|e| e.id == *entry_id) {
            let mut response_log = ResponseLog::new(
                response.status.as_u16(),
                response.status.canonical_reason()
                    .unwrap_or("Unknown")
                    .to_string(),
            );

            // Copy headers
            for (name, value) in response.headers.iter() {
                response_log.headers.insert(
                    name.as_str().to_string(),
                    value.to_str().unwrap_or("").to_string(),
                );
            }

            // Set body
            if !response.body.is_empty() {
                response_log.set_body(response.body.clone());
            }

            // Set content type
            if let Some(ct) = response.headers.get("content-type") {
                response_log.content_type = Some(ct.to_str().unwrap_or("").to_string());
            }

            entry.set_response(response_log, response.duration);
        }
    }

    /// Log an error
    pub fn log_error(&mut self, entry_id: &Uuid, error: String) {
        if let Some(entry) = self.entries.iter_mut().find(|e| e.id == *entry_id) {
            let mut response_log = ResponseLog::new(0, "Error".to_string());
            response_log.set_error(error);
            entry.response = Some(response_log);
        }
    }

    /// Get all entries
    pub fn get_entries(&self) -> &[HistoryEntry] {
        &self.entries
    }

    /// Get entry by ID
    pub fn get_entry(&self, id: &Uuid) -> Option<&HistoryEntry> {
        self.entries.iter().find(|e| e.id == *id)
    }

    /// Get last N entries
    pub fn get_last_n(&self, n: usize) -> Vec<&HistoryEntry> {
        self.entries.iter().rev().take(n).collect()
    }

    /// Filter entries by method
    pub fn filter_by_method(&self, method: &str) -> Vec<&HistoryEntry> {
        self.entries
            .iter()
            .filter(|e| e.request.method.eq_ignore_ascii_case(method))
            .collect()
    }

    /// Filter entries by status code
    pub fn filter_by_status(&self, status_code: u16) -> Vec<&HistoryEntry> {
        self.entries
            .iter()
            .filter(|e| {
                e.response
                    .as_ref()
                    .map(|r| r.status_code == status_code)
                    .unwrap_or(false)
            })
            .collect()
    }

    /// Get successful entries only
    pub fn get_successful(&self) -> Vec<&HistoryEntry> {
        self.entries.iter().filter(|e| e.is_successful()).collect()
    }

    /// Get failed entries only
    pub fn get_failed(&self) -> Vec<&HistoryEntry> {
        self.entries.iter().filter(|e| e.has_error()).collect()
    }

    /// Search entries by URL pattern
    pub fn search_by_url(&self, pattern: &str) -> Vec<&HistoryEntry> {
        self.entries
            .iter()
            .filter(|e| e.request.url.contains(pattern))
            .collect()
    }

    /// Clear all entries
    pub fn clear(&mut self) {
        self.entries.clear();
    }

    /// Get total number of entries
    pub fn count(&self) -> usize {
        self.entries.len()
    }

    /// Export entries to HashMap for analysis
    pub fn to_hashmap(&self) -> HashMap<Uuid, &HistoryEntry> {
        self.entries.iter().map(|e| (e.id, e)).collect()
    }
}

impl Default for HistoryLogger {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::HttpMethod;

    #[test]
    fn test_logger_creation() {
        let logger = HistoryLogger::new();
        assert_eq!(logger.count(), 0);
        assert_eq!(logger.max_entries, 1000);
    }

    #[test]
    fn test_log_request() {
        let mut logger = HistoryLogger::new();
        let request = RequestBuilder::new(
            HttpMethod::Get,
            "https://api.example.com".to_string(),
        );

        let id = logger.log_request(&request);
        assert_eq!(logger.count(), 1);

        let entry = logger.get_entry(&id);
        assert!(entry.is_some());
        assert_eq!(entry.unwrap().request.method, "GET");
    }

    #[test]
    fn test_max_entries() {
        let mut logger = HistoryLogger::with_max_entries(2);

        let req1 = RequestBuilder::new(HttpMethod::Get, "https://example.com/1".to_string());
        let req2 = RequestBuilder::new(HttpMethod::Get, "https://example.com/2".to_string());
        let req3 = RequestBuilder::new(HttpMethod::Get, "https://example.com/3".to_string());

        logger.log_request(&req1);
        logger.log_request(&req2);
        logger.log_request(&req3);

        // Should only have 2 entries (oldest removed)
        assert_eq!(logger.count(), 2);
        assert!(logger.search_by_url("/1").is_empty());
        assert!(!logger.search_by_url("/3").is_empty());
    }

    #[test]
    fn test_filter_by_method() {
        let mut logger = HistoryLogger::new();

        logger.log_request(&RequestBuilder::new(HttpMethod::Get, "https://example.com/1".to_string()));
        logger.log_request(&RequestBuilder::new(HttpMethod::Post, "https://example.com/2".to_string()));
        logger.log_request(&RequestBuilder::new(HttpMethod::Get, "https://example.com/3".to_string()));

        let get_requests = logger.filter_by_method("GET");
        assert_eq!(get_requests.len(), 2);

        let post_requests = logger.filter_by_method("POST");
        assert_eq!(post_requests.len(), 1);
    }

    #[test]
    fn test_search_by_url() {
        let mut logger = HistoryLogger::new();

        logger.log_request(&RequestBuilder::new(HttpMethod::Get, "https://api.example.com/users".to_string()));
        logger.log_request(&RequestBuilder::new(HttpMethod::Get, "https://api.example.com/posts".to_string()));

        let results = logger.search_by_url("users");
        assert_eq!(results.len(), 1);

        let results = logger.search_by_url("api.example.com");
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_clear() {
        let mut logger = HistoryLogger::new();
        logger.log_request(&RequestBuilder::new(HttpMethod::Get, "https://example.com".to_string()));

        assert_eq!(logger.count(), 1);
        logger.clear();
        assert_eq!(logger.count(), 0);
    }
}
