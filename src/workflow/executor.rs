//! Workflow execution engine

use crate::assertions::validate_response;
use crate::env::VariableSubstitutor;
use crate::error::{Error, Result};
use crate::http::{HttpClient, HttpMethod, RequestBuilder};
use crate::scripts::{execute_post_response, execute_pre_request, ScriptContext};
use crate::workflow::{RequestChain, StepResult, WorkflowStep};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Result of executing a workflow
#[derive(Debug, Clone)]
pub struct ExecutionResult {
    /// Chain name
    pub chain_name: String,

    /// Step results
    pub step_results: Vec<StepResult>,

    /// Overall success
    pub success: bool,

    /// Total duration
    pub total_duration: Duration,

    /// Variables at end of execution
    pub final_variables: HashMap<String, String>,
}

impl ExecutionResult {
    /// Create a new execution result
    pub fn new(chain_name: String) -> Self {
        Self {
            chain_name,
            step_results: Vec::new(),
            success: true,
            total_duration: Duration::ZERO,
            final_variables: HashMap::new(),
        }
    }

    /// Add step result
    pub fn add_step_result(&mut self, result: StepResult) {
        if !result.success {
            self.success = false;
        }
        self.total_duration += result.duration;
        self.step_results.push(result);
    }

    /// Get summary
    pub fn summary(&self) -> String {
        let passed = self.step_results.iter().filter(|r| r.success).count();
        let failed = self.step_results.len() - passed;

        if self.success {
            format!(
                "✓ Chain '{}' completed successfully: {} steps, {:?}",
                self.chain_name,
                self.step_results.len(),
                self.total_duration
            )
        } else {
            format!(
                "✗ Chain '{}' failed: {} passed, {} failed, {:?}",
                self.chain_name, passed, failed, self.total_duration
            )
        }
    }

    /// Get detailed report
    pub fn detailed_report(&self) -> String {
        let mut report = self.summary();
        report.push_str("\n\n");

        for (i, result) in self.step_results.iter().enumerate() {
            report.push_str(&format!("{}. {}\n", i + 1, result.summary()));
        }

        report
    }
}

/// Workflow executor
pub struct WorkflowExecutor {
    /// HTTP client
    client: HttpClient,

    /// Variable substitutor
    substitutor: VariableSubstitutor,
}

impl WorkflowExecutor {
    /// Create a new executor
    pub fn new() -> Self {
        Self {
            client: HttpClient::new(),
            substitutor: VariableSubstitutor::new(),
        }
    }

    /// Execute a request chain
    pub fn execute(&self, chain: &RequestChain) -> Result<ExecutionResult> {
        let mut result = ExecutionResult::new(chain.name.clone());
        let mut context = ScriptContext::new();

        // Run for configured iterations
        for iteration in 0..chain.config.iterations {
            if iteration > 0 {
                // Apply delay between iterations
                if let Some(delay) = chain.config.delay_between_requests {
                    std::thread::sleep(delay);
                }
            }

            // Execute each step
            for step in &chain.steps {
                let step_start = Instant::now();

                match self.execute_step(step, &mut context) {
                    Ok(step_result) => {
                        result.add_step_result(step_result.clone());

                        // Check if we should stop on failure
                        if !step_result.success
                            && chain.config.stop_on_failure
                            && !step.continue_on_error
                        {
                            break;
                        }
                    }
                    Err(e) => {
                        let step_result = StepResult::failure(
                            step.name.clone(),
                            e.to_string(),
                            step_start.elapsed(),
                        );
                        result.add_step_result(step_result);

                        if chain.config.stop_on_failure && !step.continue_on_error {
                            break;
                        }
                    }
                }
            }

            // Check max duration
            if let Some(max_duration) = chain.config.max_duration {
                if result.total_duration >= max_duration {
                    break;
                }
            }
        }

        // Extract final variables
        for (name, var) in context.variables() {
            result
                .final_variables
                .insert(name.clone(), var.value.clone());
        }

        Ok(result)
    }

    /// Execute a single step
    fn execute_step(
        &self,
        step: &WorkflowStep,
        context: &mut ScriptContext,
    ) -> Result<StepResult> {
        let step_start = Instant::now();

        // Execute pre-request script
        if let Some(ref script) = step.pre_request_script {
            execute_pre_request(script, context)?;
        }

        // Build request with variable substitution
        let mut variables = HashMap::new();
        for (name, var) in context.variables() {
            variables.insert(name.as_str(), var.value.as_str());
        }

        let url = self.substitutor.substitute(&step.url, &variables);
        let mut request = RequestBuilder::new(step.method, url);

        // Substitute headers
        for header in &step.headers {
            let substituted = self.substitutor.substitute(header, &variables);
            request = request.header(substituted);
        }

        // Substitute query params
        for param in &step.query_params {
            let substituted = self.substitutor.substitute(param, &variables);
            request = request.query(substituted);
        }

        // Substitute body
        if let Some(ref body) = step.body {
            let substituted = self.substitutor.substitute(body, &variables);
            request = request.body(substituted);
        }

        // Execute request
        let response = self.client.execute(&request)?;

        // Store response data in context
        context.set_response_data("status".to_string(), response.status.as_u16().to_string());
        context.set_response_data("body".to_string(), response.body.clone());

        // Execute post-response script
        if let Some(ref script) = step.post_response_script {
            execute_post_response(script, context)?;
        }

        // Validate assertions
        if !step.assertions.is_empty() {
            let validation_report = validate_response(&response, &step.assertions)?;
            if !validation_report.success {
                return Ok(StepResult::failure(
                    step.name.clone(),
                    format!("Assertions failed: {}", validation_report.summary()),
                    step_start.elapsed(),
                ));
            }
        }

        // Extract variables from response
        let mut extracted = HashMap::new();
        for (var_name, json_path) in &step.extract_variables {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&response.body) {
                let value = self.extract_json_value(&json, json_path);
                context.set_variable(var_name.clone(), value.clone());
                extracted.insert(var_name.clone(), value);
            }
        }

        Ok(StepResult::success(
            step.name.clone(),
            response,
            extracted,
            step_start.elapsed(),
        ))
    }

    /// Extract value from JSON using simplified path
    fn extract_json_value(&self, json: &serde_json::Value, path: &str) -> String {
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
}

impl Default for WorkflowExecutor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execution_result_new() {
        let result = ExecutionResult::new("Test Chain".to_string());
        assert_eq!(result.chain_name, "Test Chain");
        assert!(result.success);
        assert_eq!(result.step_results.len(), 0);
    }

    #[test]
    fn test_execution_result_add_success() {
        use reqwest::{header::HeaderMap, StatusCode};
        let mut result = ExecutionResult::new("Test".to_string());

        let response = crate::http::HttpResponse {
            status: StatusCode::OK,
            headers: HeaderMap::new(),
            body: "ok".to_string(),
            duration: Duration::from_millis(100),
        };

        let step_result = StepResult::success(
            "Step1".to_string(),
            response,
            HashMap::new(),
            Duration::from_millis(150),
        );

        result.add_step_result(step_result);

        assert!(result.success);
        assert_eq!(result.step_results.len(), 1);
    }

    #[test]
    fn test_execution_result_add_failure() {
        let mut result = ExecutionResult::new("Test".to_string());

        let step_result = StepResult::failure(
            "Step1".to_string(),
            "Failed".to_string(),
            Duration::from_millis(50),
        );

        result.add_step_result(step_result);

        assert!(!result.success);
        assert_eq!(result.step_results.len(), 1);
    }

    #[test]
    fn test_executor_creation() {
        let _executor = WorkflowExecutor::new();
    }

    #[test]
    fn test_executor_extract_json_value() {
        let executor = WorkflowExecutor::new();
        let json: serde_json::Value =
            serde_json::from_str(r#"{"user":{"name":"Alice","id":123}}"#).unwrap();

        assert_eq!(executor.extract_json_value(&json, "$.user.name"), "Alice");
        assert_eq!(executor.extract_json_value(&json, "$.user.id"), "123");
    }
}
