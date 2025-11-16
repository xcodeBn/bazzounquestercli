//! Assertion definitions and results

use crate::assertions::matcher::Matcher;
use serde::{Deserialize, Serialize};

/// Type of assertion
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AssertionType {
    /// Assert on response status code
    StatusCode,

    /// Assert on response header
    Header(String),

    /// Assert on response body
    Body,

    /// Assert on response time
    ResponseTime,

    /// Assert on JSON path value
    JsonPath(String),

    /// Custom assertion with description
    Custom(String),
}

/// An assertion to validate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Assertion {
    /// Type of assertion
    pub assertion_type: AssertionType,

    /// Matcher to use
    pub matcher: Matcher,

    /// Description (optional)
    pub description: Option<String>,

    /// Whether assertion is enabled
    pub enabled: bool,
}

impl Assertion {
    /// Create a new assertion
    pub fn new(assertion_type: AssertionType, matcher: Matcher) -> Self {
        Self {
            assertion_type,
            matcher,
            description: None,
            enabled: true,
        }
    }

    /// Assert status code
    pub fn status_code(matcher: Matcher) -> Self {
        Self::new(AssertionType::StatusCode, matcher)
    }

    /// Assert header value
    pub fn header(header_name: String, matcher: Matcher) -> Self {
        Self::new(AssertionType::Header(header_name), matcher)
    }

    /// Assert body
    pub fn body(matcher: Matcher) -> Self {
        Self::new(AssertionType::Body, matcher)
    }

    /// Assert response time
    pub fn response_time(matcher: Matcher) -> Self {
        Self::new(AssertionType::ResponseTime, matcher)
    }

    /// Assert JSON path
    pub fn json_path(path: String, matcher: Matcher) -> Self {
        Self::new(AssertionType::JsonPath(path), matcher)
    }

    /// Set description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Set enabled
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

/// Result of an assertion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssertionResult {
    /// The assertion that was run
    pub assertion: Assertion,

    /// Whether assertion passed
    pub passed: bool,

    /// Actual value that was tested
    pub actual_value: String,

    /// Expected value
    pub expected_value: String,

    /// Error message (if failed)
    pub error_message: Option<String>,
}

impl AssertionResult {
    /// Create a passing result
    pub fn pass(assertion: Assertion, actual: String, expected: String) -> Self {
        Self {
            assertion,
            passed: true,
            actual_value: actual,
            expected_value: expected,
            error_message: None,
        }
    }

    /// Create a failing result
    pub fn fail(assertion: Assertion, actual: String, expected: String, error: String) -> Self {
        Self {
            assertion,
            passed: false,
            actual_value: actual,
            expected_value: expected,
            error_message: Some(error),
        }
    }

    /// Get a summary of the result
    pub fn summary(&self) -> String {
        let desc = self.assertion.description.as_deref().unwrap_or("Assertion");

        if self.passed {
            format!("✓ {}: PASS", desc)
        } else {
            format!(
                "✗ {}: FAIL - Expected {}, got {}",
                desc, self.expected_value, self.actual_value
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assertion_status_code() {
        let assertion = Assertion::status_code(Matcher::equals(200));
        assert_eq!(assertion.assertion_type, AssertionType::StatusCode);
        assert!(assertion.enabled);
    }

    #[test]
    fn test_assertion_header() {
        let assertion = Assertion::header(
            "Content-Type".to_string(),
            Matcher::contains("json".to_string()),
        );
        match assertion.assertion_type {
            AssertionType::Header(ref name) => assert_eq!(name, "Content-Type"),
            _ => panic!("Wrong assertion type"),
        }
    }

    #[test]
    fn test_assertion_body() {
        let assertion = Assertion::body(Matcher::contains("success".to_string()));
        assert_eq!(assertion.assertion_type, AssertionType::Body);
    }

    #[test]
    fn test_assertion_response_time() {
        let assertion = Assertion::response_time(Matcher::less_than(1000));
        assert_eq!(assertion.assertion_type, AssertionType::ResponseTime);
    }

    #[test]
    fn test_assertion_json_path() {
        let assertion = Assertion::json_path("$.status".to_string(), Matcher::equals_str("ok"));
        match assertion.assertion_type {
            AssertionType::JsonPath(ref path) => assert_eq!(path, "$.status"),
            _ => panic!("Wrong assertion type"),
        }
    }

    #[test]
    fn test_assertion_with_description() {
        let assertion = Assertion::status_code(Matcher::equals(200))
            .with_description("Status should be 200 OK".to_string());

        assert_eq!(
            assertion.description,
            Some("Status should be 200 OK".to_string())
        );
    }

    #[test]
    fn test_assertion_with_enabled() {
        let assertion = Assertion::status_code(Matcher::equals(200)).with_enabled(false);
        assert!(!assertion.enabled);
    }

    #[test]
    fn test_assertion_result_pass() {
        let assertion = Assertion::status_code(Matcher::equals(200));
        let result = AssertionResult::pass(assertion, "200".to_string(), "200".to_string());

        assert!(result.passed);
        assert!(result.error_message.is_none());
    }

    #[test]
    fn test_assertion_result_fail() {
        let assertion = Assertion::status_code(Matcher::equals(200));
        let result = AssertionResult::fail(
            assertion,
            "404".to_string(),
            "200".to_string(),
            "Status code mismatch".to_string(),
        );

        assert!(!result.passed);
        assert!(result.error_message.is_some());
    }

    #[test]
    fn test_assertion_result_summary_pass() {
        let assertion = Assertion::status_code(Matcher::equals(200))
            .with_description("Check status".to_string());
        let result = AssertionResult::pass(assertion, "200".to_string(), "200".to_string());

        let summary = result.summary();
        assert!(summary.contains("✓"));
        assert!(summary.contains("PASS"));
    }

    #[test]
    fn test_assertion_result_summary_fail() {
        let assertion = Assertion::status_code(Matcher::equals(200))
            .with_description("Check status".to_string());
        let result = AssertionResult::fail(
            assertion,
            "404".to_string(),
            "200".to_string(),
            "Mismatch".to_string(),
        );

        let summary = result.summary();
        assert!(summary.contains("✗"));
        assert!(summary.contains("FAIL"));
        assert!(summary.contains("404"));
    }

    #[test]
    fn test_assertion_serialization() {
        let assertion =
            Assertion::status_code(Matcher::equals(200)).with_description("Test".to_string());

        let json = serde_json::to_string(&assertion).unwrap();
        let deserialized: Assertion = serde_json::from_str(&json).unwrap();

        assert_eq!(assertion.assertion_type, deserialized.assertion_type);
        assert_eq!(assertion.description, deserialized.description);
    }
}
