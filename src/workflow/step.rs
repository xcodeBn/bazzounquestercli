//! Workflow step definition

use crate::assertions::Assertion;
use crate::http::{HttpMethod, HttpResponse};
use crate::scripts::Script;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// A single step in a workflow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStep {
    /// Step name
    pub name: String,

    /// HTTP method
    pub method: HttpMethod,

    /// URL (can contain variables)
    pub url: String,

    /// Headers
    pub headers: Vec<String>,

    /// Query parameters
    pub query_params: Vec<String>,

    /// Request body
    pub body: Option<String>,

    /// Pre-request script
    pub pre_request_script: Option<Script>,

    /// Post-response script
    pub post_response_script: Option<Script>,

    /// Assertions to validate
    pub assertions: Vec<Assertion>,

    /// Whether to continue on failure
    pub continue_on_error: bool,

    /// Timeout for this step
    pub timeout: Option<Duration>,

    /// Variables to extract from response
    pub extract_variables: HashMap<String, String>,
}

impl WorkflowStep {
    /// Create a new workflow step
    pub fn new(name: String, method: HttpMethod, url: String) -> Self {
        Self {
            name,
            method,
            url,
            headers: Vec::new(),
            query_params: Vec::new(),
            body: None,
            pre_request_script: None,
            post_response_script: None,
            assertions: Vec::new(),
            continue_on_error: false,
            timeout: None,
            extract_variables: HashMap::new(),
        }
    }

    /// Add a header
    pub fn with_header(mut self, header: String) -> Self {
        self.headers.push(header);
        self
    }

    /// Add query parameter
    pub fn with_query(mut self, param: String) -> Self {
        self.query_params.push(param);
        self
    }

    /// Set body
    pub fn with_body(mut self, body: String) -> Self {
        self.body = Some(body);
        self
    }

    /// Set pre-request script
    pub fn with_pre_request_script(mut self, script: Script) -> Self {
        self.pre_request_script = Some(script);
        self
    }

    /// Set post-response script
    pub fn with_post_response_script(mut self, script: Script) -> Self {
        self.post_response_script = Some(script);
        self
    }

    /// Add assertion
    pub fn with_assertion(mut self, assertion: Assertion) -> Self {
        self.assertions.push(assertion);
        self
    }

    /// Set continue on error
    pub fn with_continue_on_error(mut self, continue_on_error: bool) -> Self {
        self.continue_on_error = continue_on_error;
        self
    }

    /// Set timeout
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Extract variable from response
    pub fn extract_variable(mut self, var_name: String, json_path: String) -> Self {
        self.extract_variables.insert(var_name, json_path);
        self
    }
}

/// Result of executing a workflow step
#[derive(Debug, Clone)]
pub struct StepResult {
    /// Step name
    pub step_name: String,

    /// Whether step succeeded
    pub success: bool,

    /// HTTP response (if request was made)
    pub response: Option<HttpResponse>,

    /// Error message (if failed)
    pub error: Option<String>,

    /// Extracted variables
    pub extracted_variables: HashMap<String, String>,

    /// Execution duration
    pub duration: Duration,
}

impl StepResult {
    /// Create a success result
    pub fn success(
        step_name: String,
        response: HttpResponse,
        extracted_variables: HashMap<String, String>,
        duration: Duration,
    ) -> Self {
        Self {
            step_name,
            success: true,
            response: Some(response),
            error: None,
            extracted_variables,
            duration,
        }
    }

    /// Create a failure result
    pub fn failure(step_name: String, error: String, duration: Duration) -> Self {
        Self {
            step_name,
            success: false,
            response: None,
            error: Some(error),
            extracted_variables: HashMap::new(),
            duration,
        }
    }

    /// Get summary
    pub fn summary(&self) -> String {
        if self.success {
            format!("✓ {} - {:?}", self.step_name, self.duration)
        } else {
            format!(
                "✗ {} - {} ({:?})",
                self.step_name,
                self.error.as_ref().unwrap_or(&"Unknown error".to_string()),
                self.duration
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workflow_step_new() {
        let step = WorkflowStep::new(
            "Test Step".to_string(),
            HttpMethod::Get,
            "https://api.example.com".to_string(),
        );

        assert_eq!(step.name, "Test Step");
        assert_eq!(step.method, HttpMethod::Get);
        assert_eq!(step.url, "https://api.example.com");
        assert!(!step.continue_on_error);
    }

    #[test]
    fn test_workflow_step_with_header() {
        let step = WorkflowStep::new(
            "Test".to_string(),
            HttpMethod::Get,
            "https://example.com".to_string(),
        )
        .with_header("Authorization:Bearer token".to_string());

        assert_eq!(step.headers.len(), 1);
    }

    #[test]
    fn test_workflow_step_with_body() {
        let step = WorkflowStep::new(
            "Test".to_string(),
            HttpMethod::Post,
            "https://example.com".to_string(),
        )
        .with_body(r#"{"test":"data"}"#.to_string());

        assert!(step.body.is_some());
        assert_eq!(step.body.unwrap(), r#"{"test":"data"}"#);
    }

    #[test]
    fn test_workflow_step_extract_variable() {
        let step = WorkflowStep::new(
            "Test".to_string(),
            HttpMethod::Get,
            "https://example.com".to_string(),
        )
        .extract_variable("token".to_string(), "$.access_token".to_string());

        assert_eq!(step.extract_variables.len(), 1);
        assert_eq!(
            step.extract_variables.get("token"),
            Some(&"$.access_token".to_string())
        );
    }

    #[test]
    fn test_workflow_step_with_continue_on_error() {
        let step = WorkflowStep::new(
            "Test".to_string(),
            HttpMethod::Get,
            "https://example.com".to_string(),
        )
        .with_continue_on_error(true);

        assert!(step.continue_on_error);
    }

    #[test]
    fn test_step_result_success() {
        use reqwest::{header::HeaderMap, StatusCode};
        let response = HttpResponse {
            status: StatusCode::OK,
            headers: HeaderMap::new(),
            body: "success".to_string(),
            duration: Duration::from_millis(100),
        };

        let result = StepResult::success(
            "Test".to_string(),
            response,
            HashMap::new(),
            Duration::from_millis(150),
        );

        assert!(result.success);
        assert!(result.response.is_some());
        assert!(result.error.is_none());
    }

    #[test]
    fn test_step_result_failure() {
        let result = StepResult::failure(
            "Test".to_string(),
            "Connection failed".to_string(),
            Duration::from_millis(50),
        );

        assert!(!result.success);
        assert!(result.response.is_none());
        assert_eq!(result.error, Some("Connection failed".to_string()));
    }

    #[test]
    fn test_step_result_summary() {
        let result = StepResult::failure(
            "Login".to_string(),
            "401 Unauthorized".to_string(),
            Duration::from_millis(200),
        );

        let summary = result.summary();
        assert!(summary.contains("✗"));
        assert!(summary.contains("Login"));
        assert!(summary.contains("401"));
    }
}
