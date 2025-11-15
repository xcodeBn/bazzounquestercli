//! Matchers for assertion validation

use serde::{Deserialize, Serialize};

/// Type of matcher
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MatcherType {
    /// Equals (exact match)
    Equals,

    /// Not equals
    NotEquals,

    /// Contains substring
    Contains,

    /// Does not contain substring
    NotContains,

    /// Starts with
    StartsWith,

    /// Ends with
    EndsWith,

    /// Matches regex
    Regex,

    /// Less than (numeric)
    LessThan,

    /// Less than or equal
    LessThanOrEqual,

    /// Greater than (numeric)
    GreaterThan,

    /// Greater than or equal
    GreaterThanOrEqual,

    /// Is empty
    IsEmpty,

    /// Is not empty
    IsNotEmpty,

    /// Has length
    HasLength,

    /// Is null/None
    IsNull,

    /// Is not null
    IsNotNull,
}

/// A matcher for validating values
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Matcher {
    /// Type of matcher
    pub matcher_type: MatcherType,

    /// Expected value (stringified)
    pub expected: String,
}

impl Matcher {
    /// Create a new matcher
    pub fn new(matcher_type: MatcherType, expected: String) -> Self {
        Self {
            matcher_type,
            expected,
        }
    }

    /// Equals matcher (numeric)
    pub fn equals(value: i64) -> Self {
        Self::new(MatcherType::Equals, value.to_string())
    }

    /// Equals matcher (string)
    pub fn equals_str(value: &str) -> Self {
        Self::new(MatcherType::Equals, value.to_string())
    }

    /// Not equals matcher
    pub fn not_equals(value: i64) -> Self {
        Self::new(MatcherType::NotEquals, value.to_string())
    }

    /// Not equals matcher (string)
    pub fn not_equals_str(value: &str) -> Self {
        Self::new(MatcherType::NotEquals, value.to_string())
    }

    /// Contains matcher
    pub fn contains(substring: String) -> Self {
        Self::new(MatcherType::Contains, substring)
    }

    /// Not contains matcher
    pub fn not_contains(substring: String) -> Self {
        Self::new(MatcherType::NotContains, substring)
    }

    /// Starts with matcher
    pub fn starts_with(prefix: String) -> Self {
        Self::new(MatcherType::StartsWith, prefix)
    }

    /// Ends with matcher
    pub fn ends_with(suffix: String) -> Self {
        Self::new(MatcherType::EndsWith, suffix)
    }

    /// Regex matcher
    pub fn regex(pattern: String) -> Self {
        Self::new(MatcherType::Regex, pattern)
    }

    /// Less than matcher
    pub fn less_than(value: i64) -> Self {
        Self::new(MatcherType::LessThan, value.to_string())
    }

    /// Less than or equal matcher
    pub fn less_than_or_equal(value: i64) -> Self {
        Self::new(MatcherType::LessThanOrEqual, value.to_string())
    }

    /// Greater than matcher
    pub fn greater_than(value: i64) -> Self {
        Self::new(MatcherType::GreaterThan, value.to_string())
    }

    /// Greater than or equal matcher
    pub fn greater_than_or_equal(value: i64) -> Self {
        Self::new(MatcherType::GreaterThanOrEqual, value.to_string())
    }

    /// Is empty matcher
    pub fn is_empty() -> Self {
        Self::new(MatcherType::IsEmpty, String::new())
    }

    /// Is not empty matcher
    pub fn is_not_empty() -> Self {
        Self::new(MatcherType::IsNotEmpty, String::new())
    }

    /// Has length matcher
    pub fn has_length(length: usize) -> Self {
        Self::new(MatcherType::HasLength, length.to_string())
    }

    /// Is null matcher
    pub fn is_null() -> Self {
        Self::new(MatcherType::IsNull, String::new())
    }

    /// Is not null matcher
    pub fn is_not_null() -> Self {
        Self::new(MatcherType::IsNotNull, String::new())
    }

    /// Test if actual value matches expected
    pub fn matches(&self, actual: &str) -> bool {
        match self.matcher_type {
            MatcherType::Equals => actual == self.expected,
            MatcherType::NotEquals => actual != self.expected,
            MatcherType::Contains => actual.contains(&self.expected),
            MatcherType::NotContains => !actual.contains(&self.expected),
            MatcherType::StartsWith => actual.starts_with(&self.expected),
            MatcherType::EndsWith => actual.ends_with(&self.expected),
            MatcherType::Regex => {
                if let Ok(re) = regex::Regex::new(&self.expected) {
                    re.is_match(actual)
                } else {
                    false
                }
            }
            MatcherType::LessThan => {
                if let (Ok(a), Ok(e)) = (actual.parse::<i64>(), self.expected.parse::<i64>()) {
                    a < e
                } else {
                    false
                }
            }
            MatcherType::LessThanOrEqual => {
                if let (Ok(a), Ok(e)) = (actual.parse::<i64>(), self.expected.parse::<i64>()) {
                    a <= e
                } else {
                    false
                }
            }
            MatcherType::GreaterThan => {
                if let (Ok(a), Ok(e)) = (actual.parse::<i64>(), self.expected.parse::<i64>()) {
                    a > e
                } else {
                    false
                }
            }
            MatcherType::GreaterThanOrEqual => {
                if let (Ok(a), Ok(e)) = (actual.parse::<i64>(), self.expected.parse::<i64>()) {
                    a >= e
                } else {
                    false
                }
            }
            MatcherType::IsEmpty => actual.is_empty(),
            MatcherType::IsNotEmpty => !actual.is_empty(),
            MatcherType::HasLength => {
                if let Ok(expected_len) = self.expected.parse::<usize>() {
                    actual.len() == expected_len
                } else {
                    false
                }
            }
            MatcherType::IsNull => actual.is_empty() || actual == "null",
            MatcherType::IsNotNull => !actual.is_empty() && actual != "null",
        }
    }

    /// Get description of what this matcher expects
    pub fn description(&self) -> String {
        match self.matcher_type {
            MatcherType::Equals => format!("equals '{}'", self.expected),
            MatcherType::NotEquals => format!("not equals '{}'", self.expected),
            MatcherType::Contains => format!("contains '{}'", self.expected),
            MatcherType::NotContains => format!("does not contain '{}'", self.expected),
            MatcherType::StartsWith => format!("starts with '{}'", self.expected),
            MatcherType::EndsWith => format!("ends with '{}'", self.expected),
            MatcherType::Regex => format!("matches regex '{}'", self.expected),
            MatcherType::LessThan => format!("< {}", self.expected),
            MatcherType::LessThanOrEqual => format!("<= {}", self.expected),
            MatcherType::GreaterThan => format!("> {}", self.expected),
            MatcherType::GreaterThanOrEqual => format!(">= {}", self.expected),
            MatcherType::IsEmpty => "is empty".to_string(),
            MatcherType::IsNotEmpty => "is not empty".to_string(),
            MatcherType::HasLength => format!("has length {}", self.expected),
            MatcherType::IsNull => "is null".to_string(),
            MatcherType::IsNotNull => "is not null".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matcher_equals() {
        let matcher = Matcher::equals(200);
        assert!(matcher.matches("200"));
        assert!(!matcher.matches("404"));
    }

    #[test]
    fn test_matcher_equals_str() {
        let matcher = Matcher::equals_str("success");
        assert!(matcher.matches("success"));
        assert!(!matcher.matches("failure"));
    }

    #[test]
    fn test_matcher_not_equals() {
        let matcher = Matcher::not_equals(404);
        assert!(matcher.matches("200"));
        assert!(!matcher.matches("404"));
    }

    #[test]
    fn test_matcher_contains() {
        let matcher = Matcher::contains("json".to_string());
        assert!(matcher.matches("application/json"));
        assert!(!matcher.matches("text/html"));
    }

    #[test]
    fn test_matcher_not_contains() {
        let matcher = Matcher::not_contains("error".to_string());
        assert!(matcher.matches("success"));
        assert!(!matcher.matches("error occurred"));
    }

    #[test]
    fn test_matcher_starts_with() {
        let matcher = Matcher::starts_with("http".to_string());
        assert!(matcher.matches("https://example.com"));
        assert!(!matcher.matches("ftp://example.com"));
    }

    #[test]
    fn test_matcher_ends_with() {
        let matcher = Matcher::ends_with(".json".to_string());
        assert!(matcher.matches("data.json"));
        assert!(!matcher.matches("data.xml"));
    }

    #[test]
    fn test_matcher_regex() {
        let matcher = Matcher::regex(r"^\d{3}$".to_string());
        assert!(matcher.matches("200"));
        assert!(matcher.matches("404"));
        assert!(!matcher.matches("20"));
        assert!(!matcher.matches("2000"));
    }

    #[test]
    fn test_matcher_less_than() {
        let matcher = Matcher::less_than(1000);
        assert!(matcher.matches("500"));
        assert!(!matcher.matches("1500"));
        assert!(!matcher.matches("1000"));
    }

    #[test]
    fn test_matcher_less_than_or_equal() {
        let matcher = Matcher::less_than_or_equal(1000);
        assert!(matcher.matches("500"));
        assert!(matcher.matches("1000"));
        assert!(!matcher.matches("1500"));
    }

    #[test]
    fn test_matcher_greater_than() {
        let matcher = Matcher::greater_than(100);
        assert!(matcher.matches("200"));
        assert!(!matcher.matches("50"));
        assert!(!matcher.matches("100"));
    }

    #[test]
    fn test_matcher_greater_than_or_equal() {
        let matcher = Matcher::greater_than_or_equal(100);
        assert!(matcher.matches("200"));
        assert!(matcher.matches("100"));
        assert!(!matcher.matches("50"));
    }

    #[test]
    fn test_matcher_is_empty() {
        let matcher = Matcher::is_empty();
        assert!(matcher.matches(""));
        assert!(!matcher.matches("content"));
    }

    #[test]
    fn test_matcher_is_not_empty() {
        let matcher = Matcher::is_not_empty();
        assert!(matcher.matches("content"));
        assert!(!matcher.matches(""));
    }

    #[test]
    fn test_matcher_has_length() {
        let matcher = Matcher::has_length(5);
        assert!(matcher.matches("hello"));
        assert!(!matcher.matches("hi"));
    }

    #[test]
    fn test_matcher_is_null() {
        let matcher = Matcher::is_null();
        assert!(matcher.matches(""));
        assert!(matcher.matches("null"));
        assert!(!matcher.matches("value"));
    }

    #[test]
    fn test_matcher_is_not_null() {
        let matcher = Matcher::is_not_null();
        assert!(matcher.matches("value"));
        assert!(!matcher.matches(""));
        assert!(!matcher.matches("null"));
    }

    #[test]
    fn test_matcher_description() {
        assert_eq!(Matcher::equals(200).description(), "equals '200'");
        assert_eq!(
            Matcher::contains("test".to_string()).description(),
            "contains 'test'"
        );
        assert_eq!(Matcher::less_than(100).description(), "< 100");
        assert_eq!(Matcher::is_empty().description(), "is empty");
    }

    #[test]
    fn test_matcher_serialization() {
        let matcher = Matcher::equals(200);
        let json = serde_json::to_string(&matcher).unwrap();
        let deserialized: Matcher = serde_json::from_str(&json).unwrap();

        assert_eq!(matcher.matcher_type, deserialized.matcher_type);
        assert_eq!(matcher.expected, deserialized.expected);
    }
}
