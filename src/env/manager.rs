//! Environment manager for handling multiple environments

use crate::env::{Environment, VariableSubstitutor};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use uuid::Uuid;

/// Manager for multiple environments
pub struct EnvironmentManager {
    environments: HashMap<Uuid, Environment>,
    active_env_id: Option<Uuid>,
    substitution_engine: VariableSubstitutor,
    storage_path: PathBuf,
}

impl EnvironmentManager {
    /// Create a new environment manager
    pub fn new(storage_path: PathBuf) -> crate::Result<Self> {
        std::fs::create_dir_all(&storage_path)?;

        Ok(Self {
            environments: HashMap::new(),
            active_env_id: None,
            substitution_engine: VariableSubstitutor::new(),
            storage_path,
        })
    }

    /// Get default storage path
    pub fn default_path() -> crate::Result<PathBuf> {
        let dirs = directories::ProjectDirs::from("com", "bazzoun", "bazzounquester").ok_or_else(
            || {
                crate::Error::Io(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Could not determine data directory",
                ))
            },
        )?;

        let path = dirs.data_dir().join("environments");
        Ok(path)
    }

    /// Add an environment
    pub fn add_environment(&mut self, environment: Environment) {
        let id = environment.id;
        self.environments.insert(id, environment);
    }

    /// Remove an environment
    pub fn remove_environment(&mut self, id: &Uuid) -> bool {
        if self.active_env_id == Some(*id) {
            self.active_env_id = None;
        }
        self.environments.remove(id).is_some()
    }

    /// Get an environment by ID
    pub fn get_environment(&self, id: &Uuid) -> Option<&Environment> {
        self.environments.get(id)
    }

    /// Get mutable reference to an environment
    pub fn get_environment_mut(&mut self, id: &Uuid) -> Option<&mut Environment> {
        self.environments.get_mut(id)
    }

    /// Get environment by name
    pub fn get_environment_by_name(&self, name: &str) -> Option<&Environment> {
        self.environments.values().find(|env| env.name == name)
    }

    /// List all environments
    pub fn list_environments(&self) -> Vec<&Environment> {
        self.environments.values().collect()
    }

    /// Set active environment
    pub fn set_active(&mut self, id: &Uuid) -> bool {
        if self.environments.contains_key(id) {
            // Deactivate previous
            if let Some(prev_id) = self.active_env_id {
                if let Some(prev_env) = self.environments.get_mut(&prev_id) {
                    prev_env.deactivate();
                }
            }

            // Activate new
            if let Some(env) = self.environments.get_mut(id) {
                env.activate();
                self.active_env_id = Some(*id);
                return true;
            }
        }
        false
    }

    /// Get active environment
    pub fn get_active_environment(&self) -> Option<&Environment> {
        self.active_env_id.and_then(|id| self.environments.get(&id))
    }

    /// Get active environment ID
    pub fn get_active_id(&self) -> Option<Uuid> {
        self.active_env_id
    }

    /// Substitute variables in text using active environment
    pub fn substitute(&self, text: &str) -> String {
        if let Some(env) = self.get_active_environment() {
            let vars = env.enabled_variables();
            self.substitution_engine.substitute(text, &vars)
        } else {
            text.to_string()
        }
    }

    /// Substitute variables using a specific environment
    pub fn substitute_with_env(&self, text: &str, env_id: &Uuid) -> String {
        if let Some(env) = self.environments.get(env_id) {
            let vars = env.enabled_variables();
            self.substitution_engine.substitute(text, &vars)
        } else {
            text.to_string()
        }
    }

    /// Validate that all variables in text can be resolved
    pub fn validate(&self, text: &str) -> Result<(), Vec<String>> {
        if let Some(env) = self.get_active_environment() {
            let vars = env.enabled_variables();
            self.substitution_engine.validate(text, &vars)
        } else {
            // No active environment - find all variables
            let found_vars = self.substitution_engine.find_variables(text);
            if found_vars.is_empty() {
                Ok(())
            } else {
                Err(found_vars)
            }
        }
    }

    /// Save an environment to disk
    pub fn save_environment(&self, id: &Uuid) -> crate::Result<()> {
        if let Some(env) = self.environments.get(id) {
            let filename = format!("{}.json", id);
            let path = self.storage_path.join(filename);
            env.save_to_file(&path)
        } else {
            Err(crate::Error::InvalidCommand(format!(
                "Environment {} not found",
                id
            )))
        }
    }

    /// Save all environments to disk
    pub fn save_all(&self) -> crate::Result<()> {
        for id in self.environments.keys() {
            self.save_environment(id)?;
        }
        Ok(())
    }

    /// Load an environment from disk
    pub fn load_environment(&mut self, path: &Path) -> crate::Result<Uuid> {
        let env = Environment::load_from_file(path)?;
        let id = env.id;
        self.environments.insert(id, env);
        Ok(id)
    }

    /// Load all environments from storage directory
    pub fn load_all(&mut self) -> crate::Result<()> {
        for entry in std::fs::read_dir(&self.storage_path)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Ok(env) = Environment::load_from_file(&path) {
                    let id = env.id;
                    if env.is_active {
                        self.active_env_id = Some(id);
                    }
                    self.environments.insert(id, env);
                }
            }
        }
        Ok(())
    }

    /// Delete environment file from disk
    pub fn delete_environment_file(&self, id: &Uuid) -> crate::Result<()> {
        let filename = format!("{}.json", id);
        let path = self.storage_path.join(filename);
        std::fs::remove_file(path)?;
        Ok(())
    }

    /// Create a quick environment for common scenarios
    pub fn create_quick_env(name: &str, base_url: &str) -> Environment {
        let mut env = Environment::new(name.to_string());
        env.set_variable("BASE_URL".to_string(), base_url.to_string());
        env.set_variable("API_VERSION".to_string(), "v1".to_string());
        env
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_manager_creation() {
        let temp_dir = TempDir::new().unwrap();
        let manager = EnvironmentManager::new(temp_dir.path().to_path_buf());
        assert!(manager.is_ok());
    }

    #[test]
    fn test_add_environment() {
        let temp_dir = TempDir::new().unwrap();
        let mut manager = EnvironmentManager::new(temp_dir.path().to_path_buf()).unwrap();

        let env = Environment::new("Dev".to_string());
        let id = env.id;

        manager.add_environment(env);
        assert!(manager.get_environment(&id).is_some());
    }

    #[test]
    fn test_set_active() {
        let temp_dir = TempDir::new().unwrap();
        let mut manager = EnvironmentManager::new(temp_dir.path().to_path_buf()).unwrap();

        let env = Environment::new("Dev".to_string());
        let id = env.id;
        manager.add_environment(env);

        assert!(manager.set_active(&id));
        assert_eq!(manager.get_active_id(), Some(id));

        let active = manager.get_active_environment();
        assert!(active.is_some());
        assert!(active.unwrap().is_active);
    }

    #[test]
    fn test_substitute_with_active_env() {
        let temp_dir = TempDir::new().unwrap();
        let mut manager = EnvironmentManager::new(temp_dir.path().to_path_buf()).unwrap();

        let mut env = Environment::new("Dev".to_string());
        env.set_variable("API_URL".to_string(), "https://dev.api.com".to_string());
        let id = env.id;

        manager.add_environment(env);
        manager.set_active(&id);

        let result = manager.substitute("{{API_URL}}/users");
        assert_eq!(result, "https://dev.api.com/users");
    }

    #[test]
    fn test_substitute_without_active_env() {
        let temp_dir = TempDir::new().unwrap();
        let manager = EnvironmentManager::new(temp_dir.path().to_path_buf()).unwrap();

        let result = manager.substitute("{{API_URL}}/users");
        assert_eq!(result, "{{API_URL}}/users");
    }

    #[test]
    fn test_save_and_load() {
        let temp_dir = TempDir::new().unwrap();
        let mut manager = EnvironmentManager::new(temp_dir.path().to_path_buf()).unwrap();

        let mut env = Environment::new("Test".to_string());
        env.set_variable("KEY".to_string(), "value".to_string());
        let id = env.id;

        manager.add_environment(env);
        manager.save_environment(&id).unwrap();

        // Create new manager and load
        let mut manager2 = EnvironmentManager::new(temp_dir.path().to_path_buf()).unwrap();
        manager2.load_all().unwrap();

        let loaded = manager2.get_environment(&id);
        assert!(loaded.is_some());
        assert_eq!(loaded.unwrap().name, "Test");
    }

    #[test]
    fn test_get_by_name() {
        let temp_dir = TempDir::new().unwrap();
        let mut manager = EnvironmentManager::new(temp_dir.path().to_path_buf()).unwrap();

        let env = Environment::new("Production".to_string());
        manager.add_environment(env);

        let found = manager.get_environment_by_name("Production");
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "Production");
    }

    #[test]
    fn test_create_quick_env() {
        let env = EnvironmentManager::create_quick_env("Dev", "https://dev.api.com");

        assert_eq!(env.name, "Dev");
        assert_eq!(env.get_variable("BASE_URL"), Some("https://dev.api.com"));
        assert_eq!(env.get_variable("API_VERSION"), Some("v1"));
    }

    #[test]
    fn test_remove_environment() {
        let temp_dir = TempDir::new().unwrap();
        let mut manager = EnvironmentManager::new(temp_dir.path().to_path_buf()).unwrap();

        let env = Environment::new("Test".to_string());
        let id = env.id;

        manager.add_environment(env);
        assert!(manager.remove_environment(&id));
        assert!(manager.get_environment(&id).is_none());
    }

    #[test]
    fn test_validate() {
        let temp_dir = TempDir::new().unwrap();
        let mut manager = EnvironmentManager::new(temp_dir.path().to_path_buf()).unwrap();

        let mut env = Environment::new("Dev".to_string());
        env.set_variable("API_URL".to_string(), "https://api.com".to_string());
        let id = env.id;

        manager.add_environment(env);
        manager.set_active(&id);

        // Should pass - variable exists
        assert!(manager.validate("{{API_URL}}/users").is_ok());

        // Should fail - variable doesn't exist
        let result = manager.validate("{{MISSING}}/users");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), vec!["MISSING".to_string()]);
    }
}
