//! Workspace management for organizing collections

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use uuid::Uuid;

/// A workspace containing multiple collections
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Workspace {
    /// Workspace ID
    pub id: Uuid,

    /// Workspace name
    pub name: String,

    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Collection IDs in this workspace
    #[serde(default)]
    pub collection_ids: Vec<Uuid>,

    /// Workspace-level environment variables
    #[serde(default)]
    pub variables: HashMap<String, String>,

    /// Created timestamp
    pub created_at: DateTime<Utc>,

    /// Last modified timestamp
    pub updated_at: DateTime<Utc>,
}

impl Workspace {
    /// Create a new workspace
    pub fn new(name: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            description: None,
            collection_ids: Vec::new(),
            variables: HashMap::new(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Set description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Add a collection to this workspace
    pub fn add_collection(&mut self, collection_id: Uuid) {
        if !self.collection_ids.contains(&collection_id) {
            self.collection_ids.push(collection_id);
            self.updated_at = Utc::now();
        }
    }

    /// Remove a collection from this workspace
    pub fn remove_collection(&mut self, collection_id: &Uuid) -> bool {
        if let Some(pos) = self
            .collection_ids
            .iter()
            .position(|id| id == collection_id)
        {
            self.collection_ids.remove(pos);
            self.updated_at = Utc::now();
            true
        } else {
            false
        }
    }

    /// Set a workspace variable
    pub fn set_variable(&mut self, key: String, value: String) {
        self.variables.insert(key, value);
        self.updated_at = Utc::now();
    }

    /// Get a workspace variable
    pub fn get_variable(&self, key: &str) -> Option<&String> {
        self.variables.get(key)
    }
}

/// Storage for workspaces
pub struct WorkspaceStorage {
    base_path: PathBuf,
}

impl WorkspaceStorage {
    /// Create a new workspace storage
    pub fn new(base_path: PathBuf) -> crate::Result<Self> {
        std::fs::create_dir_all(&base_path)?;
        Ok(Self { base_path })
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

        let path = dirs.data_dir().join("workspaces");
        Ok(path)
    }

    /// Save a workspace
    pub fn save(&self, workspace: &Workspace) -> crate::Result<()> {
        let filename = format!("{}.json", workspace.id);
        let path = self.base_path.join(filename);
        let json = serde_json::to_string_pretty(workspace)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Load a workspace
    pub fn load(&self, id: &Uuid) -> crate::Result<Workspace> {
        let filename = format!("{}.json", id);
        let path = self.base_path.join(filename);
        let content = std::fs::read_to_string(path)?;
        let workspace = serde_json::from_str(&content)?;
        Ok(workspace)
    }

    /// List all workspaces
    pub fn list_all(&self) -> crate::Result<Vec<Workspace>> {
        let mut workspaces = Vec::new();

        for entry in std::fs::read_dir(&self.base_path)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let content = std::fs::read_to_string(&path)?;
                if let Ok(workspace) = serde_json::from_str::<Workspace>(&content) {
                    workspaces.push(workspace);
                }
            }
        }

        Ok(workspaces)
    }

    /// Delete a workspace
    pub fn delete(&self, id: &Uuid) -> crate::Result<()> {
        let filename = format!("{}.json", id);
        let path = self.base_path.join(filename);
        std::fs::remove_file(path)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workspace_creation() {
        let workspace = Workspace::new("My Workspace".to_string());
        assert_eq!(workspace.name, "My Workspace");
        assert!(workspace.collection_ids.is_empty());
    }

    #[test]
    fn test_add_collection() {
        let mut workspace = Workspace::new("Test".to_string());
        let collection_id = Uuid::new_v4();

        workspace.add_collection(collection_id);
        assert_eq!(workspace.collection_ids.len(), 1);
        assert!(workspace.collection_ids.contains(&collection_id));
    }

    #[test]
    fn test_remove_collection() {
        let mut workspace = Workspace::new("Test".to_string());
        let collection_id = Uuid::new_v4();

        workspace.add_collection(collection_id);
        assert_eq!(workspace.collection_ids.len(), 1);

        workspace.remove_collection(&collection_id);
        assert_eq!(workspace.collection_ids.len(), 0);
    }

    #[test]
    fn test_workspace_variables() {
        let mut workspace = Workspace::new("Test".to_string());

        workspace.set_variable("API_URL".to_string(), "https://api.example.com".to_string());
        workspace.set_variable("API_KEY".to_string(), "secret123".to_string());

        assert_eq!(
            workspace.get_variable("API_URL"),
            Some(&"https://api.example.com".to_string())
        );
        assert_eq!(
            workspace.get_variable("API_KEY"),
            Some(&"secret123".to_string())
        );
    }
}
