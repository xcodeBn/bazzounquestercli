//! Form data handling

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents form data (application/x-www-form-urlencoded or multipart/form-data)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FormData {
    /// Form fields
    fields: HashMap<String, FormField>,
}

/// A single form field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FormField {
    /// Text field
    Text(String),

    /// File field (path to file)
    File(String),
}

impl FormData {
    /// Create a new form data
    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
        }
    }

    /// Add a text field
    pub fn add_text(&mut self, name: String, value: String) {
        self.fields.insert(name, FormField::Text(value));
    }

    /// Add a file field
    pub fn add_file(&mut self, name: String, path: String) {
        self.fields.insert(name, FormField::File(path));
    }

    /// Get a field
    pub fn get(&self, name: &str) -> Option<&FormField> {
        self.fields.get(name)
    }

    /// Remove a field
    pub fn remove(&mut self, name: &str) -> Option<FormField> {
        self.fields.remove(name)
    }

    /// Get all fields
    pub fn fields(&self) -> &HashMap<String, FormField> {
        &self.fields
    }

    /// Count fields
    pub fn count(&self) -> usize {
        self.fields.len()
    }

    /// Convert to URL-encoded string
    pub fn to_urlencoded(&self) -> String {
        let parts: Vec<String> = self
            .fields
            .iter()
            .filter_map(|(name, field)| {
                if let FormField::Text(value) = field {
                    Some(format!(
                        "{}={}",
                        urlencoding::encode(name),
                        urlencoding::encode(value)
                    ))
                } else {
                    None
                }
            })
            .collect();

        parts.join("&")
    }

    /// Check if form contains files
    pub fn has_files(&self) -> bool {
        self.fields
            .values()
            .any(|f| matches!(f, FormField::File(_)))
    }

    /// Get text fields only
    pub fn text_fields(&self) -> HashMap<&str, &str> {
        self.fields
            .iter()
            .filter_map(|(name, field)| {
                if let FormField::Text(value) = field {
                    Some((name.as_str(), value.as_str()))
                } else {
                    None
                }
            })
            .collect()
    }

    /// Get file fields only
    pub fn file_fields(&self) -> HashMap<&str, &str> {
        self.fields
            .iter()
            .filter_map(|(name, field)| {
                if let FormField::File(path) = field {
                    Some((name.as_str(), path.as_str()))
                } else {
                    None
                }
            })
            .collect()
    }

    /// Builder pattern - add text
    pub fn with_text(mut self, name: String, value: String) -> Self {
        self.add_text(name, value);
        self
    }

    /// Builder pattern - add file
    pub fn with_file(mut self, name: String, path: String) -> Self {
        self.add_file(name, path);
        self
    }
}

// Need urlencoding crate for proper URL encoding
mod urlencoding {
    pub fn encode(s: &str) -> String {
        s.chars()
            .map(|c| match c {
                'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => c.to_string(),
                ' ' => "+".to_string(),
                _ => format!("%{:02X}", c as u8),
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_form_data_creation() {
        let form = FormData::new();
        assert_eq!(form.count(), 0);
    }

    #[test]
    fn test_add_text_field() {
        let mut form = FormData::new();
        form.add_text("username".to_string(), "john".to_string());
        form.add_text("password".to_string(), "secret".to_string());

        assert_eq!(form.count(), 2);
        assert!(matches!(form.get("username"), Some(FormField::Text(_))));
    }

    #[test]
    fn test_add_file_field() {
        let mut form = FormData::new();
        form.add_file("avatar".to_string(), "/path/to/image.jpg".to_string());

        assert_eq!(form.count(), 1);
        assert!(matches!(form.get("avatar"), Some(FormField::File(_))));
    }

    #[test]
    fn test_has_files() {
        let mut form = FormData::new();
        form.add_text("name".to_string(), "value".to_string());
        assert!(!form.has_files());

        form.add_file("file".to_string(), "/path/to/file".to_string());
        assert!(form.has_files());
    }

    #[test]
    fn test_to_urlencoded() {
        let mut form = FormData::new();
        form.add_text("name".to_string(), "John Doe".to_string());
        form.add_text("email".to_string(), "john@example.com".to_string());

        let encoded = form.to_urlencoded();
        assert!(encoded.contains("name=John+Doe"));
        assert!(encoded.contains("email=john"));
    }

    #[test]
    fn test_text_fields() {
        let mut form = FormData::new();
        form.add_text("field1".to_string(), "value1".to_string());
        form.add_file("file1".to_string(), "/path".to_string());

        let text_fields = form.text_fields();
        assert_eq!(text_fields.len(), 1);
        assert_eq!(text_fields.get("field1"), Some(&"value1"));
    }

    #[test]
    fn test_file_fields() {
        let mut form = FormData::new();
        form.add_text("field1".to_string(), "value1".to_string());
        form.add_file("file1".to_string(), "/path".to_string());

        let file_fields = form.file_fields();
        assert_eq!(file_fields.len(), 1);
        assert_eq!(file_fields.get("file1"), Some(&"/path"));
    }

    #[test]
    fn test_remove_field() {
        let mut form = FormData::new();
        form.add_text("test".to_string(), "value".to_string());

        assert_eq!(form.count(), 1);
        form.remove("test");
        assert_eq!(form.count(), 0);
    }

    #[test]
    fn test_builder_pattern() {
        let form = FormData::new()
            .with_text("name".to_string(), "value".to_string())
            .with_file("file".to_string(), "/path".to_string());

        assert_eq!(form.count(), 2);
    }
}
