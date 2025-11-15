//! Environment data structure and management

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use uuid::Uuid;

/// An environment with variables
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Environment {
    /// Unique identifier
    pub id: Uuid,

    /// Environment name
    pub name: String,

    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Variables (key-value pairs)
    #[serde(default)]
    pub variables: HashMap<String, EnvironmentVariable>,

    /// Created timestamp
    pub created_at: DateTime<Utc>,

    /// Last modified timestamp
    pub updated_at: DateTime<Utc>,

    /// Is this environment active?
    #[serde(default)]
    pub is_active: bool,
}

/// An individual environment variable
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EnvironmentVariable {
    /// Variable value
    pub value: String,

    /// Is this a secret variable?
    #[serde(default)]
    pub is_secret: bool,

    /// Variable type (for future use: string, number, boolean, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub var_type: Option<String>,

    /// Description of this variable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Is this variable enabled?
    #[serde(default = "default_enabled")]
    pub enabled: bool,
}

fn default_enabled() -> bool {
    true
}

impl Environment {
    /// Create a new environment
    pub fn new(name: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            description: None,
            variables: HashMap::new(),
            created_at: now,
            updated_at: now,
            is_active: false,
        }
    }

    /// Set description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Set a variable
    pub fn set_variable(&mut self, key: String, value: String) {
        self.variables.insert(
            key,
            EnvironmentVariable {
                value,
                is_secret: false,
                var_type: None,
                description: None,
                enabled: true,
            },
        );
        self.updated_at = Utc::now();
    }

    /// Set a secret variable
    pub fn set_secret(&mut self, key: String, value: String) {
        self.variables.insert(
            key,
            EnvironmentVariable {
                value,
                is_secret: true,
                var_type: None,
                description: None,
                enabled: true,
            },
        );
        self.updated_at = Utc::now();
    }

    /// Set a variable with full configuration
    pub fn set_variable_full(
        &mut self,
        key: String,
        value: String,
        is_secret: bool,
        description: Option<String>,
    ) {
        self.variables.insert(
            key,
            EnvironmentVariable {
                value,
                is_secret,
                var_type: None,
                description,
                enabled: true,
            },
        );
        self.updated_at = Utc::now();
    }

    /// Get a variable value
    pub fn get_variable(&self, key: &str) -> Option<&str> {
        self.variables
            .get(key)
            .filter(|v| v.enabled)
            .map(|v| v.value.as_str())
    }

    /// Remove a variable
    pub fn remove_variable(&mut self, key: &str) -> bool {
        if self.variables.remove(key).is_some() {
            self.updated_at = Utc::now();
            true
        } else {
            false
        }
    }

    /// Enable/disable a variable
    pub fn set_variable_enabled(&mut self, key: &str, enabled: bool) -> bool {
        if let Some(var) = self.variables.get_mut(key) {
            var.enabled = enabled;
            self.updated_at = Utc::now();
            true
        } else {
            false
        }
    }

    /// Get all variable names
    pub fn variable_names(&self) -> Vec<&String> {
        self.variables.keys().collect()
    }

    /// Get all enabled variables
    pub fn enabled_variables(&self) -> HashMap<&str, &str> {
        self.variables
            .iter()
            .filter(|(_, v)| v.enabled)
            .map(|(k, v)| (k.as_str(), v.value.as_str()))
            .collect()
    }

    /// Activate this environment
    pub fn activate(&mut self) {
        self.is_active = true;
        self.updated_at = Utc::now();
    }

    /// Deactivate this environment
    pub fn deactivate(&mut self) {
        self.is_active = false;
        self.updated_at = Utc::now();
    }

    /// Save environment to file
    pub fn save_to_file(&self, path: &Path) -> crate::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Load environment from file
    pub fn load_from_file(path: &Path) -> crate::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let env = serde_json::from_str(&content)?;
        Ok(env)
    }

    /// Export to different formats
    pub fn export_yaml(&self, path: &Path) -> crate::Result<()> {
        let yaml = serde_yaml::to_string(self).map_err(|e| {
            crate::Error::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                e.to_string(),
            ))
        })?;
        std::fs::write(path, yaml)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_environment_creation() {
        let env = Environment::new("Development".to_string());
        assert_eq!(env.name, "Development");
        assert!(env.variables.is_empty());
        assert!(!env.is_active);
    }

    #[test]
    fn test_set_variable() {
        let mut env = Environment::new("Test".to_string());
        env.set_variable("API_URL".to_string(), "https://api.example.com".to_string());

        assert_eq!(env.get_variable("API_URL"), Some("https://api.example.com"));
    }

    #[test]
    fn test_set_secret() {
        let mut env = Environment::new("Test".to_string());
        env.set_secret("API_KEY".to_string(), "secret123".to_string());

        let var = env.variables.get("API_KEY").unwrap();
        assert!(var.is_secret);
        assert_eq!(var.value, "secret123");
    }

    #[test]
    fn test_remove_variable() {
        let mut env = Environment::new("Test".to_string());
        env.set_variable("TEST".to_string(), "value".to_string());

        assert!(env.remove_variable("TEST"));
        assert!(env.get_variable("TEST").is_none());
        assert!(!env.remove_variable("NONEXISTENT"));
    }

    #[test]
    fn test_enable_disable_variable() {
        let mut env = Environment::new("Test".to_string());
        env.set_variable("TEST".to_string(), "value".to_string());

        env.set_variable_enabled("TEST", false);
        assert!(env.get_variable("TEST").is_none()); // Disabled variables don't show

        env.set_variable_enabled("TEST", true);
        assert_eq!(env.get_variable("TEST"), Some("value"));
    }

    #[test]
    fn test_activate_deactivate() {
        let mut env = Environment::new("Test".to_string());
        assert!(!env.is_active);

        env.activate();
        assert!(env.is_active);

        env.deactivate();
        assert!(!env.is_active);
    }

    #[test]
    fn test_save_and_load() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test_env.json");

        let mut env = Environment::new("Test".to_string());
        env.set_variable("KEY1".to_string(), "value1".to_string());
        env.set_secret("SECRET".to_string(), "secret_value".to_string());

        env.save_to_file(&file_path).unwrap();

        let loaded = Environment::load_from_file(&file_path).unwrap();
        assert_eq!(loaded.name, "Test");
        assert_eq!(loaded.get_variable("KEY1"), Some("value1"));
        assert_eq!(loaded.variables.get("SECRET").unwrap().is_secret, true);
    }

    #[test]
    fn test_enabled_variables() {
        let mut env = Environment::new("Test".to_string());
        env.set_variable("VAR1".to_string(), "value1".to_string());
        env.set_variable("VAR2".to_string(), "value2".to_string());
        env.set_variable("VAR3".to_string(), "value3".to_string());

        env.set_variable_enabled("VAR2", false);

        let enabled = env.enabled_variables();
        assert_eq!(enabled.len(), 2);
        assert_eq!(enabled.get("VAR1"), Some(&"value1"));
        assert_eq!(enabled.get("VAR3"), Some(&"value3"));
        assert_eq!(enabled.get("VAR2"), None);
    }
}
