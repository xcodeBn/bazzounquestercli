//! Session management for maintaining state

use crate::session::CookieJar;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use uuid::Uuid;

/// A session containing cookies and state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    /// Session ID
    pub id: Uuid,

    /// Session name
    pub name: String,

    /// Cookie jar
    pub cookies: CookieJar,

    /// Custom session variables
    #[serde(default)]
    pub variables: HashMap<String, String>,

    /// Created timestamp
    pub created_at: DateTime<Utc>,

    /// Last used timestamp
    pub last_used: DateTime<Utc>,

    /// Is this session active?
    #[serde(default)]
    pub is_active: bool,
}

impl Session {
    /// Create a new session
    pub fn new(name: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            cookies: CookieJar::new(),
            variables: HashMap::new(),
            created_at: now,
            last_used: now,
            is_active: false,
        }
    }

    /// Update last used timestamp
    pub fn touch(&mut self) {
        self.last_used = Utc::now();
    }

    /// Set a session variable
    pub fn set_variable(&mut self, key: String, value: String) {
        self.variables.insert(key, value);
        self.touch();
    }

    /// Get a session variable
    pub fn get_variable(&self, key: &str) -> Option<&String> {
        self.variables.get(key)
    }

    /// Remove a session variable
    pub fn remove_variable(&mut self, key: &str) -> Option<String> {
        let result = self.variables.remove(key);
        self.touch();
        result
    }

    /// Clear all session variables
    pub fn clear_variables(&mut self) {
        self.variables.clear();
        self.touch();
    }

    /// Clear all cookies
    pub fn clear_cookies(&mut self) {
        self.cookies.clear();
        self.touch();
    }

    /// Clear everything
    pub fn clear_all(&mut self) {
        self.clear_variables();
        self.clear_cookies();
    }

    /// Activate this session
    pub fn activate(&mut self) {
        self.is_active = true;
        self.touch();
    }

    /// Deactivate this session
    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    /// Save session to file
    pub fn save_to_file(&self, path: &Path) -> crate::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Load session from file
    pub fn load_from_file(path: &Path) -> crate::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let session = serde_json::from_str(&content)?;
        Ok(session)
    }

    /// Get age of session
    pub fn age(&self) -> chrono::Duration {
        Utc::now() - self.created_at
    }

    /// Get time since last use
    pub fn idle_time(&self) -> chrono::Duration {
        Utc::now() - self.last_used
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_session_creation() {
        let session = Session::new("Test Session".to_string());
        assert_eq!(session.name, "Test Session");
        assert!(!session.is_active);
        assert_eq!(session.cookies.count(), 0);
        assert_eq!(session.variables.len(), 0);
    }

    #[test]
    fn test_session_variables() {
        let mut session = Session::new("Test".to_string());

        session.set_variable("key1".to_string(), "value1".to_string());
        session.set_variable("key2".to_string(), "value2".to_string());

        assert_eq!(session.get_variable("key1"), Some(&"value1".to_string()));
        assert_eq!(session.variables.len(), 2);

        session.remove_variable("key1");
        assert_eq!(session.variables.len(), 1);
    }

    #[test]
    fn test_session_activate() {
        let mut session = Session::new("Test".to_string());
        assert!(!session.is_active);

        session.activate();
        assert!(session.is_active);

        session.deactivate();
        assert!(!session.is_active);
    }

    #[test]
    fn test_session_touch() {
        let mut session = Session::new("Test".to_string());
        let first_used = session.last_used;

        std::thread::sleep(std::time::Duration::from_millis(10));
        session.touch();

        assert!(session.last_used > first_used);
    }

    #[test]
    fn test_save_and_load() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("session.json");

        let mut session = Session::new("Test".to_string());
        session.set_variable("key".to_string(), "value".to_string());

        session.save_to_file(&file_path).unwrap();

        let loaded = Session::load_from_file(&file_path).unwrap();
        assert_eq!(loaded.name, "Test");
        assert_eq!(loaded.get_variable("key"), Some(&"value".to_string()));
    }

    #[test]
    fn test_clear_all() {
        let mut session = Session::new("Test".to_string());
        session.set_variable("key".to_string(), "value".to_string());

        session.clear_all();
        assert_eq!(session.variables.len(), 0);
        assert_eq!(session.cookies.count(), 0);
    }
}
