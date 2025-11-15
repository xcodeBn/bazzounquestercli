//! Multipart form data builder

use crate::upload::{FileUpload, FormData, FormField};
use std::collections::HashMap;

/// Builder for multipart/form-data requests
pub struct MultipartBuilder {
    /// Boundary string for multipart
    boundary: String,

    /// Text fields
    text_fields: HashMap<String, String>,

    /// File uploads
    file_uploads: Vec<FileUpload>,
}

impl MultipartBuilder {
    /// Create a new multipart builder
    pub fn new() -> Self {
        Self {
            boundary: Self::generate_boundary(),
            text_fields: HashMap::new(),
            file_uploads: Vec::new(),
        }
    }

    /// Generate a boundary string
    fn generate_boundary() -> String {
        use std::time::{SystemTime, UNIX_EPOCH};

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        format!("----BazzounquesterBoundary{}", timestamp)
    }

    /// Add a text field
    pub fn add_text(&mut self, name: String, value: String) {
        self.text_fields.insert(name, value);
    }

    /// Add a file upload
    pub fn add_file(&mut self, upload: FileUpload) {
        self.file_uploads.push(upload);
    }

    /// Get the boundary
    pub fn boundary(&self) -> &str {
        &self.boundary
    }

    /// Get Content-Type header value
    pub fn content_type(&self) -> String {
        format!("multipart/form-data; boundary={}", self.boundary)
    }

    /// Build the multipart body
    pub fn build(&self) -> crate::Result<Vec<u8>> {
        let mut body = Vec::new();
        let boundary = format!("--{}", self.boundary);
        let crlf = b"\r\n";

        // Add text fields
        for (name, value) in &self.text_fields {
            body.extend_from_slice(boundary.as_bytes());
            body.extend_from_slice(crlf);

            body.extend_from_slice(
                format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", name).as_bytes(),
            );

            body.extend_from_slice(value.as_bytes());
            body.extend_from_slice(crlf);
        }

        // Add file uploads
        for file in &self.file_uploads {
            file.validate()?;

            body.extend_from_slice(boundary.as_bytes());
            body.extend_from_slice(crlf);

            body.extend_from_slice(
                format!(
                    "Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\n",
                    file.field_name,
                    file.filename()
                )
                .as_bytes(),
            );

            body.extend_from_slice(format!("Content-Type: {}\r\n\r\n", file.mime()).as_bytes());

            let file_contents = file.read_contents()?;
            body.extend_from_slice(&file_contents);
            body.extend_from_slice(crlf);
        }

        // Final boundary
        body.extend_from_slice(format!("{}--\r\n", boundary).as_bytes());

        Ok(body)
    }

    /// Create from FormData
    pub fn from_form_data(form: &FormData) -> crate::Result<Self> {
        let mut builder = Self::new();

        for (name, field) in form.fields() {
            match field {
                FormField::Text(value) => {
                    builder.add_text(name.clone(), value.clone());
                }
                FormField::File(path) => {
                    let upload = FileUpload::new(path, name.clone())?;
                    builder.add_file(upload);
                }
            }
        }

        Ok(builder)
    }

    /// Builder pattern - add text
    pub fn with_text(mut self, name: String, value: String) -> Self {
        self.add_text(name, value);
        self
    }

    /// Builder pattern - add file
    pub fn with_file(mut self, upload: FileUpload) -> Self {
        self.add_file(upload);
        self
    }
}

impl Default for MultipartBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_multipart_builder_creation() {
        let builder = MultipartBuilder::new();
        assert!(builder.boundary().starts_with("----BazzounquesterBoundary"));
    }

    #[test]
    fn test_add_text_field() {
        let mut builder = MultipartBuilder::new();
        builder.add_text("name".to_string(), "value".to_string());

        assert_eq!(builder.text_fields.len(), 1);
    }

    #[test]
    fn test_content_type() {
        let builder = MultipartBuilder::new();
        let ct = builder.content_type();

        assert!(ct.starts_with("multipart/form-data; boundary="));
    }

    #[test]
    fn test_build_with_text() {
        let mut builder = MultipartBuilder::new();
        builder.add_text("field1".to_string(), "value1".to_string());

        let body = builder.build().unwrap();
        let body_str = String::from_utf8_lossy(&body);

        assert!(body_str.contains("Content-Disposition: form-data; name=\"field1\""));
        assert!(body_str.contains("value1"));
    }

    #[test]
    fn test_build_with_file() {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"test file content").unwrap();

        let upload = FileUpload::new(temp_file.path(), "file".to_string()).unwrap();

        let mut builder = MultipartBuilder::new();
        builder.add_file(upload);

        let body = builder.build().unwrap();
        let body_str = String::from_utf8_lossy(&body);

        assert!(body_str.contains("Content-Disposition: form-data; name=\"file\""));
        assert!(body_str.contains("test file content"));
    }

    #[test]
    fn test_from_form_data() {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"content").unwrap();

        let mut form = FormData::new();
        form.add_text("name".to_string(), "value".to_string());
        form.add_file("file".to_string(), temp_file.path().to_str().unwrap().to_string());

        let builder = MultipartBuilder::from_form_data(&form).unwrap();
        assert_eq!(builder.text_fields.len(), 1);
        assert_eq!(builder.file_uploads.len(), 1);
    }

    #[test]
    fn test_builder_pattern() {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"test").unwrap();

        let upload = FileUpload::new(temp_file.path(), "file".to_string()).unwrap();

        let builder = MultipartBuilder::new()
            .with_text("name".to_string(), "value".to_string())
            .with_file(upload);

        assert_eq!(builder.text_fields.len(), 1);
        assert_eq!(builder.file_uploads.len(), 1);
    }
}
