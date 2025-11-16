//! HTTP response handling and formatting

use crate::error::Result;
use reqwest::header::HeaderMap;
use reqwest::StatusCode;
use std::time::Duration;

/// Represents an HTTP response
#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status: StatusCode,
    pub headers: HeaderMap,
    pub body: String,
    pub duration: Duration,
}

impl HttpResponse {
    /// Create a response from a reqwest response
    pub fn from_reqwest(response: reqwest::blocking::Response, duration: Duration) -> Result<Self> {
        let status = response.status();
        let headers = response.headers().clone();
        let body = response.text()?;

        Ok(Self {
            status,
            headers,
            body,
            duration,
        })
    }

    /// Check if the response status is successful (2xx)
    pub fn is_success(&self) -> bool {
        self.status.is_success()
    }

    /// Check if the response status is client error (4xx)
    pub fn is_client_error(&self) -> bool {
        self.status.is_client_error()
    }

    /// Check if the response status is server error (5xx)
    pub fn is_server_error(&self) -> bool {
        self.status.is_server_error()
    }

    /// Get status code color for terminal output
    pub fn status_color(&self) -> &str {
        if self.is_success() {
            "green"
        } else if self.is_client_error() || self.is_server_error() {
            "red"
        } else {
            "yellow"
        }
    }

    /// Try to parse body as JSON and pretty-print it
    pub fn pretty_body(&self) -> String {
        if self.body.is_empty() {
            return String::new();
        }

        // Try to parse as JSON
        if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(&self.body) {
            // Pretty print JSON
            if let Ok(pretty_json) = serde_json::to_string_pretty(&json_value) {
                return pretty_json;
            }
        }

        // Return as-is if not JSON
        self.body.clone()
    }

    /// Check if body is JSON
    pub fn is_json(&self) -> bool {
        serde_json::from_str::<serde_json::Value>(&self.body).is_ok()
    }
}

/// Formatter for displaying HTTP responses
pub struct ResponseFormatter;

impl ResponseFormatter {
    /// Format a response for terminal display
    pub fn format(response: &HttpResponse) -> String {
        use colored::*;

        let mut output = String::new();

        // Status line with better colors for both light and dark modes
        let status_str = format!(
            "{} {}",
            response.status.as_str(),
            response.status.canonical_reason().unwrap_or("")
        );

        output.push_str(&format!(
            "{} {}\n",
            "Status:".bold(),
            status_str.color(response.status_color()).bold()
        ));

        // Duration
        output.push_str(&format!("{} {:.2?}\n\n", "Time:".bold(), response.duration));

        // Headers
        if !response.headers.is_empty() {
            output.push_str(&format!("{}\n", "Response Headers:".bold()));
            for (name, value) in response.headers.iter() {
                output.push_str(&format!(
                    "  {}: {}\n",
                    name.as_str().blue().bold(),
                    value.to_str().unwrap_or("<binary>")
                ));
            }
            output.push('\n');
        }

        // Body - no color for better readability in both modes
        if !response.body.is_empty() {
            output.push_str(&format!("{}\n", "Response Body:".bold()));
            let body = response.pretty_body();
            output.push_str(&format!("{}\n\n", body));
        }

        output
    }

    /// Format just the status line
    pub fn format_status(response: &HttpResponse) -> String {
        use colored::*;

        let status_str = format!(
            "{} {}",
            response.status.as_str(),
            response.status.canonical_reason().unwrap_or("")
        );

        format!(
            "{} {}",
            "Status:".bold(),
            status_str.color(response.status_color()).bold()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_color_success() {
        let response = create_mock_response(StatusCode::OK, "test");
        assert_eq!(response.status_color(), "green");
    }

    #[test]
    fn test_status_color_client_error() {
        let response = create_mock_response(StatusCode::NOT_FOUND, "test");
        assert_eq!(response.status_color(), "red");
    }

    #[test]
    fn test_status_color_server_error() {
        let response = create_mock_response(StatusCode::INTERNAL_SERVER_ERROR, "test");
        assert_eq!(response.status_color(), "red");
    }

    #[test]
    fn test_is_json() {
        let response = create_mock_response(StatusCode::OK, r#"{"key":"value"}"#);
        assert!(response.is_json());

        let response = create_mock_response(StatusCode::OK, "plain text");
        assert!(!response.is_json());
    }

    #[test]
    fn test_pretty_body_json() {
        let response = create_mock_response(StatusCode::OK, r#"{"key":"value"}"#);
        let pretty = response.pretty_body();
        assert!(pretty.contains("\"key\""));
        assert!(pretty.contains("\"value\""));
    }

    #[test]
    fn test_pretty_body_plain() {
        let response = create_mock_response(StatusCode::OK, "plain text");
        let pretty = response.pretty_body();
        assert_eq!(pretty, "plain text");
    }

    // Helper function for tests
    fn create_mock_response(status: StatusCode, body: &str) -> HttpResponse {
        HttpResponse {
            status,
            headers: HeaderMap::new(),
            body: body.to_string(),
            duration: Duration::from_millis(100),
        }
    }
}
