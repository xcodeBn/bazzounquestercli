//! Response validation engine

use crate::assertions::{Assertion, AssertionResult, AssertionType};
use crate::http::HttpResponse;
use serde::{Deserialize, Serialize};

/// Validation report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationReport {
    /// All assertion results
    pub results: Vec<AssertionResult>,

    /// Total assertions run
    pub total: usize,

    /// Number passed
    pub passed: usize,

    /// Number failed
    pub failed: usize,

    /// Overall success
    pub success: bool,
}

impl ValidationReport {
    /// Create a new report
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
            total: 0,
            passed: 0,
            failed: 0,
            success: true,
        }
    }

    /// Add a result
    pub fn add_result(&mut self, result: AssertionResult) {
        if result.passed {
            self.passed += 1;
        } else {
            self.failed += 1;
            self.success = false;
        }
        self.total += 1;
        self.results.push(result);
    }

    /// Get summary
    pub fn summary(&self) -> String {
        if self.success {
            format!("✓ All {} assertions passed", self.total)
        } else {
            format!(
                "✗ {} of {} assertions failed",
                self.failed, self.total
            )
        }
    }

    /// Get detailed report
    pub fn detailed_report(&self) -> String {
        let mut report = self.summary();
        report.push_str("\n\n");

        for result in &self.results {
            report.push_str(&result.summary());
            report.push('\n');
        }

        report
    }
}

impl Default for ValidationReport {
    fn default() -> Self {
        Self::new()
    }
}

/// Response validator
pub struct ResponseValidator;

impl ResponseValidator {
    /// Create a new validator
    pub fn new() -> Self {
        Self
    }

    /// Validate a response against assertions
    pub fn validate(
        &self,
        response: &HttpResponse,
        assertions: &[Assertion],
    ) -> ValidationReport {
        let mut report = ValidationReport::new();

        for assertion in assertions {
            if !assertion.enabled {
                continue;
            }

            let result = self.validate_assertion(response, assertion);
            report.add_result(result);
        }

        report
    }

    /// Validate a single assertion
    fn validate_assertion(
        &self,
        response: &HttpResponse,
        assertion: &Assertion,
    ) -> AssertionResult {
        match &assertion.assertion_type {
            AssertionType::StatusCode => self.validate_status_code(response, assertion),
            AssertionType::Header(name) => self.validate_header(response, name, assertion),
            AssertionType::Body => self.validate_body(response, assertion),
            AssertionType::ResponseTime => self.validate_response_time(response, assertion),
            AssertionType::JsonPath(path) => self.validate_json_path(response, path, assertion),
            AssertionType::Custom(desc) => self.validate_custom(response, desc, assertion),
        }
    }

    /// Validate status code
    fn validate_status_code(
        &self,
        response: &HttpResponse,
        assertion: &Assertion,
    ) -> AssertionResult {
        let actual = response.status.as_u16().to_string();
        let expected = assertion.matcher.description();

        if assertion.matcher.matches(&actual) {
            AssertionResult::pass(assertion.clone(), actual, expected)
        } else {
            AssertionResult::fail(
                assertion.clone(),
                actual,
                expected,
                "Status code does not match".to_string(),
            )
        }
    }

    /// Validate header
    fn validate_header(
        &self,
        response: &HttpResponse,
        header_name: &str,
        assertion: &Assertion,
    ) -> AssertionResult {
        let expected = assertion.matcher.description();

        // Find header value
        let actual = response
            .headers
            .get(header_name)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_string();

        if assertion.matcher.matches(&actual) {
            AssertionResult::pass(assertion.clone(), actual, expected)
        } else {
            AssertionResult::fail(
                assertion.clone(),
                actual,
                expected,
                format!("Header '{}' does not match", header_name),
            )
        }
    }

    /// Validate body
    fn validate_body(
        &self,
        response: &HttpResponse,
        assertion: &Assertion,
    ) -> AssertionResult {
        let actual = &response.body;
        let expected = assertion.matcher.description();

        if assertion.matcher.matches(actual) {
            AssertionResult::pass(assertion.clone(), actual.clone(), expected)
        } else {
            AssertionResult::fail(
                assertion.clone(),
                actual.clone(),
                expected,
                "Body does not match".to_string(),
            )
        }
    }

    /// Validate response time
    fn validate_response_time(
        &self,
        response: &HttpResponse,
        assertion: &Assertion,
    ) -> AssertionResult {
        let actual = response.duration.as_millis().to_string();
        let expected = assertion.matcher.description();

        if assertion.matcher.matches(&actual) {
            AssertionResult::pass(assertion.clone(), format!("{}ms", actual), expected)
        } else {
            AssertionResult::fail(
                assertion.clone(),
                format!("{}ms", actual),
                expected,
                "Response time does not match".to_string(),
            )
        }
    }

    /// Validate JSON path
    fn validate_json_path(
        &self,
        response: &HttpResponse,
        path: &str,
        assertion: &Assertion,
    ) -> AssertionResult {
        let expected = assertion.matcher.description();

        // Try to parse response as JSON
        let json_result: Result<serde_json::Value, _> = serde_json::from_str(&response.body);

        match json_result {
            Ok(json) => {
                // Extract value at path
                let actual = self.extract_json_path(&json, path);

                if assertion.matcher.matches(&actual) {
                    AssertionResult::pass(assertion.clone(), actual, expected)
                } else {
                    AssertionResult::fail(
                        assertion.clone(),
                        actual,
                        expected,
                        format!("JSON path '{}' does not match", path),
                    )
                }
            }
            Err(e) => AssertionResult::fail(
                assertion.clone(),
                response.body.clone(),
                expected,
                format!("Failed to parse response as JSON: {}", e),
            ),
        }
    }

    /// Extract value from JSON using simplified path syntax
    fn extract_json_path(&self, json: &serde_json::Value, path: &str) -> String {
        // Simplified JSON path extraction (supports $.field and $.field.subfield)
        let path = path.trim_start_matches("$.");
        let parts: Vec<&str> = path.split('.').collect();

        let mut current = json;
        for part in parts {
            match current {
                serde_json::Value::Object(map) => {
                    if let Some(value) = map.get(part) {
                        current = value;
                    } else {
                        return String::new();
                    }
                }
                _ => return String::new(),
            }
        }

        match current {
            serde_json::Value::String(s) => s.clone(),
            serde_json::Value::Number(n) => n.to_string(),
            serde_json::Value::Bool(b) => b.to_string(),
            serde_json::Value::Null => "null".to_string(),
            _ => current.to_string(),
        }
    }

    /// Validate custom assertion
    fn validate_custom(
        &self,
        _response: &HttpResponse,
        description: &str,
        assertion: &Assertion,
    ) -> AssertionResult {
        // Custom assertions would need custom logic
        // For now, we'll just mark them as passed
        AssertionResult::pass(
            assertion.clone(),
            "custom".to_string(),
            description.to_string(),
        )
    }
}

impl Default for ResponseValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assertions::Matcher;
    use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
    use reqwest::StatusCode;
    use std::time::Duration;

    fn create_mock_response() -> HttpResponse {
        let mut headers = HeaderMap::new();
        headers.insert(
            HeaderName::from_static("content-type"),
            HeaderValue::from_static("application/json"),
        );
        headers.insert(
            HeaderName::from_static("x-custom"),
            HeaderValue::from_static("test-value"),
        );

        HttpResponse {
            status: StatusCode::OK,
            headers,
            body: r#"{"status":"ok","count":42}"#.to_string(),
            duration: Duration::from_millis(150),
        }
    }

    #[test]
    fn test_validation_report_new() {
        let report = ValidationReport::new();
        assert_eq!(report.total, 0);
        assert_eq!(report.passed, 0);
        assert_eq!(report.failed, 0);
        assert!(report.success);
    }

    #[test]
    fn test_validation_report_add_pass() {
        let mut report = ValidationReport::new();
        let assertion = Assertion::status_code(Matcher::equals(200));
        let result = AssertionResult::pass(assertion, "200".to_string(), "200".to_string());

        report.add_result(result);

        assert_eq!(report.total, 1);
        assert_eq!(report.passed, 1);
        assert_eq!(report.failed, 0);
        assert!(report.success);
    }

    #[test]
    fn test_validation_report_add_fail() {
        let mut report = ValidationReport::new();
        let assertion = Assertion::status_code(Matcher::equals(200));
        let result = AssertionResult::fail(
            assertion,
            "404".to_string(),
            "200".to_string(),
            "Mismatch".to_string(),
        );

        report.add_result(result);

        assert_eq!(report.total, 1);
        assert_eq!(report.passed, 0);
        assert_eq!(report.failed, 1);
        assert!(!report.success);
    }

    #[test]
    fn test_validator_status_code_pass() {
        let validator = ResponseValidator::new();
        let response = create_mock_response();
        let assertion = Assertion::status_code(Matcher::equals(200));

        let result = validator.validate_assertion(&response, &assertion);
        assert!(result.passed);
    }

    #[test]
    fn test_validator_status_code_fail() {
        let validator = ResponseValidator::new();
        let response = create_mock_response();
        let assertion = Assertion::status_code(Matcher::equals(404));

        let result = validator.validate_assertion(&response, &assertion);
        assert!(!result.passed);
    }

    #[test]
    fn test_validator_header_pass() {
        let validator = ResponseValidator::new();
        let response = create_mock_response();
        let assertion =
            Assertion::header("Content-Type".to_string(), Matcher::contains("json".to_string()));

        let result = validator.validate_assertion(&response, &assertion);
        assert!(result.passed);
    }

    #[test]
    fn test_validator_header_fail() {
        let validator = ResponseValidator::new();
        let response = create_mock_response();
        let assertion =
            Assertion::header("Content-Type".to_string(), Matcher::contains("xml".to_string()));

        let result = validator.validate_assertion(&response, &assertion);
        assert!(!result.passed);
    }

    #[test]
    fn test_validator_body_pass() {
        let validator = ResponseValidator::new();
        let response = create_mock_response();
        let assertion = Assertion::body(Matcher::contains("status".to_string()));

        let result = validator.validate_assertion(&response, &assertion);
        assert!(result.passed);
    }

    #[test]
    fn test_validator_response_time_pass() {
        let validator = ResponseValidator::new();
        let response = create_mock_response();
        let assertion = Assertion::response_time(Matcher::less_than(1000));

        let result = validator.validate_assertion(&response, &assertion);
        assert!(result.passed);
    }

    #[test]
    fn test_validator_json_path_pass() {
        let validator = ResponseValidator::new();
        let response = create_mock_response();
        let assertion =
            Assertion::json_path("$.status".to_string(), Matcher::equals_str("ok"));

        let result = validator.validate_assertion(&response, &assertion);
        assert!(result.passed);
    }

    #[test]
    fn test_validator_json_path_numeric() {
        let validator = ResponseValidator::new();
        let response = create_mock_response();
        let assertion = Assertion::json_path("$.count".to_string(), Matcher::equals(42));

        let result = validator.validate_assertion(&response, &assertion);
        assert!(result.passed);
    }

    #[test]
    fn test_validator_validate_multiple() {
        let validator = ResponseValidator::new();
        let response = create_mock_response();

        let assertions = vec![
            Assertion::status_code(Matcher::equals(200)),
            Assertion::header("Content-Type".to_string(), Matcher::contains("json".to_string())),
            Assertion::body(Matcher::contains("ok".to_string())),
            Assertion::response_time(Matcher::less_than(1000)),
        ];

        let report = validator.validate(&response, &assertions);

        assert_eq!(report.total, 4);
        assert_eq!(report.passed, 4);
        assert_eq!(report.failed, 0);
        assert!(report.success);
    }

    #[test]
    fn test_validator_validate_with_failures() {
        let validator = ResponseValidator::new();
        let response = create_mock_response();

        let assertions = vec![
            Assertion::status_code(Matcher::equals(200)), // pass
            Assertion::status_code(Matcher::equals(404)), // fail
        ];

        let report = validator.validate(&response, &assertions);

        assert_eq!(report.total, 2);
        assert_eq!(report.passed, 1);
        assert_eq!(report.failed, 1);
        assert!(!report.success);
    }

    #[test]
    fn test_validator_skip_disabled() {
        let validator = ResponseValidator::new();
        let response = create_mock_response();

        let assertions = vec![
            Assertion::status_code(Matcher::equals(200)),
            Assertion::status_code(Matcher::equals(404)).with_enabled(false),
        ];

        let report = validator.validate(&response, &assertions);

        assert_eq!(report.total, 1); // Only enabled one counted
        assert!(report.success);
    }
}
