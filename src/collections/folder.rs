//! Folder organization for collections

use crate::collections::RequestItem;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A folder containing requests and sub-folders
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Folder {
    /// Unique identifier
    pub id: Uuid,

    /// Folder name
    pub name: String,

    /// Optional description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Requests in this folder
    #[serde(default)]
    pub requests: Vec<RequestItem>,

    /// Sub-folders
    #[serde(default)]
    pub folders: Vec<Folder>,

    /// Created timestamp
    pub created_at: DateTime<Utc>,

    /// Last modified timestamp
    pub updated_at: DateTime<Utc>,
}

impl Folder {
    /// Create a new folder
    pub fn new(name: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            description: None,
            requests: Vec::new(),
            folders: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Set description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Add a request to this folder
    pub fn add_request(&mut self, request: RequestItem) {
        self.requests.push(request);
        self.updated_at = Utc::now();
    }

    /// Add a sub-folder
    pub fn add_folder(&mut self, folder: Folder) {
        self.folders.push(folder);
        self.updated_at = Utc::now();
    }

    /// Get a request by ID
    pub fn get_request(&self, id: &Uuid) -> Option<&RequestItem> {
        // Check requests in this folder
        if let Some(req) = self.requests.iter().find(|r| r.id == *id) {
            return Some(req);
        }

        // Recursively check sub-folders
        for folder in &self.folders {
            if let Some(req) = folder.get_request(id) {
                return Some(req);
            }
        }

        None
    }

    /// Get a mutable reference to a request by ID
    pub fn get_request_mut(&mut self, id: &Uuid) -> Option<&mut RequestItem> {
        // Check requests in this folder
        if let Some(req) = self.requests.iter_mut().find(|r| r.id == *id) {
            return Some(req);
        }

        // Recursively check sub-folders
        for folder in &mut self.folders {
            if let Some(req) = folder.get_request_mut(id) {
                return Some(req);
            }
        }

        None
    }

    /// Remove a request by ID
    pub fn remove_request(&mut self, id: &Uuid) -> bool {
        // Try to remove from this folder
        if let Some(pos) = self.requests.iter().position(|r| r.id == *id) {
            self.requests.remove(pos);
            self.updated_at = Utc::now();
            return true;
        }

        // Recursively try sub-folders
        for folder in &mut self.folders {
            if folder.remove_request(id) {
                self.updated_at = Utc::now();
                return true;
            }
        }

        false
    }

    /// Get total count of requests (including in sub-folders)
    pub fn total_requests(&self) -> usize {
        let mut count = self.requests.len();
        for folder in &self.folders {
            count += folder.total_requests();
        }
        count
    }

    /// List all requests recursively
    pub fn list_all_requests(&self) -> Vec<&RequestItem> {
        let mut all_requests = Vec::new();

        // Add requests from this folder
        all_requests.extend(self.requests.iter());

        // Add requests from sub-folders
        for folder in &self.folders {
            all_requests.extend(folder.list_all_requests());
        }

        all_requests
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::HttpMethod;

    #[test]
    fn test_folder_creation() {
        let folder = Folder::new("Test Folder".to_string());
        assert_eq!(folder.name, "Test Folder");
        assert!(folder.requests.is_empty());
        assert!(folder.folders.is_empty());
    }

    #[test]
    fn test_add_request() {
        let mut folder = Folder::new("Test".to_string());
        let request = RequestItem::new(
            "Request 1".to_string(),
            HttpMethod::Get,
            "https://example.com".to_string(),
        );

        folder.add_request(request);
        assert_eq!(folder.requests.len(), 1);
    }

    #[test]
    fn test_add_subfolder() {
        let mut parent = Folder::new("Parent".to_string());
        let child = Folder::new("Child".to_string());

        parent.add_folder(child);
        assert_eq!(parent.folders.len(), 1);
        assert_eq!(parent.folders[0].name, "Child");
    }

    #[test]
    fn test_get_request() {
        let mut folder = Folder::new("Test".to_string());
        let request = RequestItem::new(
            "Request 1".to_string(),
            HttpMethod::Get,
            "https://example.com".to_string(),
        );
        let id = request.id;

        folder.add_request(request);

        let found = folder.get_request(&id);
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "Request 1");
    }

    #[test]
    fn test_remove_request() {
        let mut folder = Folder::new("Test".to_string());
        let request = RequestItem::new(
            "Request 1".to_string(),
            HttpMethod::Get,
            "https://example.com".to_string(),
        );
        let id = request.id;

        folder.add_request(request);
        assert_eq!(folder.requests.len(), 1);

        let removed = folder.remove_request(&id);
        assert!(removed);
        assert_eq!(folder.requests.len(), 0);
    }

    #[test]
    fn test_total_requests() {
        let mut parent = Folder::new("Parent".to_string());
        let mut child = Folder::new("Child".to_string());

        parent.add_request(RequestItem::new(
            "Request 1".to_string(),
            HttpMethod::Get,
            "https://example.com".to_string(),
        ));

        child.add_request(RequestItem::new(
            "Request 2".to_string(),
            HttpMethod::Post,
            "https://example.com".to_string(),
        ));

        parent.add_folder(child);

        assert_eq!(parent.total_requests(), 2);
    }
}
