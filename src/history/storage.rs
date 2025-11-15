//! History storage and persistence

use crate::history::HistoryEntry;
use chrono::{DateTime, Utc};
use std::path::{Path, PathBuf};
use uuid::Uuid;

/// Storage for history entries
pub struct HistoryStorage {
    base_path: PathBuf,
}

impl HistoryStorage {
    /// Create a new history storage
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

        let path = dirs.data_dir().join("history");
        Ok(path)
    }

    /// Save a single entry
    pub fn save_entry(&self, entry: &HistoryEntry) -> crate::Result<()> {
        let filename = format!("{}.json", entry.id);
        let path = self.base_path.join(filename);
        let json = serde_json::to_string_pretty(entry)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Save multiple entries
    pub fn save_entries(&self, entries: &[HistoryEntry]) -> crate::Result<()> {
        for entry in entries {
            self.save_entry(entry)?;
        }
        Ok(())
    }

    /// Load an entry by ID
    pub fn load_entry(&self, id: &Uuid) -> crate::Result<HistoryEntry> {
        let filename = format!("{}.json", id);
        let path = self.base_path.join(filename);
        let content = std::fs::read_to_string(path)?;
        let entry = serde_json::from_str(&content)?;
        Ok(entry)
    }

    /// Load all entries
    pub fn load_all(&self) -> crate::Result<Vec<HistoryEntry>> {
        let mut entries = Vec::new();

        for entry in std::fs::read_dir(&self.base_path)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Ok(hist_entry) = self.load_entry_from_path(&path) {
                    entries.push(hist_entry);
                }
            }
        }

        // Sort by timestamp (newest first)
        entries.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

        Ok(entries)
    }

    /// Load entry from specific path
    fn load_entry_from_path(&self, path: &Path) -> crate::Result<HistoryEntry> {
        let content = std::fs::read_to_string(path)?;
        let entry = serde_json::from_str(&content)?;
        Ok(entry)
    }

    /// Delete an entry
    pub fn delete_entry(&self, id: &Uuid) -> crate::Result<()> {
        let filename = format!("{}.json", id);
        let path = self.base_path.join(filename);
        std::fs::remove_file(path)?;
        Ok(())
    }

    /// Delete entries older than a certain date
    pub fn delete_older_than(&self, date: DateTime<Utc>) -> crate::Result<usize> {
        let entries = self.load_all()?;
        let mut deleted = 0;

        for entry in entries {
            if entry.timestamp < date {
                self.delete_entry(&entry.id)?;
                deleted += 1;
            }
        }

        Ok(deleted)
    }

    /// Clear all history
    pub fn clear_all(&self) -> crate::Result<usize> {
        let mut deleted = 0;

        for entry in std::fs::read_dir(&self.base_path)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                std::fs::remove_file(path)?;
                deleted += 1;
            }
        }

        Ok(deleted)
    }

    /// Export history to a single file
    pub fn export_to_file(&self, path: &Path) -> crate::Result<()> {
        let entries = self.load_all()?;
        let json = serde_json::to_string_pretty(&entries)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Import history from a single file
    pub fn import_from_file(&self, path: &Path) -> crate::Result<usize> {
        let content = std::fs::read_to_string(path)?;
        let entries: Vec<HistoryEntry> = serde_json::from_str(&content)?;
        let count = entries.len();

        self.save_entries(&entries)?;
        Ok(count)
    }

    /// Get count of stored entries
    pub fn count(&self) -> crate::Result<usize> {
        let mut count = 0;

        for entry in std::fs::read_dir(&self.base_path)? {
            let entry = entry?;
            if entry.path().extension().and_then(|s| s.to_str()) == Some("json") {
                count += 1;
            }
        }

        Ok(count)
    }

    /// Get storage size in bytes
    pub fn storage_size(&self) -> crate::Result<u64> {
        let mut total_size = 0;

        for entry in std::fs::read_dir(&self.base_path)? {
            let entry = entry?;
            if let Ok(metadata) = entry.metadata() {
                total_size += metadata.len();
            }
        }

        Ok(total_size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::history::RequestLog;
    use tempfile::TempDir;

    #[test]
    fn test_storage_creation() {
        let temp_dir = TempDir::new().unwrap();
        let storage = HistoryStorage::new(temp_dir.path().to_path_buf());
        assert!(storage.is_ok());
    }

    #[test]
    fn test_save_and_load() {
        let temp_dir = TempDir::new().unwrap();
        let storage = HistoryStorage::new(temp_dir.path().to_path_buf()).unwrap();

        let request = RequestLog::new("GET".to_string(), "https://example.com".to_string());
        let entry = HistoryEntry::new(request);
        let id = entry.id;

        storage.save_entry(&entry).unwrap();

        let loaded = storage.load_entry(&id).unwrap();
        assert_eq!(loaded.id, id);
        assert_eq!(loaded.request.method, "GET");
    }

    #[test]
    fn test_load_all() {
        let temp_dir = TempDir::new().unwrap();
        let storage = HistoryStorage::new(temp_dir.path().to_path_buf()).unwrap();

        let entry1 = HistoryEntry::new(RequestLog::new("GET".to_string(), "https://example.com/1".to_string()));
        let entry2 = HistoryEntry::new(RequestLog::new("POST".to_string(), "https://example.com/2".to_string()));

        storage.save_entry(&entry1).unwrap();
        storage.save_entry(&entry2).unwrap();

        let all = storage.load_all().unwrap();
        assert_eq!(all.len(), 2);
    }

    #[test]
    fn test_delete_entry() {
        let temp_dir = TempDir::new().unwrap();
        let storage = HistoryStorage::new(temp_dir.path().to_path_buf()).unwrap();

        let entry = HistoryEntry::new(RequestLog::new("GET".to_string(), "https://example.com".to_string()));
        let id = entry.id;

        storage.save_entry(&entry).unwrap();
        assert!(storage.load_entry(&id).is_ok());

        storage.delete_entry(&id).unwrap();
        assert!(storage.load_entry(&id).is_err());
    }

    #[test]
    fn test_clear_all() {
        let temp_dir = TempDir::new().unwrap();
        let storage = HistoryStorage::new(temp_dir.path().to_path_buf()).unwrap();

        storage.save_entry(&HistoryEntry::new(RequestLog::new("GET".to_string(), "https://example.com/1".to_string()))).unwrap();
        storage.save_entry(&HistoryEntry::new(RequestLog::new("GET".to_string(), "https://example.com/2".to_string()))).unwrap();

        let deleted = storage.clear_all().unwrap();
        assert_eq!(deleted, 2);
        assert_eq!(storage.count().unwrap(), 0);
    }

    #[test]
    fn test_export_import() {
        let temp_dir = TempDir::new().unwrap();
        let storage = HistoryStorage::new(temp_dir.path().to_path_buf()).unwrap();

        storage.save_entry(&HistoryEntry::new(RequestLog::new("GET".to_string(), "https://example.com/1".to_string()))).unwrap();
        storage.save_entry(&HistoryEntry::new(RequestLog::new("POST".to_string(), "https://example.com/2".to_string()))).unwrap();

        // Export to a file
        let export_path = temp_dir.path().join("export.json");
        storage.export_to_file(&export_path).unwrap();

        // Verify export file exists and has content
        assert!(export_path.exists());

        // Create a new storage directory for import
        let import_dir = temp_dir.path().join("import_storage");
        let import_storage = HistoryStorage::new(import_dir).unwrap();

        // Import into new storage
        let imported = import_storage.import_from_file(&export_path).unwrap();
        assert_eq!(imported, 2);
        assert_eq!(import_storage.count().unwrap(), 2);
    }

    #[test]
    fn test_storage_size() {
        let temp_dir = TempDir::new().unwrap();
        let storage = HistoryStorage::new(temp_dir.path().to_path_buf()).unwrap();

        storage.save_entry(&HistoryEntry::new(RequestLog::new("GET".to_string(), "https://example.com".to_string()))).unwrap();

        let size = storage.storage_size().unwrap();
        assert!(size > 0);
    }
}
