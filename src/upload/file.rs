//! File upload handling

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// Represents a file to be uploaded
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileUpload {
    /// Path to the file
    pub path: PathBuf,

    /// Field name for the upload
    pub field_name: String,

    /// Custom filename (if different from actual filename)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_filename: Option<String>,

    /// MIME type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,

    /// File size in bytes
    #[serde(skip)]
    pub size: Option<u64>,
}

impl FileUpload {
    /// Create a new file upload
    pub fn new<P: AsRef<Path>>(path: P, field_name: String) -> crate::Result<Self> {
        let path_buf = path.as_ref().to_path_buf();

        // Check if file exists
        if !path_buf.exists() {
            return Err(crate::Error::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("File not found: {:?}", path_buf),
            )));
        }

        // Check if it's a file (not a directory)
        if !path_buf.is_file() {
            return Err(crate::Error::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Not a file: {:?}", path_buf),
            )));
        }

        // Get file size
        let metadata = fs::metadata(&path_buf)?;
        let size = Some(metadata.len());

        // Detect MIME type
        let mime_type = mime_guess::from_path(&path_buf)
            .first()
            .map(|m| m.to_string());

        Ok(Self {
            path: path_buf,
            field_name,
            custom_filename: None,
            mime_type,
            size,
        })
    }

    /// Set custom filename
    pub fn with_filename(mut self, filename: String) -> Self {
        self.custom_filename = Some(filename);
        self
    }

    /// Set MIME type
    pub fn with_mime_type(mut self, mime_type: String) -> Self {
        self.mime_type = Some(mime_type);
        self
    }

    /// Get the filename to use for upload
    pub fn filename(&self) -> String {
        if let Some(ref custom) = self.custom_filename {
            custom.clone()
        } else {
            self.path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("file")
                .to_string()
        }
    }

    /// Read file contents
    pub fn read_contents(&self) -> crate::Result<Vec<u8>> {
        let contents = fs::read(&self.path)?;
        Ok(contents)
    }

    /// Get MIME type (with fallback to application/octet-stream)
    pub fn mime(&self) -> String {
        self.mime_type
            .clone()
            .unwrap_or_else(|| "application/octet-stream".to_string())
    }

    /// Validate file can be read
    pub fn validate(&self) -> crate::Result<()> {
        if !self.path.exists() {
            return Err(crate::Error::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("File not found: {:?}", self.path),
            )));
        }

        // Try to read metadata to ensure we have permissions
        fs::metadata(&self.path)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_file_upload_creation() {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"test content").unwrap();

        let upload = FileUpload::new(temp_file.path(), "file".to_string());
        assert!(upload.is_ok());

        let upload = upload.unwrap();
        assert_eq!(upload.field_name, "file");
        assert!(upload.size.is_some());
    }

    #[test]
    fn test_file_upload_nonexistent() {
        let result = FileUpload::new("/nonexistent/file.txt", "file".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_custom_filename() {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"test").unwrap();

        let upload = FileUpload::new(temp_file.path(), "file".to_string())
            .unwrap()
            .with_filename("custom.txt".to_string());

        assert_eq!(upload.filename(), "custom.txt");
    }

    #[test]
    fn test_mime_type_detection() {
        let temp_file = NamedTempFile::with_suffix(".txt").unwrap();

        let upload = FileUpload::new(temp_file.path(), "file".to_string()).unwrap();

        // Should detect text/plain for .txt files
        assert!(upload.mime().contains("text"));
    }

    #[test]
    fn test_read_contents() {
        let mut temp_file = NamedTempFile::new().unwrap();
        let test_data = b"Hello, World!";
        temp_file.write_all(test_data).unwrap();

        let upload = FileUpload::new(temp_file.path(), "file".to_string()).unwrap();
        let contents = upload.read_contents().unwrap();

        assert_eq!(contents, test_data);
    }

    #[test]
    fn test_validate() {
        let temp_file = NamedTempFile::new().unwrap();
        let upload = FileUpload::new(temp_file.path(), "file".to_string()).unwrap();

        assert!(upload.validate().is_ok());
    }
}
