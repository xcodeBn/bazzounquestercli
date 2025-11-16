//! Session manager for handling multiple sessions

use crate::session::Session;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use uuid::Uuid;

/// Manager for multiple sessions
pub struct SessionManager {
    sessions: HashMap<Uuid, Session>,
    active_session_id: Option<Uuid>,
    storage_path: PathBuf,
}

impl SessionManager {
    /// Create a new session manager
    pub fn new(storage_path: PathBuf) -> crate::Result<Self> {
        std::fs::create_dir_all(&storage_path)?;

        Ok(Self {
            sessions: HashMap::new(),
            active_session_id: None,
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

        let path = dirs.data_dir().join("sessions");
        Ok(path)
    }

    /// Add a session
    pub fn add_session(&mut self, session: Session) -> Uuid {
        let id = session.id;
        self.sessions.insert(id, session);
        id
    }

    /// Remove a session
    pub fn remove_session(&mut self, id: &Uuid) -> bool {
        if self.active_session_id == Some(*id) {
            self.active_session_id = None;
        }
        self.sessions.remove(id).is_some()
    }

    /// Get a session by ID
    pub fn get_session(&self, id: &Uuid) -> Option<&Session> {
        self.sessions.get(id)
    }

    /// Get mutable reference to a session
    pub fn get_session_mut(&mut self, id: &Uuid) -> Option<&mut Session> {
        self.sessions.get_mut(id)
    }

    /// Get session by name
    pub fn get_session_by_name(&self, name: &str) -> Option<&Session> {
        self.sessions.values().find(|s| s.name == name)
    }

    /// List all sessions
    pub fn list_sessions(&self) -> Vec<&Session> {
        self.sessions.values().collect()
    }

    /// Set active session
    pub fn set_active(&mut self, id: &Uuid) -> bool {
        if self.sessions.contains_key(id) {
            // Deactivate previous
            if let Some(prev_id) = self.active_session_id {
                if let Some(prev) = self.sessions.get_mut(&prev_id) {
                    prev.deactivate();
                }
            }

            // Activate new
            if let Some(session) = self.sessions.get_mut(id) {
                session.activate();
                self.active_session_id = Some(*id);
                return true;
            }
        }
        false
    }

    /// Get active session
    pub fn get_active_session(&self) -> Option<&Session> {
        self.active_session_id.and_then(|id| self.sessions.get(&id))
    }

    /// Get mutable active session
    pub fn get_active_session_mut(&mut self) -> Option<&mut Session> {
        self.active_session_id
            .and_then(|id| self.sessions.get_mut(&id))
    }

    /// Create a new session and optionally activate it
    pub fn create_session(&mut self, name: String, activate: bool) -> Uuid {
        let session = Session::new(name);
        let id = session.id;

        self.sessions.insert(id, session);

        if activate {
            self.set_active(&id);
        }

        id
    }

    /// Save a session to disk
    pub fn save_session(&self, id: &Uuid) -> crate::Result<()> {
        if let Some(session) = self.sessions.get(id) {
            let filename = format!("{}.json", id);
            let path = self.storage_path.join(filename);
            session.save_to_file(&path)
        } else {
            Err(crate::Error::InvalidCommand(format!(
                "Session {} not found",
                id
            )))
        }
    }

    /// Save all sessions
    pub fn save_all(&self) -> crate::Result<()> {
        for id in self.sessions.keys() {
            self.save_session(id)?;
        }
        Ok(())
    }

    /// Load a session from disk
    pub fn load_session(&mut self, path: &Path) -> crate::Result<Uuid> {
        let session = Session::load_from_file(path)?;
        let id = session.id;

        if session.is_active {
            self.active_session_id = Some(id);
        }

        self.sessions.insert(id, session);
        Ok(id)
    }

    /// Load all sessions from storage
    pub fn load_all(&mut self) -> crate::Result<()> {
        for entry in std::fs::read_dir(&self.storage_path)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Ok(session) = Session::load_from_file(&path) {
                    let id = session.id;
                    if session.is_active {
                        self.active_session_id = Some(id);
                    }
                    self.sessions.insert(id, session);
                }
            }
        }
        Ok(())
    }

    /// Delete session file from disk
    pub fn delete_session_file(&self, id: &Uuid) -> crate::Result<()> {
        let filename = format!("{}.json", id);
        let path = self.storage_path.join(filename);
        std::fs::remove_file(path)?;
        Ok(())
    }

    /// Count sessions
    pub fn count(&self) -> usize {
        self.sessions.len()
    }

    /// Clean up idle sessions
    pub fn cleanup_idle(&mut self, max_idle_minutes: i64) {
        let now = chrono::Utc::now();
        self.sessions.retain(|_, session| {
            let idle_time = now - session.last_used;
            idle_time.num_minutes() < max_idle_minutes
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_manager_creation() {
        let temp_dir = TempDir::new().unwrap();
        let manager = SessionManager::new(temp_dir.path().to_path_buf());
        assert!(manager.is_ok());
    }

    #[test]
    fn test_create_session() {
        let temp_dir = TempDir::new().unwrap();
        let mut manager = SessionManager::new(temp_dir.path().to_path_buf()).unwrap();

        let id = manager.create_session("Test Session".to_string(), false);
        assert_eq!(manager.count(), 1);

        let session = manager.get_session(&id);
        assert!(session.is_some());
        assert_eq!(session.unwrap().name, "Test Session");
    }

    #[test]
    fn test_set_active() {
        let temp_dir = TempDir::new().unwrap();
        let mut manager = SessionManager::new(temp_dir.path().to_path_buf()).unwrap();

        let id = manager.create_session("Test".to_string(), false);
        assert!(manager.set_active(&id));

        let active = manager.get_active_session();
        assert!(active.is_some());
        assert_eq!(active.unwrap().id, id);
        assert!(active.unwrap().is_active);
    }

    #[test]
    fn test_save_and_load() {
        let temp_dir = TempDir::new().unwrap();
        let mut manager = SessionManager::new(temp_dir.path().to_path_buf()).unwrap();

        let id = manager.create_session("Test".to_string(), false);
        manager.save_session(&id).unwrap();

        // Create new manager and load
        let mut manager2 = SessionManager::new(temp_dir.path().to_path_buf()).unwrap();
        manager2.load_all().unwrap();

        assert_eq!(manager2.count(), 1);
        let session = manager2.get_session(&id);
        assert!(session.is_some());
        assert_eq!(session.unwrap().name, "Test");
    }

    #[test]
    fn test_remove_session() {
        let temp_dir = TempDir::new().unwrap();
        let mut manager = SessionManager::new(temp_dir.path().to_path_buf()).unwrap();

        let id = manager.create_session("Test".to_string(), false);
        assert_eq!(manager.count(), 1);

        assert!(manager.remove_session(&id));
        assert_eq!(manager.count(), 0);
    }

    #[test]
    fn test_get_by_name() {
        let temp_dir = TempDir::new().unwrap();
        let mut manager = SessionManager::new(temp_dir.path().to_path_buf()).unwrap();

        manager.create_session("Session 1".to_string(), false);
        manager.create_session("Session 2".to_string(), false);

        let found = manager.get_session_by_name("Session 1");
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "Session 1");
    }
}
