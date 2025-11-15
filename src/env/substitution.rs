//! Variable substitution engine for {{VAR}} syntax

use regex::Regex;
use std::collections::HashMap;

/// Variable substitution engine
pub struct VariableSubstitutor {
    pattern: Regex,
}

impl VariableSubstitutor {
    /// Create a new substitution engine
    pub fn new() -> Self {
        // Matches {{VARIABLE_NAME}} pattern
        let pattern = Regex::new(r"\{\{([A-Za-z_][A-Za-z0-9_]*)\}\}").unwrap();
        Self { pattern }
    }

    /// Substitute variables in a string
    pub fn substitute(&self, text: &str, variables: &HashMap<&str, &str>) -> String {
        let result = self.pattern.replace_all(text, |caps: &regex::Captures| {
            let var_name = &caps[1];
            match variables.get(var_name) {
                Some(value) => value.to_string(),
                None => caps.get(0).unwrap().as_str().to_string(),
            }
        });
        result.to_string()
    }

    /// Substitute variables with default fallback
    pub fn substitute_with_default(
        &self,
        text: &str,
        variables: &HashMap<&str, &str>,
        default: &str,
    ) -> String {
        let default_str = default.to_string();
        let result = self.pattern.replace_all(text, |caps: &regex::Captures| {
            let var_name = &caps[1];
            match variables.get(var_name) {
                Some(value) => value.to_string(),
                None => default_str.clone(),
            }
        });
        result.to_string()
    }

    /// Find all variable references in a string
    pub fn find_variables(&self, text: &str) -> Vec<String> {
        self.pattern
            .captures_iter(text)
            .map(|cap| cap[1].to_string())
            .collect()
    }

    /// Check if a string contains variable references
    pub fn has_variables(&self, text: &str) -> bool {
        self.pattern.is_match(text)
    }

    /// Substitute multiple strings at once
    pub fn substitute_multiple(
        &self,
        texts: &[&str],
        variables: &HashMap<&str, &str>,
    ) -> Vec<String> {
        texts
            .iter()
            .map(|text| self.substitute(text, variables))
            .collect()
    }

    /// Validate that all variables in text can be resolved
    pub fn validate(&self, text: &str, variables: &HashMap<&str, &str>) -> Result<(), Vec<String>> {
        let mut missing = Vec::new();

        for cap in self.pattern.captures_iter(text) {
            let var_name = &cap[1];
            if !variables.contains_key(var_name) {
                missing.push(var_name.to_string());
            }
        }

        if missing.is_empty() {
            Ok(())
        } else {
            Err(missing)
        }
    }
}

impl Default for VariableSubstitutor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_substitution() {
        let sub = VariableSubstitutor::new();
        let mut vars = HashMap::new();
        vars.insert("API_URL", "https://api.example.com");
        vars.insert("VERSION", "v1");

        let result = sub.substitute("{{API_URL}}/{{VERSION}}/users", &vars);
        assert_eq!(result, "https://api.example.com/v1/users");
    }

    #[test]
    fn test_substitution_with_missing_var() {
        let sub = VariableSubstitutor::new();
        let mut vars = HashMap::new();
        vars.insert("API_URL", "https://api.example.com");

        let result = sub.substitute("{{API_URL}}/{{MISSING}}/users", &vars);
        assert_eq!(result, "https://api.example.com/{{MISSING}}/users");
    }

    #[test]
    fn test_substitution_with_default() {
        let sub = VariableSubstitutor::new();
        let mut vars = HashMap::new();
        vars.insert("API_URL", "https://api.example.com");

        let result =
            sub.substitute_with_default("{{API_URL}}/{{MISSING}}/users", &vars, "unknown");
        assert_eq!(result, "https://api.example.com/unknown/users");
    }

    #[test]
    fn test_find_variables() {
        let sub = VariableSubstitutor::new();
        let text = "{{API_URL}}/{{VERSION}}/users/{{USER_ID}}";

        let vars = sub.find_variables(text);
        assert_eq!(vars.len(), 3);
        assert!(vars.contains(&"API_URL".to_string()));
        assert!(vars.contains(&"VERSION".to_string()));
        assert!(vars.contains(&"USER_ID".to_string()));
    }

    #[test]
    fn test_has_variables() {
        let sub = VariableSubstitutor::new();

        assert!(sub.has_variables("{{API_URL}}/users"));
        assert!(!sub.has_variables("https://api.example.com/users"));
    }

    #[test]
    fn test_validate_success() {
        let sub = VariableSubstitutor::new();
        let mut vars = HashMap::new();
        vars.insert("API_URL", "https://api.example.com");
        vars.insert("VERSION", "v1");

        let result = sub.validate("{{API_URL}}/{{VERSION}}/users", &vars);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_failure() {
        let sub = VariableSubstitutor::new();
        let mut vars = HashMap::new();
        vars.insert("API_URL", "https://api.example.com");

        let result = sub.validate("{{API_URL}}/{{VERSION}}/{{USER_ID}}", &vars);
        assert!(result.is_err());

        let missing = result.unwrap_err();
        assert_eq!(missing.len(), 2);
        assert!(missing.contains(&"VERSION".to_string()));
        assert!(missing.contains(&"USER_ID".to_string()));
    }

    #[test]
    fn test_multiple_substitution() {
        let sub = VariableSubstitutor::new();
        let mut vars = HashMap::new();
        vars.insert("API_URL", "https://api.example.com");
        vars.insert("VERSION", "v1");

        let texts = vec!["{{API_URL}}/users", "{{API_URL}}/{{VERSION}}/posts"];

        let results = sub.substitute_multiple(&texts, &vars);
        assert_eq!(results[0], "https://api.example.com/users");
        assert_eq!(results[1], "https://api.example.com/v1/posts");
    }

    #[test]
    fn test_no_variables() {
        let sub = VariableSubstitutor::new();
        let vars = HashMap::new();

        let result = sub.substitute("https://api.example.com/users", &vars);
        assert_eq!(result, "https://api.example.com/users");
    }

    #[test]
    fn test_variable_name_rules() {
        let sub = VariableSubstitutor::new();
        let mut vars = HashMap::new();
        vars.insert("API_URL_V2", "https://api.example.com");
        vars.insert("_private", "secret");

        let result = sub.substitute("{{API_URL_V2}}/{{_private}}", &vars);
        assert_eq!(result, "https://api.example.com/secret");
    }

    #[test]
    fn test_complex_text() {
        let sub = VariableSubstitutor::new();
        let mut vars = HashMap::new();
        vars.insert("BASE", "https://api.example.com");
        vars.insert("KEY", "abc123");

        let text = r#"{"url":"{{BASE}}/auth","key":"{{KEY}}"}"#;
        let result = sub.substitute(text, &vars);
        assert_eq!(result, r#"{"url":"https://api.example.com/auth","key":"abc123"}"#);
    }
}
