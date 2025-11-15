//! Script types and definitions

use serde::{Deserialize, Serialize};

/// Type of script
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ScriptType {
    /// Pre-request script (runs before sending request)
    PreRequest,

    /// Post-response script (runs after receiving response)
    PostResponse,
}

/// A script that can be executed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Script {
    /// Script type
    pub script_type: ScriptType,

    /// Script code
    pub code: String,

    /// Script name/description (optional)
    pub name: Option<String>,

    /// Whether script is enabled
    pub enabled: bool,
}

impl Script {
    /// Create a new script
    pub fn new(script_type: ScriptType, code: String) -> Self {
        Self {
            script_type,
            code,
            name: None,
            enabled: true,
        }
    }

    /// Create a pre-request script
    pub fn pre_request(code: String) -> Self {
        Self::new(ScriptType::PreRequest, code)
    }

    /// Create a post-response script
    pub fn post_response(code: String) -> Self {
        Self::new(ScriptType::PostResponse, code)
    }

    /// Set name
    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// Set enabled status
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Check if script should execute
    pub fn should_execute(&self) -> bool {
        self.enabled && !self.code.trim().is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_script_new() {
        let script = Script::new(ScriptType::PreRequest, "let x = 1;".to_string());
        assert_eq!(script.script_type, ScriptType::PreRequest);
        assert_eq!(script.code, "let x = 1;");
        assert!(script.enabled);
    }

    #[test]
    fn test_script_pre_request() {
        let script = Script::pre_request("console.log('test');".to_string());
        assert_eq!(script.script_type, ScriptType::PreRequest);
    }

    #[test]
    fn test_script_post_response() {
        let script = Script::post_response("console.log('done');".to_string());
        assert_eq!(script.script_type, ScriptType::PostResponse);
    }

    #[test]
    fn test_script_with_name() {
        let script = Script::pre_request("test".to_string())
            .with_name("My Script".to_string());
        assert_eq!(script.name, Some("My Script".to_string()));
    }

    #[test]
    fn test_script_with_enabled() {
        let script = Script::pre_request("test".to_string())
            .with_enabled(false);
        assert!(!script.enabled);
    }

    #[test]
    fn test_script_should_execute() {
        let enabled = Script::pre_request("test".to_string());
        assert!(enabled.should_execute());

        let disabled = Script::pre_request("test".to_string())
            .with_enabled(false);
        assert!(!disabled.should_execute());

        let empty = Script::pre_request("  ".to_string());
        assert!(!empty.should_execute());
    }

    #[test]
    fn test_script_serialization() {
        let script = Script::pre_request("let x = 1;".to_string())
            .with_name("Test".to_string());

        let json = serde_json::to_string(&script).unwrap();
        let deserialized: Script = serde_json::from_str(&json).unwrap();

        assert_eq!(script.code, deserialized.code);
        assert_eq!(script.name, deserialized.name);
    }
}
