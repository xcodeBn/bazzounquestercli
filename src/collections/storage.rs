//! Collection storage and persistence

use crate::collections::Collection;
use std::path::{Path, PathBuf};
use uuid::Uuid;

/// Storage for collections
pub struct CollectionStorage {
    base_path: PathBuf,
}

impl CollectionStorage {
    /// Create a new collection storage
    pub fn new(base_path: PathBuf) -> crate::Result<Self> {
        std::fs::create_dir_all(&base_path)?;
        Ok(Self { base_path })
    }

    /// Get default storage path
    pub fn default_path() -> crate::Result<PathBuf> {
        let dirs = directories::ProjectDirs::from("com", "bazzoun", "bazzounquester")
            .ok_or_else(|| {
                crate::Error::Io(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Could not determine data directory",
                ))
            })?;

        let path = dirs.data_dir().join("collections");
        Ok(path)
    }

    /// Save a collection
    pub fn save(&self, collection: &Collection) -> crate::Result<()> {
        let filename = format!("{}.json", collection.info.id);
        let path = self.base_path.join(filename);
        collection.save_to_file(&path)
    }

    /// Load a collection by ID
    pub fn load(&self, id: &Uuid) -> crate::Result<Collection> {
        let filename = format!("{}.json", id);
        let path = self.base_path.join(filename);
        Collection::load_from_file(&path)
    }

    /// Load a collection from a specific path
    pub fn load_from_path(&self, path: &Path) -> crate::Result<Collection> {
        Collection::load_from_file(path)
    }

    /// List all collections
    pub fn list_all(&self) -> crate::Result<Vec<Collection>> {
        let mut collections = Vec::new();

        for entry in std::fs::read_dir(&self.base_path)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Ok(collection) = Collection::load_from_file(&path) {
                    collections.push(collection);
                }
            }
        }

        Ok(collections)
    }

    /// Delete a collection
    pub fn delete(&self, id: &Uuid) -> crate::Result<()> {
        let filename = format!("{}.json", id);
        let path = self.base_path.join(filename);
        std::fs::remove_file(path)?;
        Ok(())
    }

    /// Export collection to different formats
    pub fn export(&self, collection: &Collection, path: &Path, format: ExportFormat) -> crate::Result<()> {
        match format {
            ExportFormat::Json => {
                collection.save_to_file(path)
            }
            ExportFormat::Yaml => {
                let yaml = serde_yaml::to_string(collection)
                    .map_err(|e| crate::Error::Io(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        e.to_string(),
                    )))?;
                std::fs::write(path, yaml)?;
                Ok(())
            }
        }
    }

    /// Import collection from different formats
    pub fn import(&self, path: &Path, format: ImportFormat) -> crate::Result<Collection> {
        let content = std::fs::read_to_string(path)?;

        match format {
            ImportFormat::Json => {
                let collection = serde_json::from_str(&content)?;
                Ok(collection)
            }
            ImportFormat::Yaml => {
                let collection = serde_yaml::from_str(&content)
                    .map_err(|e| crate::Error::Io(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        e.to_string(),
                    )))?;
                Ok(collection)
            }
            ImportFormat::Postman => {
                // TODO: Implement Postman format conversion
                Err(crate::Error::InvalidCommand(
                    "Postman import not yet implemented".to_string(),
                ))
            }
        }
    }
}

/// Export formats for collections
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
    Json,
    Yaml,
}

/// Import formats for collections
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImportFormat {
    Json,
    Yaml,
    Postman,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_storage_creation() {
        let temp_dir = TempDir::new().unwrap();
        let storage = CollectionStorage::new(temp_dir.path().to_path_buf());
        assert!(storage.is_ok());
    }

    #[test]
    fn test_save_and_load() {
        let temp_dir = TempDir::new().unwrap();
        let storage = CollectionStorage::new(temp_dir.path().to_path_buf()).unwrap();
        let collection = Collection::new("Test Collection".to_string());
        let id = collection.info.id;

        // Save
        storage.save(&collection).unwrap();

        // Load
        let loaded = storage.load(&id).unwrap();
        assert_eq!(loaded.info.name, "Test Collection");
        assert_eq!(loaded.info.id, id);
    }

    #[test]
    fn test_list_all() {
        let temp_dir = TempDir::new().unwrap();
        let storage = CollectionStorage::new(temp_dir.path().to_path_buf()).unwrap();

        let collection1 = Collection::new("Collection 1".to_string());
        let collection2 = Collection::new("Collection 2".to_string());

        storage.save(&collection1).unwrap();
        storage.save(&collection2).unwrap();

        let all = storage.list_all().unwrap();
        assert_eq!(all.len(), 2);
    }

    #[test]
    fn test_delete() {
        let temp_dir = TempDir::new().unwrap();
        let storage = CollectionStorage::new(temp_dir.path().to_path_buf()).unwrap();
        let collection = Collection::new("Test".to_string());
        let id = collection.info.id;

        storage.save(&collection).unwrap();
        assert!(storage.load(&id).is_ok());

        storage.delete(&id).unwrap();
        assert!(storage.load(&id).is_err());
    }
}
