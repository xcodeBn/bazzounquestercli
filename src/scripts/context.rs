//! Script execution context

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A variable in the script context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptVariable {
    /// Variable value
    pub value: String,

    /// Whether this is a secret variable
    pub is_secret: bool,
}

impl ScriptVariable {
    /// Create a new variable
    pub fn new(value: String) -> Self {
        Self {
            value,
            is_secret: false,
        }
    }

    /// Create a secret variable
    pub fn secret(value: String) -> Self {
        Self {
            value,
            is_secret: true,
        }
    }
}

/// Script execution context
#[derive(Debug, Clone)]
pub struct ScriptContext {
    /// Variables available to scripts
    variables: HashMap<String, ScriptVariable>,

    /// Request data (method, URL, headers, body)
    request_data: HashMap<String, String>,

    /// Response data (status, headers, body)
    response_data: HashMap<String, String>,

    /// Script output/console logs
    console_output: Vec<String>,
}

impl ScriptContext {
    /// Create a new script context
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            request_data: HashMap::new(),
            response_data: HashMap::new(),
            console_output: Vec::new(),
        }
    }

    /// Set a variable
    pub fn set_variable(&mut self, name: String, value: String) {
        self.variables.insert(name, ScriptVariable::new(value));
    }

    /// Set a secret variable
    pub fn set_secret_variable(&mut self, name: String, value: String) {
        self.variables.insert(name, ScriptVariable::secret(value));
    }

    /// Get a variable
    pub fn get_variable(&self, name: &str) -> Option<&ScriptVariable> {
        self.variables.get(name)
    }

    /// Get variable value
    pub fn get_variable_value(&self, name: &str) -> Option<&str> {
        self.variables.get(name).map(|v| v.value.as_str())
    }

    /// Remove a variable
    pub fn remove_variable(&mut self, name: &str) -> Option<ScriptVariable> {
        self.variables.remove(name)
    }

    /// Get all variables
    pub fn variables(&self) -> &HashMap<String, ScriptVariable> {
        &self.variables
    }

    /// Set request data
    pub fn set_request_data(&mut self, key: String, value: String) {
        self.request_data.insert(key, value);
    }

    /// Get request data
    pub fn get_request_data(&self, key: &str) -> Option<&str> {
        self.request_data.get(key).map(|s| s.as_str())
    }

    /// Set response data
    pub fn set_response_data(&mut self, key: String, value: String) {
        self.response_data.insert(key, value);
    }

    /// Get response data
    pub fn get_response_data(&self, key: &str) -> Option<&str> {
        self.response_data.get(key).map(|s| s.as_str())
    }

    /// Add console output
    pub fn console_log(&mut self, message: String) {
        self.console_output.push(message);
    }

    /// Get console output
    pub fn console_output(&self) -> &[String] {
        &self.console_output
    }

    /// Clear console output
    pub fn clear_console(&mut self) {
        self.console_output.clear();
    }

    /// Get all request data
    pub fn request_data(&self) -> &HashMap<String, String> {
        &self.request_data
    }

    /// Get all response data
    pub fn response_data(&self) -> &HashMap<String, String> {
        &self.response_data
    }
}

impl Default for ScriptContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_script_variable_new() {
        let var = ScriptVariable::new("value".to_string());
        assert_eq!(var.value, "value");
        assert!(!var.is_secret);
    }

    #[test]
    fn test_script_variable_secret() {
        let var = ScriptVariable::secret("secret".to_string());
        assert_eq!(var.value, "secret");
        assert!(var.is_secret);
    }

    #[test]
    fn test_context_creation() {
        let context = ScriptContext::new();
        assert_eq!(context.variables.len(), 0);
        assert_eq!(context.console_output.len(), 0);
    }

    #[test]
    fn test_context_set_variable() {
        let mut context = ScriptContext::new();
        context.set_variable("key".to_string(), "value".to_string());

        assert_eq!(context.get_variable_value("key"), Some("value"));
    }

    #[test]
    fn test_context_set_secret_variable() {
        let mut context = ScriptContext::new();
        context.set_secret_variable("apiKey".to_string(), "secret123".to_string());

        let var = context.get_variable("apiKey").unwrap();
        assert_eq!(var.value, "secret123");
        assert!(var.is_secret);
    }

    #[test]
    fn test_context_remove_variable() {
        let mut context = ScriptContext::new();
        context.set_variable("test".to_string(), "value".to_string());

        assert!(context.get_variable("test").is_some());

        context.remove_variable("test");
        assert!(context.get_variable("test").is_none());
    }

    #[test]
    fn test_context_request_data() {
        let mut context = ScriptContext::new();
        context.set_request_data("method".to_string(), "GET".to_string());
        context.set_request_data("url".to_string(), "https://api.example.com".to_string());

        assert_eq!(context.get_request_data("method"), Some("GET"));
        assert_eq!(context.get_request_data("url"), Some("https://api.example.com"));
    }

    #[test]
    fn test_context_response_data() {
        let mut context = ScriptContext::new();
        context.set_response_data("status".to_string(), "200".to_string());
        context.set_response_data("body".to_string(), r#"{"result":"ok"}"#.to_string());

        assert_eq!(context.get_response_data("status"), Some("200"));
        assert_eq!(context.get_response_data("body"), Some(r#"{"result":"ok"}"#));
    }

    #[test]
    fn test_context_console_log() {
        let mut context = ScriptContext::new();
        context.console_log("Message 1".to_string());
        context.console_log("Message 2".to_string());

        assert_eq!(context.console_output().len(), 2);
        assert_eq!(context.console_output()[0], "Message 1");
        assert_eq!(context.console_output()[1], "Message 2");
    }

    #[test]
    fn test_context_clear_console() {
        let mut context = ScriptContext::new();
        context.console_log("Test".to_string());
        assert_eq!(context.console_output().len(), 1);

        context.clear_console();
        assert_eq!(context.console_output().len(), 0);
    }
}
