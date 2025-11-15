//! Request chain configuration

use crate::workflow::WorkflowStep;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Configuration for chain execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainConfig {
    /// Stop execution on first failure
    pub stop_on_failure: bool,

    /// Delay between requests
    pub delay_between_requests: Option<Duration>,

    /// Maximum total duration
    pub max_duration: Option<Duration>,

    /// Number of iterations
    pub iterations: usize,
}

impl ChainConfig {
    /// Create default config
    pub fn new() -> Self {
        Self {
            stop_on_failure: true,
            delay_between_requests: None,
            max_duration: None,
            iterations: 1,
        }
    }

    /// Set stop on failure
    pub fn with_stop_on_failure(mut self, stop: bool) -> Self {
        self.stop_on_failure = stop;
        self
    }

    /// Set delay between requests
    pub fn with_delay(mut self, delay: Duration) -> Self {
        self.delay_between_requests = Some(delay);
        self
    }

    /// Set max duration
    pub fn with_max_duration(mut self, duration: Duration) -> Self {
        self.max_duration = Some(duration);
        self
    }

    /// Set iterations
    pub fn with_iterations(mut self, iterations: usize) -> Self {
        self.iterations = iterations;
        self
    }
}

impl Default for ChainConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// A chain of requests to execute in sequence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestChain {
    /// Chain name
    pub name: String,

    /// Description
    pub description: Option<String>,

    /// Steps in the chain
    pub steps: Vec<WorkflowStep>,

    /// Execution configuration
    pub config: ChainConfig,
}

impl RequestChain {
    /// Create a new request chain
    pub fn new(name: String) -> Self {
        Self {
            name,
            description: None,
            steps: Vec::new(),
            config: ChainConfig::default(),
        }
    }

    /// Set description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Add a step
    pub fn add_step(mut self, step: WorkflowStep) -> Self {
        self.steps.push(step);
        self
    }

    /// Set config
    pub fn with_config(mut self, config: ChainConfig) -> Self {
        self.config = config;
        self
    }

    /// Get step count
    pub fn step_count(&self) -> usize {
        self.steps.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::HttpMethod;

    #[test]
    fn test_chain_config_new() {
        let config = ChainConfig::new();
        assert!(config.stop_on_failure);
        assert_eq!(config.iterations, 1);
    }

    #[test]
    fn test_chain_config_with_delay() {
        let config = ChainConfig::new().with_delay(Duration::from_secs(1));
        assert_eq!(
            config.delay_between_requests,
            Some(Duration::from_secs(1))
        );
    }

    #[test]
    fn test_chain_config_with_iterations() {
        let config = ChainConfig::new().with_iterations(5);
        assert_eq!(config.iterations, 5);
    }

    #[test]
    fn test_request_chain_new() {
        let chain = RequestChain::new("Test Chain".to_string());
        assert_eq!(chain.name, "Test Chain");
        assert_eq!(chain.step_count(), 0);
    }

    #[test]
    fn test_request_chain_with_description() {
        let chain = RequestChain::new("Test".to_string())
            .with_description("A test chain".to_string());
        assert_eq!(chain.description, Some("A test chain".to_string()));
    }

    #[test]
    fn test_request_chain_add_step() {
        let step = WorkflowStep::new(
            "Step 1".to_string(),
            HttpMethod::Get,
            "https://example.com".to_string(),
        );

        let chain = RequestChain::new("Test".to_string()).add_step(step);
        assert_eq!(chain.step_count(), 1);
    }

    #[test]
    fn test_request_chain_multiple_steps() {
        let step1 = WorkflowStep::new(
            "Login".to_string(),
            HttpMethod::Post,
            "https://api.example.com/login".to_string(),
        );
        let step2 = WorkflowStep::new(
            "Get Data".to_string(),
            HttpMethod::Get,
            "https://api.example.com/data".to_string(),
        );

        let chain = RequestChain::new("API Test".to_string())
            .add_step(step1)
            .add_step(step2);

        assert_eq!(chain.step_count(), 2);
    }

    #[test]
    fn test_request_chain_with_config() {
        let config = ChainConfig::new()
            .with_stop_on_failure(false)
            .with_iterations(3);

        let chain = RequestChain::new("Test".to_string()).with_config(config);

        assert!(!chain.config.stop_on_failure);
        assert_eq!(chain.config.iterations, 3);
    }

    #[test]
    fn test_chain_serialization() {
        let chain = RequestChain::new("Test".to_string())
            .with_description("Test chain".to_string());

        let json = serde_json::to_string(&chain).unwrap();
        let deserialized: RequestChain = serde_json::from_str(&json).unwrap();

        assert_eq!(chain.name, deserialized.name);
        assert_eq!(chain.description, deserialized.description);
    }
}
