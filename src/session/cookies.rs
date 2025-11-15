//! Cookie handling and storage

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A simple cookie representation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Cookie {
    /// Cookie name
    pub name: String,

    /// Cookie value
    pub value: String,

    /// Domain
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    /// Path
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    /// Expiry time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<DateTime<Utc>>,

    /// Is HTTP only?
    #[serde(default)]
    pub http_only: bool,

    /// Is secure?
    #[serde(default)]
    pub secure: bool,

    /// Same site policy
    #[serde(skip_serializing_if = "Option::is_none")]
    pub same_site: Option<String>,
}

impl Cookie {
    /// Create a new cookie
    pub fn new(name: String, value: String) -> Self {
        Self {
            name,
            value,
            domain: None,
            path: None,
            expires: None,
            http_only: false,
            secure: false,
            same_site: None,
        }
    }

    /// Set domain
    pub fn with_domain(mut self, domain: String) -> Self {
        self.domain = Some(domain);
        self
    }

    /// Set path
    pub fn with_path(mut self, path: String) -> Self {
        self.path = Some(path);
        self
    }

    /// Set expiry
    pub fn with_expires(mut self, expires: DateTime<Utc>) -> Self {
        self.expires = Some(expires);
        self
    }

    /// Check if cookie is expired
    pub fn is_expired(&self) -> bool {
        if let Some(expires) = self.expires {
            expires < Utc::now()
        } else {
            false
        }
    }

    /// Parse from Set-Cookie header
    pub fn from_header(header: &str) -> Option<Self> {
        let parts: Vec<&str> = header.split(';').collect();
        if parts.is_empty() {
            return None;
        }

        // First part is name=value
        let name_value: Vec<&str> = parts[0].split('=').collect();
        if name_value.len() != 2 {
            return None;
        }

        let mut cookie = Cookie::new(
            name_value[0].trim().to_string(),
            name_value[1].trim().to_string(),
        );

        // Parse attributes
        for part in &parts[1..] {
            let attr: Vec<&str> = part.split('=').collect();
            let attr_name = attr[0].trim().to_lowercase();

            match attr_name.as_str() {
                "domain" if attr.len() > 1 => {
                    cookie.domain = Some(attr[1].trim().to_string());
                }
                "path" if attr.len() > 1 => {
                    cookie.path = Some(attr[1].trim().to_string());
                }
                "httponly" => {
                    cookie.http_only = true;
                }
                "secure" => {
                    cookie.secure = true;
                }
                "samesite" if attr.len() > 1 => {
                    cookie.same_site = Some(attr[1].trim().to_string());
                }
                _ => {}
            }
        }

        Some(cookie)
    }

    /// Convert to Cookie header format
    pub fn to_header(&self) -> String {
        format!("{}={}", self.name, self.value)
    }
}

/// Cookie jar for managing multiple cookies
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CookieJar {
    cookies: HashMap<String, Cookie>,
}

impl CookieJar {
    /// Create a new cookie jar
    pub fn new() -> Self {
        Self {
            cookies: HashMap::new(),
        }
    }

    /// Add a cookie
    pub fn add(&mut self, cookie: Cookie) {
        self.cookies.insert(cookie.name.clone(), cookie);
    }

    /// Get a cookie by name
    pub fn get(&self, name: &str) -> Option<&Cookie> {
        self.cookies.get(name)
    }

    /// Remove a cookie
    pub fn remove(&mut self, name: &str) -> Option<Cookie> {
        self.cookies.remove(name)
    }

    /// Remove expired cookies
    pub fn remove_expired(&mut self) {
        self.cookies.retain(|_, cookie| !cookie.is_expired());
    }

    /// Get all cookies
    pub fn all(&self) -> Vec<&Cookie> {
        self.cookies.values().collect()
    }

    /// Get cookies for a specific domain
    pub fn for_domain(&self, domain: &str) -> Vec<&Cookie> {
        self.cookies
            .values()
            .filter(|c| {
                c.domain
                    .as_ref()
                    .map(|d| domain.ends_with(d))
                    .unwrap_or(true)
            })
            .collect()
    }

    /// Get Cookie header value for request
    pub fn cookie_header(&self, domain: &str) -> Option<String> {
        let cookies = self.for_domain(domain);
        if cookies.is_empty() {
            None
        } else {
            Some(
                cookies
                    .iter()
                    .map(|c| c.to_header())
                    .collect::<Vec<_>>()
                    .join("; "),
            )
        }
    }

    /// Parse Set-Cookie headers
    pub fn add_from_headers(&mut self, headers: &[(String, String)]) {
        for (name, value) in headers {
            if name.eq_ignore_ascii_case("set-cookie") {
                if let Some(cookie) = Cookie::from_header(value) {
                    self.add(cookie);
                }
            }
        }
    }

    /// Clear all cookies
    pub fn clear(&mut self) {
        self.cookies.clear();
    }

    /// Count cookies
    pub fn count(&self) -> usize {
        self.cookies.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cookie_creation() {
        let cookie = Cookie::new("session".to_string(), "abc123".to_string());
        assert_eq!(cookie.name, "session");
        assert_eq!(cookie.value, "abc123");
        assert!(!cookie.is_expired());
    }

    #[test]
    fn test_cookie_with_domain() {
        let cookie = Cookie::new("test".to_string(), "value".to_string())
            .with_domain("example.com".to_string())
            .with_path("/".to_string());

        assert_eq!(cookie.domain, Some("example.com".to_string()));
        assert_eq!(cookie.path, Some("/".to_string()));
    }

    #[test]
    fn test_cookie_from_header() {
        let header = "session=abc123; Domain=example.com; Path=/; HttpOnly; Secure";
        let cookie = Cookie::from_header(header).unwrap();

        assert_eq!(cookie.name, "session");
        assert_eq!(cookie.value, "abc123");
        assert_eq!(cookie.domain, Some("example.com".to_string()));
        assert!(cookie.http_only);
        assert!(cookie.secure);
    }

    #[test]
    fn test_cookie_to_header() {
        let cookie = Cookie::new("session".to_string(), "abc123".to_string());
        assert_eq!(cookie.to_header(), "session=abc123");
    }

    #[test]
    fn test_cookie_jar() {
        let mut jar = CookieJar::new();

        jar.add(Cookie::new("session".to_string(), "abc123".to_string()));
        jar.add(Cookie::new("user".to_string(), "john".to_string()));

        assert_eq!(jar.count(), 2);
        assert!(jar.get("session").is_some());
    }

    #[test]
    fn test_cookie_jar_for_domain() {
        let mut jar = CookieJar::new();

        jar.add(
            Cookie::new("cookie1".to_string(), "value1".to_string())
                .with_domain("example.com".to_string()),
        );
        jar.add(
            Cookie::new("cookie2".to_string(), "value2".to_string())
                .with_domain("other.com".to_string()),
        );

        let cookies = jar.for_domain("example.com");
        assert_eq!(cookies.len(), 1);
        assert_eq!(cookies[0].name, "cookie1");
    }

    #[test]
    fn test_cookie_header() {
        let mut jar = CookieJar::new();

        jar.add(Cookie::new("session".to_string(), "abc123".to_string()));
        jar.add(Cookie::new("user".to_string(), "john".to_string()));

        let header = jar.cookie_header("example.com").unwrap();
        assert!(header.contains("session=abc123"));
        assert!(header.contains("user=john"));
    }

    #[test]
    fn test_remove_cookie() {
        let mut jar = CookieJar::new();
        jar.add(Cookie::new("test".to_string(), "value".to_string()));

        assert_eq!(jar.count(), 1);
        jar.remove("test");
        assert_eq!(jar.count(), 0);
    }
}
