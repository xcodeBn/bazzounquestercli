//! Collection data structure

use crate::collections::{Folder, RequestItem};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::Path;
use uuid::Uuid;

/// Collection information/metadata
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CollectionInfo {
    /// Collection name
    pub name: String,

    /// Collection ID
    pub id: Uuid,

    /// Version
    pub version: String,

    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Schema version
    pub schema: String,
}

/// A collection of HTTP requests
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Collection {
    /// Collection metadata
    pub info: CollectionInfo,

    /// Root-level requests
    #[serde(default)]
    pub requests: Vec<RequestItem>,

    /// Folders
    #[serde(default)]
    pub folders: Vec<Folder>,

    /// Created timestamp
    pub created_at: DateTime<Utc>,

    /// Last modified timestamp
    pub updated_at: DateTime<Utc>,
}

impl Collection {
    /// Create a new collection
    pub fn new(name: String) -> Self {
        let now = Utc::now();
        Self {
            info: CollectionInfo {
                name: name.clone(),
                id: Uuid::new_v4(),
                version: "1.0.0".to_string(),
                description: None,
                schema: "bazzounquester-1.0".to_string(),
            },
            requests: Vec::new(),
            folders: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Set description
    pub fn with_description(mut self, description: String) -> Self {
        self.info.description = Some(description);
        self
    }

    /// Add a request at the root level
    pub fn add_request(&mut self, request: RequestItem) {
        self.requests.push(request);
        self.updated_at = Utc::now();
    }

    /// Add a folder
    pub fn add_folder(&mut self, folder: Folder) {
        self.folders.push(folder);
        self.updated_at = Utc::now();
    }

    /// Get a request by ID (searches all folders)
    pub fn get_request(&self, id: &Uuid) -> Option<&RequestItem> {
        // Check root-level requests
        if let Some(req) = self.requests.iter().find(|r| r.id == *id) {
            return Some(req);
        }

        // Check folders
        for folder in &self.folders {
            if let Some(req) = folder.get_request(id) {
                return Some(req);
            }
        }

        None
    }

    /// Get a mutable request by ID
    pub fn get_request_mut(&mut self, id: &Uuid) -> Option<&mut RequestItem> {
        // Check root-level requests
        if let Some(req) = self.requests.iter_mut().find(|r| r.id == *id) {
            return Some(req);
        }

        // Check folders
        for folder in &mut self.folders {
            if let Some(req) = folder.get_request_mut(id) {
                return Some(req);
            }
        }

        None
    }

    /// Remove a request by ID
    pub fn remove_request(&mut self, id: &Uuid) -> bool {
        // Try root-level
        if let Some(pos) = self.requests.iter().position(|r| r.id == *id) {
            self.requests.remove(pos);
            self.updated_at = Utc::now();
            return true;
        }

        // Try folders
        for folder in &mut self.folders {
            if folder.remove_request(id) {
                self.updated_at = Utc::now();
                return true;
            }
        }

        false
    }

    /// Get total count of requests
    pub fn total_requests(&self) -> usize {
        let mut count = self.requests.len();
        for folder in &self.folders {
            count += folder.total_requests();
        }
        count
    }

    /// List all requests in the collection
    pub fn list_all_requests(&self) -> Vec<&RequestItem> {
        let mut all_requests = Vec::new();

        // Add root requests
        all_requests.extend(self.requests.iter());

        // Add folder requests
        for folder in &self.folders {
            all_requests.extend(folder.list_all_requests());
        }

        all_requests
    }

    /// Save collection to file
    pub fn save_to_file(&self, path: &Path) -> crate::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Load collection from file
    pub fn load_from_file(path: &Path) -> crate::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let collection = serde_json::from_str(&content)?;
        Ok(collection)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::HttpMethod;

    #[test]
    fn test_collection_creation() {
        let collection = Collection::new("My API".to_string());
        assert_eq!(collection.info.name, "My API");
        assert_eq!(collection.info.version, "1.0.0");
    }

    #[test]
    fn test_add_request() {
        let mut collection = Collection::new("Test".to_string());
        let request = RequestItem::new(
            "Get Users".to_string(),
            HttpMethod::Get,
            "https://api.example.com/users".to_string(),
        );

        collection.add_request(request);
        assert_eq!(collection.total_requests(), 1);
    }

    #[test]
    fn test_add_folder() {
        let mut collection = Collection::new("Test".to_string());
        let folder = Folder::new("Authentication".to_string());

        collection.add_folder(folder);
        assert_eq!(collection.folders.len(), 1);
    }

    #[test]
    fn test_total_requests() {
        let mut collection = Collection::new("Test".to_string());
        let mut folder = Folder::new("Folder 1".to_string());

        collection.add_request(RequestItem::new(
            "Request 1".to_string(),
            HttpMethod::Get,
            "https://example.com/1".to_string(),
        ));

        folder.add_request(RequestItem::new(
            "Request 2".to_string(),
            HttpMethod::Post,
            "https://example.com/2".to_string(),
        ));

        collection.add_folder(folder);

        assert_eq!(collection.total_requests(), 2);
    }

    #[test]
    fn test_serialization() {
        let collection = Collection::new("Test API".to_string());
        let json = serde_json::to_string(&collection).unwrap();
        let deserialized: Collection = serde_json::from_str(&json).unwrap();

        assert_eq!(collection.info.name, deserialized.info.name);
        assert_eq!(collection.info.id, deserialized.info.id);
    }
}
